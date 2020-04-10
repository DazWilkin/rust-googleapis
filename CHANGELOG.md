
## v0.0.7

+ Added Google Common Types (`google/type`) **NB** these are distinct from Google Well-Known Types
+ Fixed misspelling of proto name `google/type/quaternion`

## v0.0.6

+ Missed a few `mod.rs` files to ensure the directory hierarchy is created correctly for `protoc`

## v0.0.5

+ Ensure that `src` tree is fully replicated by ensuring that each level has a file (e.g. `mod.rs`)

## v0.0.4

+ Embedded (!?) protoc builds into repo::clone

## v0.0.3

+ Revised `build.rs` to handle existence of cloned directories
+ Added [CHANGELOG](./CHANGELOG.md)
+ Added [LICENSE](./LICENSE)

## v0.0.2

+ Added `git2::Repository::clone`
+ Added `googleapis/cloud/iot` and `googleapis/cloud/iam`
+ Added `googleapis/grafeas`

## v0.0.1

+ Initial commit(s)