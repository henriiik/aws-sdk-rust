// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_set_user_mfa_preference::_admin_set_user_mfa_preference_output::AdminSetUserMfaPreferenceOutputBuilder;

pub use crate::operation::admin_set_user_mfa_preference::_admin_set_user_mfa_preference_input::AdminSetUserMfaPreferenceInputBuilder;

/// Fluent builder constructing a request to `AdminSetUserMFAPreference`.
///
/// <p>The user's multi-factor authentication (MFA) preference, including which MFA options are activated, and if any are preferred. Only one factor can be set as preferred. The preferred MFA factor will be used to authenticate a user if multiple factors are activated. If multiple options are activated and no preference is set, a challenge to choose an MFA option will be returned during sign-in.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AdminSetUserMFAPreferenceFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::admin_set_user_mfa_preference::builders::AdminSetUserMfaPreferenceInputBuilder
            }
impl AdminSetUserMFAPreferenceFluentBuilder {
    /// Creates a new `AdminSetUserMFAPreference`.
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
            crate::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreference,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
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
        crate::operation::admin_set_user_mfa_preference::AdminSetUserMfaPreferenceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::admin_set_user_mfa_preference::AdminSetUserMFAPreferenceError,
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
    /// <p>The SMS text message MFA settings.</p>
    pub fn sms_mfa_settings(mut self, input: crate::types::SmsMfaSettingsType) -> Self {
        self.inner = self.inner.sms_mfa_settings(input);
        self
    }
    /// <p>The SMS text message MFA settings.</p>
    pub fn set_sms_mfa_settings(
        mut self,
        input: std::option::Option<crate::types::SmsMfaSettingsType>,
    ) -> Self {
        self.inner = self.inner.set_sms_mfa_settings(input);
        self
    }
    /// <p>The time-based one-time password software token MFA settings.</p>
    pub fn software_token_mfa_settings(
        mut self,
        input: crate::types::SoftwareTokenMfaSettingsType,
    ) -> Self {
        self.inner = self.inner.software_token_mfa_settings(input);
        self
    }
    /// <p>The time-based one-time password software token MFA settings.</p>
    pub fn set_software_token_mfa_settings(
        mut self,
        input: std::option::Option<crate::types::SoftwareTokenMfaSettingsType>,
    ) -> Self {
        self.inner = self.inner.set_software_token_mfa_settings(input);
        self
    }
    /// <p>The user pool username or alias.</p>
    pub fn username(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user pool username or alias.</p>
    pub fn set_username(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
}
