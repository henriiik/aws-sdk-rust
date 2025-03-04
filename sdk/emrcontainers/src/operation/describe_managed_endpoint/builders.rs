// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_managed_endpoint::_describe_managed_endpoint_output::DescribeManagedEndpointOutputBuilder;

pub use crate::operation::describe_managed_endpoint::_describe_managed_endpoint_input::DescribeManagedEndpointInputBuilder;

/// Fluent builder constructing a request to `DescribeManagedEndpoint`.
///
/// <p>Displays detailed information about a managed endpoint. A managed endpoint is a gateway that connects Amazon EMR Studio to Amazon EMR on EKS so that Amazon EMR Studio can communicate with your virtual cluster.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeManagedEndpointFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::describe_managed_endpoint::builders::DescribeManagedEndpointInputBuilder,
}
impl DescribeManagedEndpointFluentBuilder {
    /// Creates a new `DescribeManagedEndpoint`.
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
            crate::operation::describe_managed_endpoint::DescribeManagedEndpoint,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_managed_endpoint::DescribeManagedEndpointError,
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
        crate::operation::describe_managed_endpoint::DescribeManagedEndpointOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_managed_endpoint::DescribeManagedEndpointError,
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
    /// <p>This output displays ID of the managed endpoint.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>This output displays ID of the managed endpoint.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the endpoint's virtual cluster.</p>
    pub fn virtual_cluster_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.virtual_cluster_id(input.into());
        self
    }
    /// <p>The ID of the endpoint's virtual cluster.</p>
    pub fn set_virtual_cluster_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_virtual_cluster_id(input);
        self
    }
}
