// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_schema_extension::_cancel_schema_extension_output::CancelSchemaExtensionOutputBuilder;

pub use crate::operation::cancel_schema_extension::_cancel_schema_extension_input::CancelSchemaExtensionInputBuilder;

/// Fluent builder constructing a request to `CancelSchemaExtension`.
///
/// <p>Cancels an in-progress schema extension to a Microsoft AD directory. Once a schema extension has started replicating to all domain controllers, the task can no longer be canceled. A schema extension can be canceled during any of the following states; <code>Initializing</code>, <code>CreatingSnapshot</code>, and <code>UpdatingSchema</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CancelSchemaExtensionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_schema_extension::builders::CancelSchemaExtensionInputBuilder,
}
impl CancelSchemaExtensionFluentBuilder {
    /// Creates a new `CancelSchemaExtension`.
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
            crate::operation::cancel_schema_extension::CancelSchemaExtension,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::cancel_schema_extension::CancelSchemaExtensionError,
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
        crate::operation::cancel_schema_extension::CancelSchemaExtensionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::cancel_schema_extension::CancelSchemaExtensionError,
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
    /// <p>The identifier of the directory whose schema extension will be canceled.</p>
    pub fn directory_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory whose schema extension will be canceled.</p>
    pub fn set_directory_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier of the schema extension that will be canceled.</p>
    pub fn schema_extension_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.schema_extension_id(input.into());
        self
    }
    /// <p>The identifier of the schema extension that will be canceled.</p>
    pub fn set_schema_extension_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_schema_extension_id(input);
        self
    }
}
