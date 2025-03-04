// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_inventory_deletions::_describe_inventory_deletions_output::DescribeInventoryDeletionsOutputBuilder;

pub use crate::operation::describe_inventory_deletions::_describe_inventory_deletions_input::DescribeInventoryDeletionsInputBuilder;

/// Fluent builder constructing a request to `DescribeInventoryDeletions`.
///
/// <p>Describes a specific delete inventory operation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeInventoryDeletionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_inventory_deletions::builders::DescribeInventoryDeletionsInputBuilder
            }
impl DescribeInventoryDeletionsFluentBuilder {
    /// Creates a new `DescribeInventoryDeletions`.
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
            crate::operation::describe_inventory_deletions::DescribeInventoryDeletions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_inventory_deletions::DescribeInventoryDeletionsError,
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
        crate::operation::describe_inventory_deletions::DescribeInventoryDeletionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_inventory_deletions::DescribeInventoryDeletionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_inventory_deletions::paginator::DescribeInventoryDeletionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_inventory_deletions::paginator::DescribeInventoryDeletionsPaginator{
        crate::operation::describe_inventory_deletions::paginator::DescribeInventoryDeletionsPaginator::new(self.handle, self.inner)
    }
    /// <p>Specify the delete inventory ID for which you want information. This ID was returned by the <code>DeleteInventory</code> operation.</p>
    pub fn deletion_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.deletion_id(input.into());
        self
    }
    /// <p>Specify the delete inventory ID for which you want information. This ID was returned by the <code>DeleteInventory</code> operation.</p>
    pub fn set_deletion_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_deletion_id(input);
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token to start the list. Use this token to get the next set of results. </p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
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
}
