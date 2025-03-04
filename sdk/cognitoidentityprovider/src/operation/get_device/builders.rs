// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_device::_get_device_output::GetDeviceOutputBuilder;

pub use crate::operation::get_device::_get_device_input::GetDeviceInputBuilder;

/// Fluent builder constructing a request to `GetDevice`.
///
/// <p>Gets the device.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetDeviceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_device::builders::GetDeviceInputBuilder,
}
impl GetDeviceFluentBuilder {
    /// Creates a new `GetDevice`.
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
            crate::operation::get_device::GetDevice,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::get_device::GetDeviceError>,
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
        crate::operation::get_device::GetDeviceOutput,
        aws_smithy_http::result::SdkError<crate::operation::get_device::GetDeviceError>,
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
    /// <p>The device key.</p>
    pub fn device_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_key(input.into());
        self
    }
    /// <p>The device key.</p>
    pub fn set_device_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_device_key(input);
        self
    }
    /// <p>A valid access token that Amazon Cognito issued to the user whose device information you want to request.</p>
    pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.access_token(input.into());
        self
    }
    /// <p>A valid access token that Amazon Cognito issued to the user whose device information you want to request.</p>
    pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_access_token(input);
        self
    }
}
