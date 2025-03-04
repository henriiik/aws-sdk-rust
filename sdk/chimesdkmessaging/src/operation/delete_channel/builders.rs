// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_channel::_delete_channel_output::DeleteChannelOutputBuilder;

pub use crate::operation::delete_channel::_delete_channel_input::DeleteChannelInputBuilder;

/// Fluent builder constructing a request to `DeleteChannel`.
///
/// <p>Immediately makes a channel and its memberships inaccessible and marks them for deletion. This is an irreversible process.</p> <note>
/// <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUserArn</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteChannelFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_channel::builders::DeleteChannelInputBuilder,
}
impl DeleteChannelFluentBuilder {
    /// Creates a new `DeleteChannel`.
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
            crate::operation::delete_channel::DeleteChannel,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_channel::DeleteChannelError>,
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
        crate::operation::delete_channel::DeleteChannelOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_channel::DeleteChannelError>,
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
    /// <p>The ARN of the channel being deleted.</p>
    pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the channel being deleted.</p>
    pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>The ID of the SubChannel in the request.</p>
    pub fn sub_channel_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sub_channel_id(input.into());
        self
    }
    /// <p>The ID of the SubChannel in the request.</p>
    pub fn set_sub_channel_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_sub_channel_id(input);
        self
    }
}
