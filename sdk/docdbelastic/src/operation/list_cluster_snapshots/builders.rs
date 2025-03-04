// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_cluster_snapshots::_list_cluster_snapshots_output::ListClusterSnapshotsOutputBuilder;

pub use crate::operation::list_cluster_snapshots::_list_cluster_snapshots_input::ListClusterSnapshotsInputBuilder;

/// Fluent builder constructing a request to `ListClusterSnapshots`.
///
/// <p>Returns information about Elastic DocumentDB snapshots for a specified cluster.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListClusterSnapshotsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_cluster_snapshots::builders::ListClusterSnapshotsInputBuilder,
}
impl ListClusterSnapshotsFluentBuilder {
    /// Creates a new `ListClusterSnapshots`.
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
            crate::operation::list_cluster_snapshots::ListClusterSnapshots,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_cluster_snapshots::ListClusterSnapshotsError,
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
        crate::operation::list_cluster_snapshots::ListClusterSnapshotsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_cluster_snapshots::ListClusterSnapshotsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_cluster_snapshots::paginator::ListClusterSnapshotsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_cluster_snapshots::paginator::ListClusterSnapshotsPaginator {
        crate::operation::list_cluster_snapshots::paginator::ListClusterSnapshotsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    pub fn cluster_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cluster_arn(input.into());
        self
    }
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    pub fn set_cluster_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_arn(input);
        self
    }
    /// <p>The nextToken which is used the get the next page of data.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The nextToken which is used the get the next page of data.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of entries to recieve in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of entries to recieve in the response.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
