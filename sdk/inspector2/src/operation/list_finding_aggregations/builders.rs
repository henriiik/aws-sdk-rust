// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_finding_aggregations::_list_finding_aggregations_output::ListFindingAggregationsOutputBuilder;

pub use crate::operation::list_finding_aggregations::_list_finding_aggregations_input::ListFindingAggregationsInputBuilder;

/// Fluent builder constructing a request to `ListFindingAggregations`.
///
/// <p>Lists aggregated finding data for your environment based on specific criteria.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListFindingAggregationsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_finding_aggregations::builders::ListFindingAggregationsInputBuilder,
}
impl ListFindingAggregationsFluentBuilder {
    /// Creates a new `ListFindingAggregations`.
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
            crate::operation::list_finding_aggregations::ListFindingAggregations,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_finding_aggregations::ListFindingAggregationsError,
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
        crate::operation::list_finding_aggregations::ListFindingAggregationsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_finding_aggregations::ListFindingAggregationsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_finding_aggregations::paginator::ListFindingAggregationsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_finding_aggregations::paginator::ListFindingAggregationsPaginator
    {
        crate::operation::list_finding_aggregations::paginator::ListFindingAggregationsPaginator::new(self.handle, self.inner)
    }
    /// <p>The type of the aggregation request.</p>
    pub fn aggregation_type(mut self, input: crate::types::AggregationType) -> Self {
        self.inner = self.inner.aggregation_type(input);
        self
    }
    /// <p>The type of the aggregation request.</p>
    pub fn set_aggregation_type(
        mut self,
        input: std::option::Option<crate::types::AggregationType>,
    ) -> Self {
        self.inner = self.inner.set_aggregation_type(input);
        self
    }
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to use for paginating results that are returned in the response. Set the value of this parameter to null for the first request to a list action. For subsequent calls, use the <code>NextToken</code> value returned from the previous request to continue listing results after the first page.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in the response.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// Appends an item to `accountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>The Amazon Web Services account IDs to retrieve finding aggregation data for.</p>
    pub fn account_ids(mut self, input: crate::types::StringFilter) -> Self {
        self.inner = self.inner.account_ids(input);
        self
    }
    /// <p>The Amazon Web Services account IDs to retrieve finding aggregation data for.</p>
    pub fn set_account_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::StringFilter>>,
    ) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
    /// <p>Details of the aggregation request that is used to filter your aggregation results.</p>
    pub fn aggregation_request(mut self, input: crate::types::AggregationRequest) -> Self {
        self.inner = self.inner.aggregation_request(input);
        self
    }
    /// <p>Details of the aggregation request that is used to filter your aggregation results.</p>
    pub fn set_aggregation_request(
        mut self,
        input: std::option::Option<crate::types::AggregationRequest>,
    ) -> Self {
        self.inner = self.inner.set_aggregation_request(input);
        self
    }
}
