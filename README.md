# Rust SDK for [Google APIs](https://github.com/googleapis/googleapis)

```bash
git clone https://github.com/googleapis/googleapis.git 
```

Update `${PATH}` to include `protoc`:

```bash
PATH=${PATH}:${PWD}/protoc-3.11.4-linux-x86_64/bin
```

Then `cargo build`

Should generate:

```
tree -L 2 src/google
src/google
├── api
│   ├── annotations.rs
│   ├── httpbody.rs
│   └── http.rs
└── rpc
    ├── code.rs
    ├── error_details.rs
    └── status.rs
```

Explain `build.rs`:

```rust
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
```

> **NB** IIUC I need to duplicate the `protoc_rust_grpc` to ensure the distinct output directories (`google/api` and `google/rpc`) are generated

Replicates, e.g.:

```bash
protoc \
--proto_path=googleapis \
--rust_out=src/google/api \
  googleapis/google/api/annotations.proto \
  googleapis/google/api/http.proto \
  googleapis/google/api/httpbody.proto
```

The root of the protobuf sources is `./googleapis`.

For example `google/api/annotations.proto` is defined to be:

```protobuf
package google.api;

option go_package = "google.golang.org/genproto/googleapis/api/annotations;annotations";
option java_package = "com.google.api";
```

The generated files should be namespaced to `google.api`.

To replicate this in rust, we want a module structure:

```rust
pub mod google {
    pub mod api {
        pub mod annotations;
        pub mod http;
        pub mod httpbody;
    }
    pub mod rpc {
        pub mod code;
        pub mod error_details;
        pub mod status;
    }
}
```

> **NB** There are many other services defined in `googleapis` that are not currently generated.
