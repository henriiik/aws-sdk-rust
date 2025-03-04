// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_phone_number::_batch_update_phone_number_output::BatchUpdatePhoneNumberOutputBuilder;

pub use crate::operation::batch_update_phone_number::_batch_update_phone_number_input::BatchUpdatePhoneNumberInputBuilder;

/// Fluent builder constructing a request to `BatchUpdatePhoneNumber`.
///
/// <p>Updates one or more phone numbers.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct BatchUpdatePhoneNumberFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::batch_update_phone_number::builders::BatchUpdatePhoneNumberInputBuilder,
}
impl BatchUpdatePhoneNumberFluentBuilder {
    /// Creates a new `BatchUpdatePhoneNumber`.
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
            crate::operation::batch_update_phone_number::BatchUpdatePhoneNumber,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberError,
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
        crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::batch_update_phone_number::BatchUpdatePhoneNumberError,
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
    /// Appends an item to `UpdatePhoneNumberRequestItems`.
    ///
    /// To override the contents of this collection use [`set_update_phone_number_request_items`](Self::set_update_phone_number_request_items).
    ///
    /// <p>Lists the phone numbers in the update request.</p>
    pub fn update_phone_number_request_items(
        mut self,
        input: crate::types::UpdatePhoneNumberRequestItem,
    ) -> Self {
        self.inner = self.inner.update_phone_number_request_items(input);
        self
    }
    /// <p>Lists the phone numbers in the update request.</p>
    pub fn set_update_phone_number_request_items(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::UpdatePhoneNumberRequestItem>>,
    ) -> Self {
        self.inner = self.inner.set_update_phone_number_request_items(input);
        self
    }
}
