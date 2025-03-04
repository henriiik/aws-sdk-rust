// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_wireless_device_from_thing::_disassociate_wireless_device_from_thing_output::DisassociateWirelessDeviceFromThingOutputBuilder;

pub use crate::operation::disassociate_wireless_device_from_thing::_disassociate_wireless_device_from_thing_input::DisassociateWirelessDeviceFromThingInputBuilder;

/// Fluent builder constructing a request to `DisassociateWirelessDeviceFromThing`.
///
/// <p>Disassociates a wireless device from its currently associated thing.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateWirelessDeviceFromThingFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disassociate_wireless_device_from_thing::builders::DisassociateWirelessDeviceFromThingInputBuilder
            }
impl DisassociateWirelessDeviceFromThingFluentBuilder {
    /// Creates a new `DisassociateWirelessDeviceFromThing`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disassociate_wireless_device_from_thing::DisassociateWirelessDeviceFromThing, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::disassociate_wireless_device_from_thing::DisassociateWirelessDeviceFromThingError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::disassociate_wireless_device_from_thing::DisassociateWirelessDeviceFromThingOutput, aws_smithy_http::result::SdkError<crate::operation::disassociate_wireless_device_from_thing::DisassociateWirelessDeviceFromThingError>>
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
    /// <p>The ID of the resource to update.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the resource to update.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
}
