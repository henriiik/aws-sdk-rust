// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_workspaces::_describe_workspaces_output::DescribeWorkspacesOutputBuilder;

pub use crate::operation::describe_workspaces::_describe_workspaces_input::DescribeWorkspacesInputBuilder;

/// Fluent builder constructing a request to `DescribeWorkspaces`.
///
/// <p>Describes the specified WorkSpaces.</p>
/// <p>You can filter the results by using the bundle identifier, directory identifier, or owner, but you can specify only one filter at a time.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeWorkspacesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_workspaces::builders::DescribeWorkspacesInputBuilder,
}
impl DescribeWorkspacesFluentBuilder {
    /// Creates a new `DescribeWorkspaces`.
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
            crate::operation::describe_workspaces::DescribeWorkspaces,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_workspaces::DescribeWorkspacesError,
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
        crate::operation::describe_workspaces::DescribeWorkspacesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_workspaces::DescribeWorkspacesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_workspaces::paginator::DescribeWorkspacesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_workspaces::paginator::DescribeWorkspacesPaginator {
        crate::operation::describe_workspaces::paginator::DescribeWorkspacesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `WorkspaceIds`.
    ///
    /// To override the contents of this collection use [`set_workspace_ids`](Self::set_workspace_ids).
    ///
    /// <p>The identifiers of the WorkSpaces. You cannot combine this parameter with any other filter.</p>
    /// <p>Because the <code>CreateWorkspaces</code> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <code>DescribeWorkspaces</code> with this identifier, no information is returned.</p>
    pub fn workspace_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.workspace_ids(input.into());
        self
    }
    /// <p>The identifiers of the WorkSpaces. You cannot combine this parameter with any other filter.</p>
    /// <p>Because the <code>CreateWorkspaces</code> operation is asynchronous, the identifier it returns is not immediately available. If you immediately call <code>DescribeWorkspaces</code> with this identifier, no information is returned.</p>
    pub fn set_workspace_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_workspace_ids(input);
        self
    }
    /// <p>The identifier of the directory. In addition, you can optionally specify a specific directory user (see <code>UserName</code>). You cannot combine this parameter with any other filter.</p>
    pub fn directory_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory. In addition, you can optionally specify a specific directory user (see <code>UserName</code>). You cannot combine this parameter with any other filter.</p>
    pub fn set_directory_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The name of the directory user. You must specify this parameter with <code>DirectoryId</code>.</p>
    pub fn user_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>The name of the directory user. You must specify this parameter with <code>DirectoryId</code>.</p>
    pub fn set_user_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>The identifier of the bundle. All WorkSpaces that are created from this bundle are retrieved. You cannot combine this parameter with any other filter.</p>
    pub fn bundle_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bundle_id(input.into());
        self
    }
    /// <p>The identifier of the bundle. All WorkSpaces that are created from this bundle are retrieved. You cannot combine this parameter with any other filter.</p>
    pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bundle_id(input);
        self
    }
    /// <p>The maximum number of items to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items to return.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
