# Rust SDK for [Google APIs](https://github.com/googleapis/googleapis)

## Summary

[Google APIs](https://github.com/googleapis/googleapis) includes definitive protobufs from Google for its gRPC-based APIs. These protobufs are imported by many other projects particularly `google/api/*` and `google/rpc/*`. This repo provides rust sources for (a subset -- please submit PRs) of these protobufs and structures these using a complementary module hierarchy. For example `google/api/annotations.proto` becomes `google::api::annotations`.

> **NB** I'm using [stepancheg](https://github.com/stepancheg) protobuf and gRPC implementations. There are other protobuf and gRPC implements in Rust. Stepan's solutions work, are easy to understand and are pure Rust.

## Clone|Build

```bash
git clone https://github.com/googleapis/googleapis.git [./googleapis]
```

Update `${PATH}` to include [`protoc`](https://github.com/protocolbuffers/protobuf/releases):

```bash
VERS="3.11.4"
ARCH="linux-x86_64"
PATH=${PATH}:path/to/protoc-${VERS}-${ARCH}/bin
```

> **NB** If you use Visual Studio Code with the (recommended!) [Rust Language Server extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust), it will automatically run `cargo build`. If you run the command concurrently, you will receive errors; if you receive `cargo build` errors, it's likely that the Rust Language Server extension is running the build concurrently.

`cargo build` or Visual Studio Code should generate:

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


## Explanation

Rust has an excellent rust-based build system. This repo uses this build system (`build.rs`) to `git clone` the Google APIs repo and then use `protoc` and a crate [`protoc-rust-grpc`](https://crates.io/crates/protoc-rust-grpc) to generate rust sources from the protobuf files.

> **NB** I'm using [stepancheg](https://github.com/stepancheg) protobuf and gRPC implementations. There are other protobuf and gRPC implements in Rust. Stepan's solutions work, are easy to understand and are pure Rust.

> **NB** You must have a version of `protoc` in the path in order for the build to succeed; the crates use the singular implementation of `protoc` as used by everyone.


The first step in the build is to `git clone` the Google APIs repo. If succesful, this will be placed in `./googleapis`.

The subsequent steps are manifestations of `protoc` represented by rust code. Here's an explanation of the constituents:

|args|value|explanation|
|----|-----|-----------|
|`out_dir`|`src/google/apis`|Where the rust sources will be created|
|`includes`|`googleapis`|Directories from which included protobuf files may be found|
|`input`|`googleapis/google/api/annotations.proto`|Paths to protobuf files NB these replicate the `includes` paths|
|`rust_protobuf`|`true`|We'll default to generate gRPC `service`s if they exist by having this as `true`|

> **NB** Stepan provides a command-line `protoc` plugin [`protoc-gen-rust-grpc'](https://github.com/stepancheg/grpc-rust/tree/master/grpc-compiler) too

`build.rs`:

```rust
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

> **NB** Because this library includes protobuf `message`s only (no gRPC `service`s) we may use `--rust_out`. For gRPC `service`s, we must use `--rust-grpc_out`


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

Revised the above approach to the module declaration.

When the crate is built, by e.g. `git clone`, it won't include the `src/google/**` directories and fails.

By putting `mod.rs` in each subdirectory, these directories will exist on the clone and the build will succeed.

## Extending

Google APIs includes many directories not currently being included by this build.

Here's an overview of extending this repo to include an API.

E.g. Cloud IoT the protobufs are in `googleapis/google/cloud/iot/v1`

> **NB** Cloud services commonly use e.g. `/v1` as a way to version the APIs

### 1. Review Imports

First, review the imports to ensure that these dependencies are already being generated as rust sources; otherwise, while you're be able to generate rust sources for this service, those services won't compile.

### 2. Create Output Directories

This step could be automated.

The Google APIs directory structure of the protobufs must be reflected as a module structure for the rust sources.

For e.g. Cloud IoT v1 (`googleapis/google/cloud/iot/v1`) we will need:

```bash
mkdir -p src/google/cloud/iot/v1
```
yielding:
```
src
├── google
│   ├── ...
│   ├── cloud
│   │   └── iot
│   ├── ...
│   └── mod.rs
├── ...
└── lib.rs
```
And, depending upon our preference, we'll need `mod.rs` files at each level.

We know what to expect, so we may create and revise these files in advance.

### 3. Revise `build.rs`

```rust
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
.expect("protoc");
```

Visual Studio Code will auto-detect changes and rebuild. Otherwise `cargo build`

### 4. Revise Modules

Now that we have sources, we need to revise the `lib.rs` and `mod.rs` definitions to reflect these.

Let's start at the top of the tree and work down:

+ `lib.rs` already includes `google` (and `grafeas`) so this is OK
+ `google/mod.rs` includes `api` and `rpc` but not `cloud` so we need to add `pub mod cloud`
+ we must create `google/cloud/mod.rs` and `google/cloud/iot/mod.rs` and `googl/cloud/iot/v1/mod.rs`

Here's the mod definitions wrapped into a single file but do **not** use this approach. When the repo is cloned, directories are only created if they contain files. If a single `mod.rs` is created as shown before, the subdirectories (e.g. `iot`, `iot/v1`) will not be created. This causes errors in `protoc` compilation (and, from experience, these can be difficult to debug). A more robust approach is to create `mod.rs` files in each expected subdirectory. This forces the creation of these directories and the `protoc` compilation should then succeed.

```rust
pub mod iot {
    pub mod v1 {
        use ::google::iam::v1::{iam_policy, policy};
        use ::google::protobuf::empty;
        use ::google::rpc::status;

        pub mod device_manager;
        pub mod device_manager_grpc;
        pub mod resources;
    }
}
```

> **NB** We must `use` the rust sources that represent the probobuf `import` files


### 5. Done

That should be it.

Unless, as with Cloud IoT, these protos themselves depend upon another Google API. In this case IAM.

## Protobuf [Well-Known Types](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf)

E.g. `super::empty::Empty`

The protobuf crate includes `protobuf::well_known_types` but IIUC these are all mapped into a single module

E.g.
```
mod empty
...

pub mod self::empty::*;
```

I was able to reference `empty::Empty` by `pub use protobuf::well_known_types as empty` but this put *everything* not just `empty` into `empty` which isn't what I want.

The alternative is to add a build step for `protoc-${VERS}-${ARCH}/include` which includes these (e.g. `empty.proto`) files and permits `pub use ::google:protobuf::empty` which feels (!?) cleaner.
