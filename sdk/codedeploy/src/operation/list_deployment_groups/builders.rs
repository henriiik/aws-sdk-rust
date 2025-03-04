// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_deployment_groups::_list_deployment_groups_output::ListDeploymentGroupsOutputBuilder;

pub use crate::operation::list_deployment_groups::_list_deployment_groups_input::ListDeploymentGroupsInputBuilder;

/// Fluent builder constructing a request to `ListDeploymentGroups`.
///
/// <p>Lists the deployment groups for an application registered with the IAM user or Amazon Web Services account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListDeploymentGroupsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_deployment_groups::builders::ListDeploymentGroupsInputBuilder,
}
impl ListDeploymentGroupsFluentBuilder {
    /// Creates a new `ListDeploymentGroups`.
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
            crate::operation::list_deployment_groups::ListDeploymentGroups,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_deployment_groups::ListDeploymentGroupsError,
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
        crate::operation::list_deployment_groups::ListDeploymentGroupsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_deployment_groups::ListDeploymentGroupsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_deployment_groups::paginator::ListDeploymentGroupsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_deployment_groups::paginator::ListDeploymentGroupsPaginator {
        crate::operation::list_deployment_groups::paginator::ListDeploymentGroupsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.application_name(input.into());
        self
    }
    /// <p>The name of an CodeDeploy application associated with the IAM user or Amazon Web Services account.</p>
    pub fn set_application_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_application_name(input);
        self
    }
    /// <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An identifier returned from the previous list deployment groups call. It can be used to return the next set of deployment groups in the list.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
