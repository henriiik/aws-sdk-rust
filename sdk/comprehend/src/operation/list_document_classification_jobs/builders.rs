// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_document_classification_jobs::_list_document_classification_jobs_output::ListDocumentClassificationJobsOutputBuilder;

pub use crate::operation::list_document_classification_jobs::_list_document_classification_jobs_input::ListDocumentClassificationJobsInputBuilder;

/// Fluent builder constructing a request to `ListDocumentClassificationJobs`.
///
/// <p>Gets a list of the documentation classification jobs that you have submitted.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListDocumentClassificationJobsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_document_classification_jobs::builders::ListDocumentClassificationJobsInputBuilder
            }
impl ListDocumentClassificationJobsFluentBuilder {
    /// Creates a new `ListDocumentClassificationJobs`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_document_classification_jobs::ListDocumentClassificationJobs, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::list_document_classification_jobs::ListDocumentClassificationJobsError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::list_document_classification_jobs::ListDocumentClassificationJobsOutput, aws_smithy_http::result::SdkError<crate::operation::list_document_classification_jobs::ListDocumentClassificationJobsError>>
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_document_classification_jobs::paginator::ListDocumentClassificationJobsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_document_classification_jobs::paginator::ListDocumentClassificationJobsPaginator{
        crate::operation::list_document_classification_jobs::paginator::ListDocumentClassificationJobsPaginator::new(self.handle, self.inner)
    }
    /// <p>Filters the jobs that are returned. You can filter jobs on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    pub fn filter(mut self, input: crate::types::DocumentClassificationJobFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>Filters the jobs that are returned. You can filter jobs on their names, status, or the date and time that they were submitted. You can only set one filter at a time.</p>
    pub fn set_filter(
        mut self,
        input: std::option::Option<crate::types::DocumentClassificationJobFilter>,
    ) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>Identifies the next page of results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Identifies the next page of results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in each page. The default is 100.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
