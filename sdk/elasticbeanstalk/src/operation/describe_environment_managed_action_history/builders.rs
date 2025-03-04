// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_environment_managed_action_history::_describe_environment_managed_action_history_output::DescribeEnvironmentManagedActionHistoryOutputBuilder;

pub use crate::operation::describe_environment_managed_action_history::_describe_environment_managed_action_history_input::DescribeEnvironmentManagedActionHistoryInputBuilder;

/// Fluent builder constructing a request to `DescribeEnvironmentManagedActionHistory`.
///
/// <p>Lists an environment's completed and failed managed actions.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeEnvironmentManagedActionHistoryFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_environment_managed_action_history::builders::DescribeEnvironmentManagedActionHistoryInputBuilder
            }
impl DescribeEnvironmentManagedActionHistoryFluentBuilder {
    /// Creates a new `DescribeEnvironmentManagedActionHistory`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistory, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryOutput, aws_smithy_http::result::SdkError<crate::operation::describe_environment_managed_action_history::DescribeEnvironmentManagedActionHistoryError>>
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_environment_managed_action_history::paginator::DescribeEnvironmentManagedActionHistoryPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_environment_managed_action_history::paginator::DescribeEnvironmentManagedActionHistoryPaginator{
        crate::operation::describe_environment_managed_action_history::paginator::DescribeEnvironmentManagedActionHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>The environment ID of the target environment.</p>
    pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The environment ID of the target environment.</p>
    pub fn set_environment_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The name of the target environment.</p>
    pub fn environment_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>The name of the target environment.</p>
    pub fn set_environment_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
    /// <p>The pagination token returned by a previous request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token returned by a previous request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of items to return for a single request.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of items to return for a single request.</p>
    pub fn set_max_items(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
}
