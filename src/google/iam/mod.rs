pub mod admin {
    pub mod v1 {
        use crate::google::iam::v1::{iam_policy, policy};
        use protobuf::well_known_types as empty;
        pub mod iam;
        pub mod iam_grpc;
    }
}
pub mod credentials {
    pub mod v1 {
        pub mod common;
        pub mod iamcredentials;
        pub mod iamcredentials_grpc;
    }
}
pub mod v1 {
    use crate::google::r#type::expr; // syntax permits using a reserved word (`type`) as a name
    pub mod iam_policy;
    pub mod iam_policy_grpc;
    pub mod options;
    pub mod policy;
}
