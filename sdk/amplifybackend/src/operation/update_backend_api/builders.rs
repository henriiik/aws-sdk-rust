// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_backend_api::_update_backend_api_output::UpdateBackendApiOutputBuilder;

pub use crate::operation::update_backend_api::_update_backend_api_input::UpdateBackendApiInputBuilder;

/// Fluent builder constructing a request to `UpdateBackendAPI`.
///
/// <p>Updates an existing backend API resource.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBackendAPIFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_backend_api::builders::UpdateBackendApiInputBuilder,
}
impl UpdateBackendAPIFluentBuilder {
    /// Creates a new `UpdateBackendAPI`.
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
            crate::operation::update_backend_api::UpdateBackendAPI,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_backend_api::UpdateBackendAPIError,
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
        crate::operation::update_backend_api::UpdateBackendApiOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_backend_api::UpdateBackendAPIError,
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
    /// <p>Defines the resource configuration for the data model in your Amplify project.</p>
    pub fn resource_config(mut self, input: crate::types::BackendApiResourceConfig) -> Self {
        self.inner = self.inner.resource_config(input);
        self
    }
    /// <p>Defines the resource configuration for the data model in your Amplify project.</p>
    pub fn set_resource_config(
        mut self,
        input: std::option::Option<crate::types::BackendApiResourceConfig>,
    ) -> Self {
        self.inner = self.inner.set_resource_config(input);
        self
    }
    /// <p>The name of this resource.</p>
    pub fn resource_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_name(input.into());
        self
    }
    /// <p>The name of this resource.</p>
    pub fn set_resource_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_name(input);
        self
    }
}
