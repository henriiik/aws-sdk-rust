#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Amazon Web Services Single Sign On OpenID Connect (OIDC) is a web service that enables a client (such as Amazon Web Services CLI
//! or a native application) to register with Amazon Web Services SSO. The service also enables the client to
//! fetch the user’s access token upon successful authentication and authorization with
//! Amazon Web Services SSO.</p>
//! <note>
//! <p>Although Amazon Web Services Single Sign-On was renamed, the <code>sso</code> and
//! <code>identitystore</code> API namespaces will continue to retain their original name for
//! backward compatibility purposes. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/what-is.html#renamed">Amazon Web Services SSO rename</a>.</p>
//! </note>
//! <p>
//! <b>Considerations for Using This Guide</b>
//! </p>
//! <p>Before you begin using this guide, we recommend that you first review the following
//! important information about how the Amazon Web Services SSO OIDC service works.</p>
//! <ul>
//! <li>
//! <p>The Amazon Web Services SSO OIDC service currently implements only the portions of the OAuth 2.0
//! Device Authorization Grant standard (<a href="https://tools.ietf.org/html/rfc8628">https://tools.ietf.org/html/rfc8628</a>) that are necessary to enable single
//! sign-on authentication with the AWS CLI. Support for other OIDC flows frequently needed
//! for native applications, such as Authorization Code Flow (+ PKCE), will be addressed in
//! future releases.</p>
//! </li>
//! <li>
//! <p>The service emits only OIDC access tokens, such that obtaining a new token (For
//! example, token refresh) requires explicit user re-authentication.</p>
//! </li>
//! <li>
//! <p>The access tokens provided by this service grant access to all AWS account
//! entitlements assigned to an Amazon Web Services SSO user, not just a particular application.</p>
//! </li>
//! <li>
//! <p>The documentation in this guide does not describe the mechanism to convert the access
//! token into AWS Auth (“sigv4”) credentials for use with IAM-protected AWS service
//! endpoints. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/PortalAPIReference/API_GetRoleCredentials.html">GetRoleCredentials</a> in the <i>Amazon Web Services SSO Portal API Reference
//! Guide</i>.</p>
//! </li>
//! </ul>
//!
//! <p>For general information about Amazon Web Services SSO, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/what-is.html">What is
//! Amazon Web Services SSO?</a> in the <i>Amazon Web Services SSO User Guide</i>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod middleware;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("ssooidc", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
