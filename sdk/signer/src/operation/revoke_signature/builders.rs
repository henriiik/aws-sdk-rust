// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_signature::_revoke_signature_output::RevokeSignatureOutputBuilder;

pub use crate::operation::revoke_signature::_revoke_signature_input::RevokeSignatureInputBuilder;

/// Fluent builder constructing a request to `RevokeSignature`.
///
/// <p>Changes the state of a signing job to REVOKED. This indicates that the signature is no longer valid.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RevokeSignatureFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::revoke_signature::builders::RevokeSignatureInputBuilder,
}
impl RevokeSignatureFluentBuilder {
    /// Creates a new `RevokeSignature`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::revoke_signature::RevokeSignature,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::revoke_signature::RevokeSignatureError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::revoke_signature::RevokeSignatureOutput,
        aws_smithy_http::result::SdkError<crate::operation::revoke_signature::RevokeSignatureError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>ID of the signing job to be revoked.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>ID of the signing job to be revoked.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>AWS account ID of the job owner.</p>
    pub fn job_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_owner(input.into());
        self
    }
    /// <p>AWS account ID of the job owner.</p>
    pub fn set_job_owner(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_owner(input);
        self
    }
    /// <p>The reason for revoking the signing job.</p>
    pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.reason(input.into());
        self
    }
    /// <p>The reason for revoking the signing job.</p>
    pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_reason(input);
        self
    }
}
