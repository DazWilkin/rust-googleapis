fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/google/api",
        includes: &["googleapis"],
        input: &[
            "googleapis/google/api/annotations.proto",
            "googleapis/google/api/http.proto",
            "googleapis/google/api/httpbody.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc");
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/google/rpc",
        includes: &["googleapis"],
        input: &[
            "googleapis/google/rpc/code.proto",
            "googleapis/google/rpc/error_details.proto",
            "googleapis/google/rpc/status.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc");
}
