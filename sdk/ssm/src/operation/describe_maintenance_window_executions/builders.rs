// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_maintenance_window_executions::_describe_maintenance_window_executions_output::DescribeMaintenanceWindowExecutionsOutputBuilder;

pub use crate::operation::describe_maintenance_window_executions::_describe_maintenance_window_executions_input::DescribeMaintenanceWindowExecutionsInputBuilder;

/// Fluent builder constructing a request to `DescribeMaintenanceWindowExecutions`.
///
/// <p>Lists the executions of a maintenance window. This includes information about when the maintenance window was scheduled to be active, and information about tasks registered and run with the maintenance window.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeMaintenanceWindowExecutionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_maintenance_window_executions::builders::DescribeMaintenanceWindowExecutionsInputBuilder
            }
impl DescribeMaintenanceWindowExecutionsFluentBuilder {
    /// Creates a new `DescribeMaintenanceWindowExecutions`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutions, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsOutput, aws_smithy_http::result::SdkError<crate::operation::describe_maintenance_window_executions::DescribeMaintenanceWindowExecutionsError>>
                     {
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator{
        crate::operation::describe_maintenance_window_executions::paginator::DescribeMaintenanceWindowExecutionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    pub fn window_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.window_id(input.into());
        self
    }
    /// <p>The ID of the maintenance window whose executions should be retrieved.</p>
    pub fn set_window_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_window_id(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Each entry in the array is a structure containing:</p>
    /// <ul>
    /// <li> <p>Key. A string between 1 and 128 characters. Supported keys include <code>ExecutedBefore</code> and <code>ExecutedAfter</code>.</p> </li>
    /// <li> <p>Values. An array of strings, each between 1 and 256 characters. Supported values are date/time strings in a valid ISO 8601 date/time format, such as <code>2021-11-04T05:00:00Z</code>.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::MaintenanceWindowFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Each entry in the array is a structure containing:</p>
    /// <ul>
    /// <li> <p>Key. A string between 1 and 128 characters. Supported keys include <code>ExecutedBefore</code> and <code>ExecutedAfter</code>.</p> </li>
    /// <li> <p>Values. An array of strings, each between 1 and 256 characters. Supported values are date/time strings in a valid ISO 8601 date/time format, such as <code>2021-11-04T05:00:00Z</code>.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::MaintenanceWindowFilter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a previous call.)</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
