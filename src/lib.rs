//! Kubernetes CRDs for cert-manager 1.14.4
//!
//! This library provides automatically generated types for the [cert-manager 1.14.4 definitions]. It is
//! intended to be used with the [Kube-rs] library.
//!
//! [cert-manager 1.14.4 definitions]: https://github.com/cert-manager/cert-manager/releases/download/v1.14.4/cert-manager.crds.yaml
//! [Kube-rs]: https://kube.rs/

pub mod acme;
pub use acme::*;
pub mod certificaterequests;
pub use certificaterequests::*;
pub mod certificates;
pub use certificates::*;
pub mod clusterissuers;
pub use clusterissuers::*;
pub mod issuers;
pub use issuers::*;
