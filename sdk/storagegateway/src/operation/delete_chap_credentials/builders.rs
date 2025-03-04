// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_chap_credentials::_delete_chap_credentials_output::DeleteChapCredentialsOutputBuilder;

pub use crate::operation::delete_chap_credentials::_delete_chap_credentials_input::DeleteChapCredentialsInputBuilder;

/// Fluent builder constructing a request to `DeleteChapCredentials`.
///
/// <p>Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair. This operation is supported in volume and tape gateway types.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteChapCredentialsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_chap_credentials::builders::DeleteChapCredentialsInputBuilder,
}
impl DeleteChapCredentialsFluentBuilder {
    /// Creates a new `DeleteChapCredentials`.
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
            crate::operation::delete_chap_credentials::DeleteChapCredentials,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_chap_credentials::DeleteChapCredentialsError,
        >,
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
        crate::operation::delete_chap_credentials::DeleteChapCredentialsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_chap_credentials::DeleteChapCredentialsError,
        >,
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
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <code>DescribeStorediSCSIVolumes</code> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    pub fn target_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the iSCSI volume target. Use the <code>DescribeStorediSCSIVolumes</code> operation to return to retrieve the TargetARN for specified VolumeARN.</p>
    pub fn set_target_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The iSCSI initiator that connects to the target.</p>
    pub fn initiator_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.initiator_name(input.into());
        self
    }
    /// <p>The iSCSI initiator that connects to the target.</p>
    pub fn set_initiator_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_initiator_name(input);
        self
    }
}
