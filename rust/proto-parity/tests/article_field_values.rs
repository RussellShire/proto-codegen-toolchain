use prost::Message;
use proto_parity::article::v1::{Article, ArticlePatch, ArticleStatus};
use proto_parity::common::v1::{
    field_value, primitive_value, FieldSet, FieldValue, PrimitiveValue,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn load_fixture() -> Value {
    let json = fs::read_to_string("../../fixtures/articles/article-field-values.json")
        .expect("fixture should be readable");

    serde_json::from_str(&json).expect("fixture should be valid json")
}

fn field_value_from_fixture(input: &Value) -> FieldValue {
    let state = &input["state"];

     match state["case"].as_str().expect("field state case should exist") {
         "unset" => FieldValue {
            state: Some(field_value::State::Unset(())),
         },
         "null" => FieldValue {
            state: Some(field_value::State::Null(0)),
         },
         "value" => FieldValue {
             state: Some(field_value::State::Value(primitive_value_from_fixture(
                 &state["value"],
             ))),
         },
        other => panic!("unsupported field state case: {other:?}"),
    }
}

fn primitive_value_from_fixture(input: &Value) -> PrimitiveValue {
    let kind = &input["kind"];

    match kind["case"]
        .as_str()
        .expect("primitive kind case should exist")
    {
        "string_value" => PrimitiveValue {
            kind: Some(primitive_value::Kind::StringValue(
                kind["stringValue"]
                    .as_str()
                    .expect("string value should exist")
                    .to_owned(),
            )),
        },
        "bool_value" => PrimitiveValue {
            kind: Some(primitive_value::Kind::BoolValue(
                kind["boolValue"]
                    .as_bool()
                    .expect("bool value should exist"),
            )),
        },
        "int32_value" => PrimitiveValue {
            kind: Some(primitive_value::Kind::Int32Value(
                kind["int32Value"]
                    .as_i64()
                    .expect("int32 value should exist") as i32,
            )),
        },
        other => panic!("unsupported primitive kind case: {other}"),
    }
}

#[test]
fn preserves_null_false_zero_and_empty_string_semantics() {
    let fixture = load_fixture();
    let article_fixture = &fixture["article"];

    let article = Article {
        id: article_fixture["id"]
            .as_str()
            .expect("article id should exist")
            .to_owned(),
        status: ArticleStatus::Draft as i32,
        tags: article_fixture["tags"]
            .as_array()
            .expect("tags should exist")
            .iter()
            .map(|tag| tag.as_str().expect("tag should be a string").to_owned())
            .collect(),
        title: Some(field_value_from_fixture(&article_fixture["title"])),
        subtitle: Some(field_value_from_fixture(&article_fixture["subtitle"])),
        is_featured: Some(field_value_from_fixture(&article_fixture["isFeatured"])),
        priority: Some(field_value_from_fixture(&article_fixture["priority"])),
    };

    assert!(matches!(
        article.subtitle.as_ref().and_then(|field| field.state.as_ref()),
        Some(field_value::State::Null(0))
    ));

    let is_featured = article
        .is_featured
        .as_ref()
        .and_then(|field| field.state.as_ref());

    let is_featured_value = match is_featured {
        Some(field_value::State::Value(value)) => value,
        other => panic!("expected is_featured value, got {other:?}"),
    };

    assert_eq!(
        is_featured_value.kind,
        Some(primitive_value::Kind::BoolValue(false))
    );

    let priority = article
        .priority
        .as_ref()
        .and_then(|field| field.state.as_ref());

    let priority_value = match priority {
        Some(field_value::State::Value(value)) => value,
        other => panic!("expected priority value, got {other:?}"),
    };

    assert_eq!(
        priority_value.kind,
        Some(primitive_value::Kind::Int32Value(0))
    );

    let patch_fixture = &fixture["patch"];
    let patch_fields = patch_fixture["fields"]
        .as_object()
        .expect("patch fields should exist")
        .iter()
        .map(|(key, value)| (key.to_owned(), field_value_from_fixture(value)))
        .collect::<HashMap<_, _>>();

    let patch = ArticlePatch {
        id: patch_fixture["id"]
            .as_str()
            .expect("patch id should exist")
            .to_owned(),
        fields: Some(FieldSet {
            fields: patch_fields,
        }),
    };

    assert!(matches!(
        patch
            .fields
            .as_ref()
            .and_then(|field_set| field_set.fields.get("subtitle"))
            .and_then(|field| field.state.as_ref()),
        Some(field_value::State::Unset(_))
    ));

    let patch_title = patch
        .fields
        .as_ref()
        .and_then(|field_set| field_set.fields.get("title"))
        .and_then(|field| field.state.as_ref());

    let patch_title_value = match patch_title {
        Some(field_value::State::Value(value)) => value,
        other => panic!("expected patch title value, got {other:?}"),
    };

    assert_eq!(
        patch_title_value.kind,
        Some(primitive_value::Kind::StringValue(String::new()))
    );
}

#[test]
fn round_trips_through_protobuf_binary_encoding() {
    let fixture = load_fixture();
    let article_fixture = &fixture["article"];

    let article = Article {
        id: article_fixture["id"]
            .as_str()
            .expect("article id should exist")
            .to_owned(),
        status: ArticleStatus::Draft as i32,
        tags: article_fixture["tags"]
            .as_array()
            .expect("tags should exist")
            .iter()
            .map(|tag| tag.as_str().expect("tag should be a string").to_owned())
            .collect(),
        title: Some(field_value_from_fixture(&article_fixture["title"])),
        subtitle: Some(field_value_from_fixture(&article_fixture["subtitle"])),
        is_featured: Some(field_value_from_fixture(&article_fixture["isFeatured"])),
        priority: Some(field_value_from_fixture(&article_fixture["priority"])),
    };

    let encoded = article.encode_to_vec();
    let decoded = Article::decode(encoded.as_slice()).expect("article should decode");

    assert_eq!(decoded, article);
}
