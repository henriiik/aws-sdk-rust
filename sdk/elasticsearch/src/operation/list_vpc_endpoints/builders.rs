// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_vpc_endpoints::_list_vpc_endpoints_output::ListVpcEndpointsOutputBuilder;

pub use crate::operation::list_vpc_endpoints::_list_vpc_endpoints_input::ListVpcEndpointsInputBuilder;

/// Fluent builder constructing a request to `ListVpcEndpoints`.
///
/// <p>Retrieves all Amazon OpenSearch Service-managed VPC endpoints in the current account and Region.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListVpcEndpointsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_vpc_endpoints::builders::ListVpcEndpointsInputBuilder,
}
impl ListVpcEndpointsFluentBuilder {
    /// Creates a new `ListVpcEndpoints`.
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
            crate::operation::list_vpc_endpoints::ListVpcEndpoints,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints::ListVpcEndpointsError,
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
        crate::operation::list_vpc_endpoints::ListVpcEndpointsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_vpc_endpoints::ListVpcEndpointsError,
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
    /// <p>Identifier to allow retrieval of paginated results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Identifier to allow retrieval of paginated results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
