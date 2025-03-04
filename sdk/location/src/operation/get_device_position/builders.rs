// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_device_position::_get_device_position_output::GetDevicePositionOutputBuilder;

pub use crate::operation::get_device_position::_get_device_position_input::GetDevicePositionInputBuilder;

/// Fluent builder constructing a request to `GetDevicePosition`.
///
/// <p>Retrieves a device's most recent position according to its sample time.</p> <note>
/// <p>Device positions are deleted after 30 days.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetDevicePositionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_device_position::builders::GetDevicePositionInputBuilder,
}
impl GetDevicePositionFluentBuilder {
    /// Creates a new `GetDevicePosition`.
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
            crate::operation::get_device_position::GetDevicePosition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_device_position::GetDevicePositionError,
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
        crate::operation::get_device_position::GetDevicePositionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_device_position::GetDevicePositionError,
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
    /// <p>The tracker resource receiving the position update.</p>
    pub fn tracker_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.tracker_name(input.into());
        self
    }
    /// <p>The tracker resource receiving the position update.</p>
    pub fn set_tracker_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_tracker_name(input);
        self
    }
    /// <p>The device whose position you want to retrieve.</p>
    pub fn device_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The device whose position you want to retrieve.</p>
    pub fn set_device_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
}
