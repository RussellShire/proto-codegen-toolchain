fn main() {
    prost_build::Config::new()
        .compile_protos(
            &[
                "../../proto/common/v1/field_value.proto",
                "../../proto/article/v1/article.proto",
            ],
            &["../../proto", "/usr/include"],
        )
        .expect("failed to compile protobuf definitions");
}
