use git2::Repository;
use std::path::Path;

fn main() {
    Repository::clone(
        "https://github.com/googleapis/googleapis.git",
        Path::new("googleapis"),
    )
    .expect("failed to clone");

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
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/grafeas/v1",
        includes: &["googleapis", "protoc-3.11.4-linux-x86_64/include"],
        input: &[
            "googleapis/grafeas/v1/attestation.proto",
            "googleapis/grafeas/v1/build.proto",
            "googleapis/grafeas/v1/common.proto",
            "googleapis/grafeas/v1/cvss.proto",
            "googleapis/grafeas/v1/deployment.proto",
            "googleapis/grafeas/v1/discovery.proto",
            "googleapis/grafeas/v1/grafeas.proto",
            "googleapis/grafeas/v1/image.proto",
            "googleapis/grafeas/v1/package.proto",
            "googleapis/grafeas/v1/provenance.proto",
            "googleapis/grafeas/v1/upgrade.proto",
            "googleapis/grafeas/v1/vulnerability.proto",
        ],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc");
}
