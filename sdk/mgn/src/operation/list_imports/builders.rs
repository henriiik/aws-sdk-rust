// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_imports::_list_imports_output::ListImportsOutputBuilder;

pub use crate::operation::list_imports::_list_imports_input::ListImportsInputBuilder;

/// Fluent builder constructing a request to `ListImports`.
///
/// <p>List imports.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListImportsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_imports::builders::ListImportsInputBuilder,
}
impl ListImportsFluentBuilder {
    /// Creates a new `ListImports`.
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
            crate::operation::list_imports::ListImports,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::list_imports::ListImportsError>,
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
        crate::operation::list_imports::ListImportsOutput,
        aws_smithy_http::result::SdkError<crate::operation::list_imports::ListImportsError>,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_imports::paginator::ListImportsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_imports::paginator::ListImportsPaginator {
        crate::operation::list_imports::paginator::ListImportsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>List imports request filters.</p>
    pub fn filters(mut self, input: crate::types::ListImportsRequestFilters) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>List imports request filters.</p>
    pub fn set_filters(
        mut self,
        input: std::option::Option<crate::types::ListImportsRequestFilters>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>List imports request max results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>List imports request max results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>List imports request next token.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>List imports request next token.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
