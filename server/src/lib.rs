#[cfg(feature = "bundle-viperserver")]
mod bundled;
pub mod client;
pub mod error;
mod opts;
pub mod server;
#[cfg(test)]
mod tests;
pub mod verification;

pub use client::{Client, VerificationRequest, VerificationStatus};
pub use error::ViperServerError;
pub use server::ViperServer;
