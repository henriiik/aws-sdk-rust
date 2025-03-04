// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reimport_api::_reimport_api_output::ReimportApiOutputBuilder;

pub use crate::operation::reimport_api::_reimport_api_input::ReimportApiInputBuilder;

/// Fluent builder constructing a request to `ReimportApi`.
///
/// <p>Puts an Api resource.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ReimportApiFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::reimport_api::builders::ReimportApiInputBuilder,
}
impl ReimportApiFluentBuilder {
    /// Creates a new `ReimportApi`.
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
            crate::operation::reimport_api::ReimportApi,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::reimport_api::ReimportApiError>,
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
        crate::operation::reimport_api::ReimportApiOutput,
        aws_smithy_http::result::SdkError<crate::operation::reimport_api::ReimportApiError>,
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
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>Specifies how to interpret the base path of the API during import. Valid values are ignore, prepend, and split. The default value is ignore. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-import-api-basePath.html">Set the OpenAPI basePath Property</a>. Supported only for HTTP APIs.</p>
    pub fn basepath(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.basepath(input.into());
        self
    }
    /// <p>Specifies how to interpret the base path of the API during import. Valid values are ignore, prepend, and split. The default value is ignore. To learn more, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-import-api-basePath.html">Set the OpenAPI basePath Property</a>. Supported only for HTTP APIs.</p>
    pub fn set_basepath(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_basepath(input);
        self
    }
    /// <p>The OpenAPI definition. Supported only for HTTP APIs.</p>
    pub fn body(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.body(input.into());
        self
    }
    /// <p>The OpenAPI definition. Supported only for HTTP APIs.</p>
    pub fn set_body(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_body(input);
        self
    }
    /// <p>Specifies whether to rollback the API creation when a warning is encountered. By default, API creation continues if a warning is encountered.</p>
    pub fn fail_on_warnings(mut self, input: bool) -> Self {
        self.inner = self.inner.fail_on_warnings(input);
        self
    }
    /// <p>Specifies whether to rollback the API creation when a warning is encountered. By default, API creation continues if a warning is encountered.</p>
    pub fn set_fail_on_warnings(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_fail_on_warnings(input);
        self
    }
}
