import { describe, expect, test } from "vitest";
import { readFileSync } from "node:fs";
import { Article, ArticlePatch, ArticleStatus } from "../src/generated/article/v1/article.js";
import {
    FieldValue,
    PrimitiveValue,
} from "../src/generated/common/v1/field_value.js";

type Fixture = {
    article: {
        id: string;
        status: keyof typeof ArticleStatus;
        tags: string[];
        title?: FixtureFieldValue;
        subtitle?: FixtureFieldValue;
        isFeatured?: FixtureFieldValue;
        priority?: FixtureFieldValue;
    };
    patch: {
        id: string;
        fields: Record<string, FixtureFieldValue>;
    };
};

type FixtureFieldValue = {
    state:
        | { case: "unset" }
        | { case: "null" }
        | { case: "value"; value: FixturePrimitiveValue };
};

type FixturePrimitiveValue = {
    kind:
        | { case: "string_value"; stringValue: string }
        | { case: "bool_value"; boolValue: boolean }
        | { case: "int32_value"; int32Value: number };
};

function fieldValueFromFixture(input: FixtureFieldValue): FieldValue {
    switch (input.state.case) {
        case "unset":
            return { state: { $case: "unset", unset: {} } };
        case "null":
            return { state: { $case: "null", null: 0 } };
        case "value":
            return {
                state: {
                    $case: "value",
                    value: primitiveValueFromFixture(input.state.value),
                },
            };
    }
}

function primitiveValueFromFixture(input: FixturePrimitiveValue): PrimitiveValue {
    switch (input.kind.case) {
        case "string_value":
            return {
                kind: {
                    $case: "stringValue",
                    stringValue: input.kind.stringValue,
                },
            };
        case "bool_value":
            return {
                kind: {
                    $case: "boolValue",
                    boolValue: input.kind.boolValue,
                },
            };
        case "int32_value":
            return {
                kind: {
                    $case: "int32Value",
                    int32Value: input.kind.int32Value,
                },
            };
    }
}

function loadFixture(): Fixture {
    return JSON.parse(
        readFileSync("fixtures/articles/article-field-values.json", "utf8"),
    ) as Fixture;
}

describe("article field value parity", () => {
    test("preserves null, false, zero, and empty string semantics", () => {
        const fixture = loadFixture();

        const article: Article = {
            id: fixture.article.id,
            status: ArticleStatus[fixture.article.status],
            tags: fixture.article.tags,
            title: fieldValueFromFixture(fixture.article.title!),
            subtitle: fieldValueFromFixture(fixture.article.subtitle!),
            isFeatured: fieldValueFromFixture(fixture.article.isFeatured!),
            priority: fieldValueFromFixture(fixture.article.priority!),
        };

        expect(article.subtitle?.state?.$case).toBe("null");
        expect(article.isFeatured?.state?.$case).toBe("value");

        const isFeaturedValue = article.isFeatured?.state?.$case === "value"
            ? article.isFeatured.state.value
            : undefined;

        expect(isFeaturedValue?.kind?.$case).toBe("boolValue");
        expect(
            isFeaturedValue?.kind?.$case === "boolValue"
                ? isFeaturedValue.kind.boolValue
                : undefined,
        ).toBe(false);

        const priorityValue = article.priority?.state?.$case === "value"
            ? article.priority.state.value
            : undefined;

        expect(priorityValue?.kind?.$case).toBe("int32Value");
        expect(
            priorityValue?.kind?.$case === "int32Value"
                ? priorityValue.kind.int32Value
                : undefined,
        ).toBe(0);

        const patch: ArticlePatch = {
            id: fixture.patch.id,
            fields: {
                fields: Object.fromEntries(
                    Object.entries(fixture.patch.fields).map(([key, value]) => [
                        key,
                        fieldValueFromFixture(value),
                    ]),
                ),
            },
        };

        expect(patch.fields?.fields.subtitle?.state?.$case).toBe("unset");

        const patchTitle = patch.fields?.fields.title;
        const patchTitleValue = patchTitle?.state?.$case === "value"
            ? patchTitle.state.value
            : undefined;

        expect(patchTitleValue?.kind?.$case).toBe("stringValue");
        expect(
            patchTitleValue?.kind?.$case === "stringValue"
                ? patchTitleValue.kind.stringValue
                : undefined,
        ).toBe("");
    });

    test("round-trips through protobuf binary encoding", () => {
        const fixture = loadFixture();

        const article: Article = {
            id: fixture.article.id,
            status: ArticleStatus[fixture.article.status],
            tags: fixture.article.tags,
            title: fieldValueFromFixture(fixture.article.title!),
            subtitle: fieldValueFromFixture(fixture.article.subtitle!),
            isFeatured: fieldValueFromFixture(fixture.article.isFeatured!),
            priority: fieldValueFromFixture(fixture.article.priority!),
        };

        const encoded = Article.encode(article).finish();
        const decoded = Article.decode(encoded);

        expect(decoded).toEqual(article);
    });
});
