// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_channel_expiration_settings::_put_channel_expiration_settings_output::PutChannelExpirationSettingsOutputBuilder;

pub use crate::operation::put_channel_expiration_settings::_put_channel_expiration_settings_input::PutChannelExpirationSettingsInputBuilder;

/// Fluent builder constructing a request to `PutChannelExpirationSettings`.
///
/// <p>Sets the number of days before the channel is automatically deleted.</p> <note>
/// <ul>
/// <li> <p>A background process deletes expired channels within 6 hours of expiration. Actual deletion times may vary.</p> </li>
/// <li> <p>Expired channels that have not yet been deleted appear as active, and you can update their expiration settings. The system honors the new settings.</p> </li>
/// <li> <p>The <code>x-amz-chime-bearer</code> request header is mandatory. Use the ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call as the value in the header.</p> </li>
/// </ul>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutChannelExpirationSettingsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::put_channel_expiration_settings::builders::PutChannelExpirationSettingsInputBuilder
            }
impl PutChannelExpirationSettingsFluentBuilder {
    /// Creates a new `PutChannelExpirationSettings`.
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
            crate::operation::put_channel_expiration_settings::PutChannelExpirationSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsError,
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
        crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_channel_expiration_settings::PutChannelExpirationSettingsError,
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
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.channel_arn(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_channel_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.chime_bearer(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_chime_bearer(input);
        self
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn expiration_settings(mut self, input: crate::types::ExpirationSettings) -> Self {
        self.inner = self.inner.expiration_settings(input);
        self
    }
    /// <p>Settings that control the interval after which a channel is deleted.</p>
    pub fn set_expiration_settings(
        mut self,
        input: std::option::Option<crate::types::ExpirationSettings>,
    ) -> Self {
        self.inner = self.inner.set_expiration_settings(input);
        self
    }
}
