// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_account_settings::_update_account_settings_output::UpdateAccountSettingsOutputBuilder;

pub use crate::operation::update_account_settings::_update_account_settings_input::UpdateAccountSettingsInputBuilder;

/// Fluent builder constructing a request to `UpdateAccountSettings`.
///
/// <p>Updates the Amazon QuickSight settings in your Amazon Web Services account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAccountSettingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_account_settings::builders::UpdateAccountSettingsInputBuilder,
}
impl UpdateAccountSettingsFluentBuilder {
    /// Creates a new `UpdateAccountSettings`.
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
            crate::operation::update_account_settings::UpdateAccountSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
    /// <p>The ID for the Amazon Web Services account that contains the Amazon QuickSight settings that you want to list.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that contains the Amazon QuickSight settings that you want to list.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The default namespace for this Amazon Web Services account. Currently, the default is <code>default</code>. IAM users that register for the first time with Amazon QuickSight provide an email address that becomes associated with the default namespace. </p>
    pub fn default_namespace(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.default_namespace(input.into());
        self
    }
    /// <p>The default namespace for this Amazon Web Services account. Currently, the default is <code>default</code>. IAM users that register for the first time with Amazon QuickSight provide an email address that becomes associated with the default namespace. </p>
    pub fn set_default_namespace(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_default_namespace(input);
        self
    }
    /// <p>The email address that you want Amazon QuickSight to send notifications to regarding your Amazon Web Services account or Amazon QuickSight subscription.</p>
    pub fn notification_email(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.notification_email(input.into());
        self
    }
    /// <p>The email address that you want Amazon QuickSight to send notifications to regarding your Amazon Web Services account or Amazon QuickSight subscription.</p>
    pub fn set_notification_email(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_notification_email(input);
        self
    }
    /// <p>A boolean value that determines whether or not an Amazon QuickSight account can be deleted. A <code>True</code> value doesn't allow the account to be deleted and results in an error message if a user tries to make a <code>DeleteAccountSubscription</code> request. A <code>False</code> value will allow the account to be deleted.</p>
    pub fn termination_protection_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.termination_protection_enabled(input);
        self
    }
    /// <p>A boolean value that determines whether or not an Amazon QuickSight account can be deleted. A <code>True</code> value doesn't allow the account to be deleted and results in an error message if a user tries to make a <code>DeleteAccountSubscription</code> request. A <code>False</code> value will allow the account to be deleted.</p>
    pub fn set_termination_protection_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_termination_protection_enabled(input);
        self
    }
}
