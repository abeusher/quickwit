/// The corresponding Rust struct \[`crate::types::DocUid`\] is defined manually and
/// externally provided during code generation (see `build.rs`).
///
/// Modify at your own risk.
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocUid {
    /// ULID encoded as a sequence of 16 bytes (big-endian u128).
    #[prost(bytes = "vec", tag = "1")]
    pub doc_uid: ::prost::alloc::vec::Vec<u8>,
}
