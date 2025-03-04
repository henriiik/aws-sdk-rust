// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_executions::_list_executions_output::ListExecutionsOutputBuilder;

pub use crate::operation::list_executions::_list_executions_input::ListExecutionsInputBuilder;

/// Fluent builder constructing a request to `ListExecutions`.
///
/// <p>Returns the status of tasks for one or more target devices.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListExecutionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_executions::builders::ListExecutionsInputBuilder,
}
impl ListExecutionsFluentBuilder {
    /// Creates a new `ListExecutions`.
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
            crate::operation::list_executions::ListExecutions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_executions::ListExecutionsError>,
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
        crate::operation::list_executions::ListExecutionsOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_executions::ListExecutionsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_executions::paginator::ListExecutionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_executions::paginator::ListExecutionsPaginator {
        crate::operation::list_executions::paginator::ListExecutionsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ID of the task.</p>
    pub fn task_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.task_id(input.into());
        self
    }
    /// <p>The ID of the task.</p>
    pub fn set_task_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_task_id(input);
        self
    }
    /// <p>A structure used to filter the tasks by their current state.</p>
    pub fn state(mut self, input: crate::types::ExecutionState) -> Self {
        self.inner = self.inner.state(input);
        self
    }
    /// <p>A structure used to filter the tasks by their current state.</p>
    pub fn set_state(mut self, input: std::option::Option<crate::types::ExecutionState>) -> Self {
        self.inner = self.inner.set_state(input);
        self
    }
    /// <p>The maximum number of tasks to list per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of tasks to list per page.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>A pagination token to continue to the next page of tasks.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A pagination token to continue to the next page of tasks.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
