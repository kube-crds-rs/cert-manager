// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f certificaterequests.yml --schema=derived --docs -b --derive=Default --derive=PartialEq --smart-derive-elision
// kopium version: 0.20.1

#[allow(unused_imports)]
mod prelude {
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use kube_derive::CustomResource;
    #[cfg(feature = "schemars")]
    pub use schemars::JsonSchema;
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::BTreeMap;
    #[cfg(feature = "builder")]
    pub use typed_builder::TypedBuilder;
}
use self::prelude::*;

/// Specification of the desired state of the CertificateRequest resource. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(not(feature = "schemars"), kube(schema = "disabled"))]
#[kube(
    group = "cert-manager.io",
    version = "v1",
    kind = "CertificateRequest",
    plural = "certificaterequests"
)]
#[kube(namespaced)]
#[kube(status = "CertificateRequestStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct CertificateRequestSpec {
    /// Requested 'duration' (i.e. lifetime) of the Certificate. Note that the issuer may choose to ignore the requested duration, just like any other requested attribute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub duration: Option<String>,
    /// Extra contains extra attributes of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub extra: Option<BTreeMap<String, String>>,
    /// Groups contains group membership of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub groups: Option<Vec<String>>,
    /// Requested basic constraints isCA value. Note that the issuer may choose to ignore the requested isCA value, just like any other requested attribute.
    ///  NOTE: If the CSR in the `Request` field has a BasicConstraints extension, it must have the same isCA value as specified here.
    ///  If true, this will automatically add the `cert sign` usage to the list of requested `usages`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "isCA")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub is_ca: Option<bool>,
    /// Reference to the issuer responsible for issuing the certificate. If the issuer is namespace-scoped, it must be in the same namespace as the Certificate. If the issuer is cluster-scoped, it can be used from any namespace.
    ///  The `name` field of the reference must always be specified.
    #[serde(rename = "issuerRef")]
    pub issuer_ref: CertificateRequestIssuerRef,
    /// The PEM-encoded X.509 certificate signing request to be submitted to the issuer for signing.
    ///  If the CSR has a BasicConstraints extension, its isCA attribute must match the `isCA` value of this CertificateRequest. If the CSR has a KeyUsage extension, its key usages must match the key usages in the `usages` field of this CertificateRequest. If the CSR has a ExtKeyUsage extension, its extended key usages must match the extended key usages in the `usages` field of this CertificateRequest.
    pub request: String,
    /// UID contains the uid of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub uid: Option<String>,
    /// Requested key usages and extended key usages.
    ///  NOTE: If the CSR in the `Request` field has uses the KeyUsage or ExtKeyUsage extension, these extensions must have the same values as specified here without any additional values.
    ///  If unset, defaults to `digital signature` and `key encipherment`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub usages: Option<Vec<String>>,
    /// Username contains the name of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub username: Option<String>,
}

/// Reference to the issuer responsible for issuing the certificate. If the issuer is namespace-scoped, it must be in the same namespace as the Certificate. If the issuer is cluster-scoped, it can be used from any namespace.
///  The `name` field of the reference must always be specified.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CertificateRequestIssuerRef {
    /// Group of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub group: Option<String>,
    /// Kind of the resource being referred to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub kind: Option<String>,
    /// Name of the resource being referred to.
    pub name: String,
}

/// Status of the CertificateRequest. This is set and managed automatically. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct CertificateRequestStatus {
    /// The PEM encoded X.509 certificate of the signer, also known as the CA (Certificate Authority). This is set on a best-effort basis by different issuers. If not set, the CA is assumed to be unknown/not available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub ca: Option<String>,
    /// The PEM encoded X.509 certificate resulting from the certificate signing request. If not set, the CertificateRequest has either not been completed or has failed. More information on failure can be found by checking the `conditions` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub certificate: Option<String>,
    /// List of status conditions to indicate the status of a CertificateRequest. Known condition types are `Ready`, `InvalidRequest`, `Approved` and `Denied`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub conditions: Option<Vec<Condition>>,
    /// FailureTime stores the time that this CertificateRequest failed. This is used to influence garbage collection and back-off.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "failureTime"
    )]
    #[cfg_attr(feature = "builder", builder(default, setter(strip_option)))]
    pub failure_time: Option<String>,
}
