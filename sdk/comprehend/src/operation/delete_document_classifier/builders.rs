// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_document_classifier::_delete_document_classifier_output::DeleteDocumentClassifierOutputBuilder;

pub use crate::operation::delete_document_classifier::_delete_document_classifier_input::DeleteDocumentClassifierInputBuilder;

/// Fluent builder constructing a request to `DeleteDocumentClassifier`.
///
/// <p>Deletes a previously created document classifier</p>
/// <p>Only those classifiers that are in terminated states (IN_ERROR, TRAINED) will be deleted. If an active inference job is using the model, a <code>ResourceInUseException</code> will be returned.</p>
/// <p>This is an asynchronous action that puts the classifier into a DELETING state, and it is then removed by a background job. Once removed, the classifier disappears from your account and is no longer available for use. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDocumentClassifierFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_document_classifier::builders::DeleteDocumentClassifierInputBuilder
            }
impl DeleteDocumentClassifierFluentBuilder {
    /// Creates a new `DeleteDocumentClassifier`.
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
            crate::operation::delete_document_classifier::DeleteDocumentClassifier,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_document_classifier::DeleteDocumentClassifierError,
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
        crate::operation::delete_document_classifier::DeleteDocumentClassifierOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_document_classifier::DeleteDocumentClassifierError,
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
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    pub fn document_classifier_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.document_classifier_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the document classifier. </p>
    pub fn set_document_classifier_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_document_classifier_arn(input);
        self
    }
}
