// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_backend::_get_backend_output::GetBackendOutputBuilder;

pub use crate::operation::get_backend::_get_backend_input::GetBackendInputBuilder;

/// Fluent builder constructing a request to `GetBackend`.
///
/// <p>Provides project-level details for your Amplify UI project.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetBackendFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_backend::builders::GetBackendInputBuilder,
}
impl GetBackendFluentBuilder {
    /// Creates a new `GetBackend`.
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
            crate::operation::get_backend::GetBackend,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_backend::GetBackendError>,
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
        crate::operation::get_backend::GetBackendOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_backend::GetBackendError>,
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
    /// <p>The app ID.</p>
    pub fn app_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.app_id(input.into());
        self
    }
    /// <p>The app ID.</p>
    pub fn set_app_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_app_id(input);
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn backend_environment_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.backend_environment_name(input.into());
        self
    }
    /// <p>The name of the backend environment.</p>
    pub fn set_backend_environment_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_backend_environment_name(input);
        self
    }
}
