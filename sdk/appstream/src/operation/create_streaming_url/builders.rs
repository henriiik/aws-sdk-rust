// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_streaming_url::_create_streaming_url_output::CreateStreamingUrlOutputBuilder;

pub use crate::operation::create_streaming_url::_create_streaming_url_input::CreateStreamingUrlInputBuilder;

/// Fluent builder constructing a request to `CreateStreamingURL`.
///
/// <p>Creates a temporary URL to start an AppStream 2.0 streaming session for the specified user. A streaming URL enables application streaming to be tested without user setup. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateStreamingURLFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_streaming_url::builders::CreateStreamingUrlInputBuilder,
}
impl CreateStreamingURLFluentBuilder {
    /// Creates a new `CreateStreamingURL`.
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
            crate::operation::create_streaming_url::CreateStreamingURL,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_streaming_url::CreateStreamingURLError,
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
        crate::operation::create_streaming_url::CreateStreamingUrlOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_streaming_url::CreateStreamingURLError,
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
    /// <p>The name of the stack.</p>
    pub fn stack_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stack_name(input.into());
        self
    }
    /// <p>The name of the stack.</p>
    pub fn set_stack_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stack_name(input);
        self
    }
    /// <p>The name of the fleet.</p>
    pub fn fleet_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.fleet_name(input.into());
        self
    }
    /// <p>The name of the fleet.</p>
    pub fn set_fleet_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_name(input);
        self
    }
    /// <p>The identifier of the user.</p>
    pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The identifier of the user.</p>
    pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The name of the application to launch after the session starts. This is the name that you specified as <b>Name</b> in the Image Assistant. If your fleet is enabled for the <b>Desktop</b> stream view, you can also choose to launch directly to the operating system desktop. To do so, specify <b>Desktop</b>.</p>
    pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The name of the application to launch after the session starts. This is the name that you specified as <b>Name</b> in the Image Assistant. If your fleet is enabled for the <b>Desktop</b> stream view, you can also choose to launch directly to the operating system desktop. To do so, specify <b>Desktop</b>.</p>
    pub fn set_application_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 60 seconds.</p>
    pub fn validity(mut self, input: i64) -> Self {
        self.inner = self.inner.validity(input);
        self
    }
    /// <p>The time that the streaming URL will be valid, in seconds. Specify a value between 1 and 604800 seconds. The default is 60 seconds.</p>
    pub fn set_validity(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_validity(input);
        self
    }
    /// <p>The session context. For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/managing-stacks-fleets.html#managing-stacks-fleets-parameters">Session Context</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    pub fn session_context(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.session_context(input.into());
        self
    }
    /// <p>The session context. For more information, see <a href="https://docs.aws.amazon.com/appstream2/latest/developerguide/managing-stacks-fleets.html#managing-stacks-fleets-parameters">Session Context</a> in the <i>Amazon AppStream 2.0 Administration Guide</i>.</p>
    pub fn set_session_context(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_session_context(input);
        self
    }
}
