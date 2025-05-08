//! Trivial
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_results)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::similar_names)]

/// Automatically derive [Claim] on a struct or enum by recurisvley calling
/// [Claim::claim] on its contents.
pub use trivial_derive::Claim;
/// Automatically derive [Trivial] on a struct or enum by recurisvley calling
/// [Trivial::dup] on its contents.
pub use trivial_derive::Trivial;

pub use self::{
    claim_arc::ClaimArc, claim_rc::ClaimRc, trivial_box::TrivialBox,
    trivial_vec::TrivialVec,
};

mod claim_arc;
mod claim_rc;
mod trivial_box;
mod trivial_vec;

/// Copy but with custom code. Roughly:
/// <https://smallcultfollowing.com/babysteps/blog/2024/06/21/claim-auto-and-otherwise/>
pub trait Claim {
    #[allow(missing_docs)]
    #[must_use]
    fn claim(&self) -> Self;
}

impl<T: Copy> Claim for T {
    fn claim(&self) -> Self {
        *self
    }
}

/// Similar to [Claim] but it allows 'trivial' memory allocations e.g [`Box<T>`] or
/// [`Vec<T>`] where `T: Trivial`
pub trait Trivial {
    #[allow(missing_docs)]
    #[must_use]
    fn dup(&self) -> Self;
}

impl<T: Claim> Trivial for T {
    fn dup(&self) -> Self {
        self.claim()
    }
}
