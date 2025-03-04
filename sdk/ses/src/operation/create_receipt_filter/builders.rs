// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_receipt_filter::_create_receipt_filter_output::CreateReceiptFilterOutputBuilder;

pub use crate::operation::create_receipt_filter::_create_receipt_filter_input::CreateReceiptFilterInputBuilder;

/// Fluent builder constructing a request to `CreateReceiptFilter`.
///
/// <p>Creates a new IP address filter.</p>
/// <p>For information about setting up IP address filters, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-ip-filters.html">Amazon SES Developer Guide</a>.</p>
/// <p>You can execute this operation no more than once per second.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateReceiptFilterFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_receipt_filter::builders::CreateReceiptFilterInputBuilder,
}
impl CreateReceiptFilterFluentBuilder {
    /// Creates a new `CreateReceiptFilter`.
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
            crate::operation::create_receipt_filter::CreateReceiptFilter,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_receipt_filter::CreateReceiptFilterError,
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
        crate::operation::create_receipt_filter::CreateReceiptFilterOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_receipt_filter::CreateReceiptFilterError,
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
    /// <p>A data structure that describes the IP address filter to create, which consists of a name, an IP address range, and whether to allow or block mail from it.</p>
    pub fn filter(mut self, input: crate::types::ReceiptFilter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>A data structure that describes the IP address filter to create, which consists of a name, an IP address range, and whether to allow or block mail from it.</p>
    pub fn set_filter(mut self, input: std::option::Option<crate::types::ReceiptFilter>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
}
