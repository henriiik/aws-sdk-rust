// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_environment_vpcs::_list_environment_vpcs_output::ListEnvironmentVpcsOutputBuilder;

pub use crate::operation::list_environment_vpcs::_list_environment_vpcs_input::ListEnvironmentVpcsInputBuilder;

/// Fluent builder constructing a request to `ListEnvironmentVpcs`.
///
/// <p>Lists all Amazon Web Services Migration Hub Refactor Spaces service virtual private clouds (VPCs) that are part of the environment. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListEnvironmentVpcsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_environment_vpcs::builders::ListEnvironmentVpcsInputBuilder,
}
impl ListEnvironmentVpcsFluentBuilder {
    /// Creates a new `ListEnvironmentVpcs`.
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
            crate::operation::list_environment_vpcs::ListEnvironmentVpcs,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_environment_vpcs::ListEnvironmentVpcsError,
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
        crate::operation::list_environment_vpcs::ListEnvironmentVpcsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_environment_vpcs::ListEnvironmentVpcsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_environment_vpcs::paginator::ListEnvironmentVpcsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_environment_vpcs::paginator::ListEnvironmentVpcsPaginator {
        crate::operation::list_environment_vpcs::paginator::ListEnvironmentVpcsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ID of the environment. </p>
    pub fn environment_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The ID of the environment. </p>
    pub fn set_environment_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
