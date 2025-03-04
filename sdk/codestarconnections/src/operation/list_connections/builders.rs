// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_connections::_list_connections_output::ListConnectionsOutputBuilder;

pub use crate::operation::list_connections::_list_connections_input::ListConnectionsInputBuilder;

/// Fluent builder constructing a request to `ListConnections`.
///
/// <p>Lists the connections associated with your account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListConnectionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_connections::builders::ListConnectionsInputBuilder,
}
impl ListConnectionsFluentBuilder {
    /// Creates a new `ListConnections`.
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
            crate::operation::list_connections::ListConnections,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_connections::ListConnectionsError>,
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
        crate::operation::list_connections::ListConnectionsOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_connections::ListConnectionsError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_connections::paginator::ListConnectionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_connections::paginator::ListConnectionsPaginator {
        crate::operation::list_connections::paginator::ListConnectionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Filters the list of connections to those associated with a specified provider, such as Bitbucket.</p>
    pub fn provider_type_filter(mut self, input: crate::types::ProviderType) -> Self {
        self.inner = self.inner.provider_type_filter(input);
        self
    }
    /// <p>Filters the list of connections to those associated with a specified provider, such as Bitbucket.</p>
    pub fn set_provider_type_filter(
        mut self,
        input: std::option::Option<crate::types::ProviderType>,
    ) -> Self {
        self.inner = self.inner.set_provider_type_filter(input);
        self
    }
    /// <p>Filters the list of connections to those associated with a specified host.</p>
    pub fn host_arn_filter(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.host_arn_filter(input.into());
        self
    }
    /// <p>Filters the list of connections to those associated with a specified host.</p>
    pub fn set_host_arn_filter(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_host_arn_filter(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token that was returned from the previous <code>ListConnections</code> call, which can be used to return the next set of connections in the list.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token that was returned from the previous <code>ListConnections</code> call, which can be used to return the next set of connections in the list.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
