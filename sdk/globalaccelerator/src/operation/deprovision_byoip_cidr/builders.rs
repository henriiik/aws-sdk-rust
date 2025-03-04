// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deprovision_byoip_cidr::_deprovision_byoip_cidr_output::DeprovisionByoipCidrOutputBuilder;

pub use crate::operation::deprovision_byoip_cidr::_deprovision_byoip_cidr_input::DeprovisionByoipCidrInputBuilder;

/// Fluent builder constructing a request to `DeprovisionByoipCidr`.
///
/// <p>Releases the specified address range that you provisioned to use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool. </p>
/// <p>Before you can release an address range, you must stop advertising it by using <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/WithdrawByoipCidr.html">WithdrawByoipCidr</a> and you must not have any accelerators that are using static IP addresses allocated from its address range. </p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeprovisionByoipCidrFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrInputBuilder,
}
impl DeprovisionByoipCidrFluentBuilder {
    /// Creates a new `DeprovisionByoipCidr`.
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
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidr,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
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
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
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
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
}
