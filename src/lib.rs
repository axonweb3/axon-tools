#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        extern crate std;

        pub mod ckb_light_client;
        mod metadata;
        pub use metadata::{CkbRelatedInfoBuilder, MetadataBuilder};
    }
}

mod error;
#[cfg(feature = "hash")]
mod hash;

#[cfg(feature = "proof")]
mod proof;
pub mod types;

pub use error::Error;

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
pub use proof::verify_proof;

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub use hash::keccak_256;

pub mod consts;
