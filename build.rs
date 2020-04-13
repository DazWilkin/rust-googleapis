use git2::{ErrorCode, Repository};
use std::path::Path;

pub mod google {
    pub fn api() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/api",
            includes: &["googleapis"],
            input: &[
                "googleapis/google/api/annotations.proto",
                "googleapis/google/api/auth.proto",
                "googleapis/google/api/backend.proto",
                "googleapis/google/api/billing.proto",
                "googleapis/google/api/client.proto",
                "googleapis/google/api/config_change.proto",
                "googleapis/google/api/consumer.proto",
                "googleapis/google/api/context.proto",
                "googleapis/google/api/control.proto",
                "googleapis/google/api/distribution.proto",
                "googleapis/google/api/documentation.proto",
                "googleapis/google/api/endpoint.proto",
                "googleapis/google/api/field_behavior.proto",
                "googleapis/google/api/http.proto",
                "googleapis/google/api/httpbody.proto",
                "googleapis/google/api/label.proto",
                "googleapis/google/api/launch_stage.proto",
                "googleapis/google/api/log.proto",
                "googleapis/google/api/logging.proto",
                "googleapis/google/api/metric.proto",
                "googleapis/google/api/monitored_resource.proto",
                "googleapis/google/api/monitoring.proto",
                "googleapis/google/api/quota.proto",
                "googleapis/google/api/resource.proto",
                "googleapis/google/api/service.proto",
                "googleapis/google/api/source_info.proto",
                "googleapis/google/api/system_parameter.proto",
                "googleapis/google/api/usage.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc")
    }
    pub mod cloud {
        pub fn iot() {
            protoc_rust_grpc::run(protoc_rust_grpc::Args {
                out_dir: "src/google/cloud/iot/v1",
                includes: &["googleapis"],
                input: &[
                    "googleapis/google/cloud/iot/v1/device_manager.proto",
                    "googleapis/google/cloud/iot/v1/resources.proto",
                ],
                rust_protobuf: true,
                ..Default::default()
            })
            .expect("protoc")
        }
    }
    pub fn container() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/container/v1",
            includes: &["googleapis"],
            input: &["googleapis/google/container/v1/cluster_service.proto"],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
    }
    pub fn iam() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/iam/v1",
            includes: &["googleapis"],
            input: &[
                "googleapis/google/iam/v1/iam_policy.proto",
                "googleapis/google/iam/v1/options.proto",
                "googleapis/google/iam/v1/policy.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/iam/admin/v1",
            includes: &["googleapis"],
            input: &["googleapis/google/iam/admin/v1/iam.proto"],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/iam/credentials/v1",
            includes: &["googleapis"],
            input: &[
                "googleapis/google/iam/credentials/v1/common.proto",
                "googleapis/google/iam/credentials/v1/iamcredentials.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
    }
    pub fn protobuf() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/protobuf",
            includes: &["protobuf/src"],
            input: &[
                "protobuf/src/google/protobuf/any.proto",
                "protobuf/src/google/protobuf/api.proto",
                "protobuf/src/google/protobuf/descriptor.proto",
                "protobuf/src/google/protobuf/duration.proto",
                "protobuf/src/google/protobuf/empty.proto",
                "protobuf/src/google/protobuf/field_mask.proto",
                "protobuf/src/google/protobuf/source_context.proto",
                "protobuf/src/google/protobuf/struct.proto",
                "protobuf/src/google/protobuf/timestamp.proto",
                "protobuf/src/google/protobuf/type.proto",
                "protobuf/src/google/protobuf/wrappers.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
    }
    pub fn rpc() {
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
    // Syntax permits using a reserved word (`type`) as a name
    // Prefer to do this as it permits google/rpc/type.proto --> google::rpc::type
    pub fn r#type() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/google/type",
            includes: &["googleapis"],
            input: &[
                "googleapis/google/type/calendar_period.proto",
                "googleapis/google/type/color.proto",
                "googleapis/google/type/date.proto",
                "googleapis/google/type/datetime.proto",
                "googleapis/google/type/dayofweek.proto",
                "googleapis/google/type/expr.proto",
                "googleapis/google/type/fraction.proto",
                "googleapis/google/type/latlng.proto",
                "googleapis/google/type/money.proto",
                "googleapis/google/type/month.proto",
                "googleapis/google/type/postal_address.proto",
                "googleapis/google/type/quaternion.proto",
                "googleapis/google/type/timeofday.proto",
            ],
            rust_protobuf: true,
            ..Default::default()
        })
        .expect("protoc");
    }
}
pub mod grafeas {
    pub fn v1() {
        protoc_rust_grpc::run(protoc_rust_grpc::Args {
            out_dir: "src/grafeas/v1",
            includes: &["googleapis"],
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
}
fn main() {
    let protobuf = "protobuf";
    match Repository::clone(
        "https://github.com/protocolbuffers/protobuf.git",
        Path::new(protobuf),
    ) {
        // If clone succeeds, proceed
        // Should possibly fold the protoc compilation (e.g. `google::protobuf()`) into this step
        // If clone fails, the only case (currently) of interest is that the cloned directory exists
        // This is (probably) not an issue and occurs if the build is rerun after the directory is cloned
        // Proceeding optimistically
        Ok(_) => {
            println!("[{}] cloned", protobuf);
            google::protobuf();
        }
        Err(e) => match e.code() {
            ErrorCode::Exists => println!("[{}] exists", protobuf),
            _ => panic!(
                "[{}] unexpected: {:?} {:?}",
                protobuf,
                e.code(),
                e.message()
            ),
        },
    }

    let googleapis = "googleapis";
    match Repository::clone(
        "https://github.com/googleapis/googleapis.git",
        Path::new(googleapis),
    ) {
        // If clone succeeds, proceed
        // Should possibly fold the protoc compilations (e.g. `google::api()`, `google::rpc()`) into this step
        // If clone fails, the only case (currently) of interest is that the cloned directory exists
        // This is (probably) not an issue and occurs if the build is rerun after the directory is cloned
        // Proceeding optimistically
        Ok(_) => {
            println!("[{}] cloned", googleapis);

            google::api();

            google::cloud::iot();

            google::container();

            google::iam();
            google::rpc();
            google::r#type();

            grafeas::v1();
        }
        Err(e) => match e.code() {
            ErrorCode::Exists => println!("[{}] exists", googleapis),
            _ => panic!(
                "[{}] unexpected: {:?} {:?}",
                googleapis,
                e.code(),
                e.message()
            ),
        },
    }
}
