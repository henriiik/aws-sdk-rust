// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_insight_events::_get_insight_events_output::GetInsightEventsOutputBuilder;

pub use crate::operation::get_insight_events::_get_insight_events_input::GetInsightEventsInputBuilder;

/// Fluent builder constructing a request to `GetInsightEvents`.
///
/// <p>X-Ray reevaluates insights periodically until they're resolved, and records each intermediate state as an event. You can review an insight's events in the Impact Timeline on the Inspect page in the X-Ray console.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetInsightEventsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_insight_events::builders::GetInsightEventsInputBuilder,
}
impl GetInsightEventsFluentBuilder {
    /// Creates a new `GetInsightEvents`.
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
            crate::operation::get_insight_events::GetInsightEvents,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_insight_events::GetInsightEventsError,
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
        crate::operation::get_insight_events::GetInsightEventsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_insight_events::GetInsightEventsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_insight_events::paginator::GetInsightEventsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_insight_events::paginator::GetInsightEventsPaginator {
        crate::operation::get_insight_events::paginator::GetInsightEventsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The insight's unique identifier. Use the GetInsightSummaries action to retrieve an InsightId.</p>
    pub fn insight_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.insight_id(input.into());
        self
    }
    /// <p>The insight's unique identifier. Use the GetInsightSummaries action to retrieve an InsightId.</p>
    pub fn set_insight_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_insight_id(input);
        self
    }
    /// <p>Used to retrieve at most the specified value of events.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Used to retrieve at most the specified value of events.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of events. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specify the pagination token returned by a previous request to retrieve the next page of events. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
