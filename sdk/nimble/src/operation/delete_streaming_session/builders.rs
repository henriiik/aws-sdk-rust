// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_streaming_session::_delete_streaming_session_output::DeleteStreamingSessionOutputBuilder;

pub use crate::operation::delete_streaming_session::_delete_streaming_session_input::DeleteStreamingSessionInputBuilder;

/// Fluent builder constructing a request to `DeleteStreamingSession`.
///
/// <p>Deletes streaming session resource.</p>
/// <p>After invoking this operation, use GetStreamingSession to poll the resource until it transitions to a <code>DELETED</code> state.</p>
/// <p>A streaming session will count against your streaming session quota until it is marked <code>DELETED</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteStreamingSessionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_streaming_session::builders::DeleteStreamingSessionInputBuilder,
}
impl DeleteStreamingSessionFluentBuilder {
    /// Creates a new `DeleteStreamingSession`.
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
            crate::operation::delete_streaming_session::DeleteStreamingSession,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_streaming_session::DeleteStreamingSessionError,
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
        crate::operation::delete_streaming_session::DeleteStreamingSessionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_streaming_session::DeleteStreamingSessionError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you don’t specify a client token, the Amazon Web Services SDK automatically generates a client token and uses it for the request to ensure idempotency.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The streaming session ID.</p>
    pub fn session_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>The streaming session ID.</p>
    pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>The studio ID. </p>
    pub fn studio_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.studio_id(input.into());
        self
    }
    /// <p>The studio ID. </p>
    pub fn set_studio_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_studio_id(input);
        self
    }
}
