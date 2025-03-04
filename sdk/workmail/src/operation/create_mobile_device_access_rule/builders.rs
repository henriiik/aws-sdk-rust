// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_mobile_device_access_rule::_create_mobile_device_access_rule_output::CreateMobileDeviceAccessRuleOutputBuilder;

pub use crate::operation::create_mobile_device_access_rule::_create_mobile_device_access_rule_input::CreateMobileDeviceAccessRuleInputBuilder;

/// Fluent builder constructing a request to `CreateMobileDeviceAccessRule`.
///
/// <p>Creates a new mobile device access rule for the specified WorkMail organization.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateMobileDeviceAccessRuleFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_mobile_device_access_rule::builders::CreateMobileDeviceAccessRuleInputBuilder
            }
impl CreateMobileDeviceAccessRuleFluentBuilder {
    /// Creates a new `CreateMobileDeviceAccessRule`.
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
            crate::operation::create_mobile_device_access_rule::CreateMobileDeviceAccessRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_mobile_device_access_rule::CreateMobileDeviceAccessRuleError,
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
        crate::operation::create_mobile_device_access_rule::CreateMobileDeviceAccessRuleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_mobile_device_access_rule::CreateMobileDeviceAccessRuleError,
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
    /// <p>The WorkMail organization under which the rule will be created.</p>
    pub fn organization_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The WorkMail organization under which the rule will be created.</p>
    pub fn set_organization_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The idempotency token for the client request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token for the client request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The rule name.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The rule name.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The rule description.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The rule description.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The effect of the rule when it matches. Allowed values are <code>ALLOW</code> or <code>DENY</code>.</p>
    pub fn effect(mut self, input: crate::types::MobileDeviceAccessRuleEffect) -> Self {
        self.inner = self.inner.effect(input);
        self
    }
    /// <p>The effect of the rule when it matches. Allowed values are <code>ALLOW</code> or <code>DENY</code>.</p>
    pub fn set_effect(
        mut self,
        input: std::option::Option<crate::types::MobileDeviceAccessRuleEffect>,
    ) -> Self {
        self.inner = self.inner.set_effect(input);
        self
    }
    /// Appends an item to `DeviceTypes`.
    ///
    /// To override the contents of this collection use [`set_device_types`](Self::set_device_types).
    ///
    /// <p>Device types that the rule will match.</p>
    pub fn device_types(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_types(input.into());
        self
    }
    /// <p>Device types that the rule will match.</p>
    pub fn set_device_types(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_device_types(input);
        self
    }
    /// Appends an item to `NotDeviceTypes`.
    ///
    /// To override the contents of this collection use [`set_not_device_types`](Self::set_not_device_types).
    ///
    /// <p>Device types that the rule <b>will not</b> match. All other device types will match.</p>
    pub fn not_device_types(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.not_device_types(input.into());
        self
    }
    /// <p>Device types that the rule <b>will not</b> match. All other device types will match.</p>
    pub fn set_not_device_types(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_not_device_types(input);
        self
    }
    /// Appends an item to `DeviceModels`.
    ///
    /// To override the contents of this collection use [`set_device_models`](Self::set_device_models).
    ///
    /// <p>Device models that the rule will match.</p>
    pub fn device_models(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_models(input.into());
        self
    }
    /// <p>Device models that the rule will match.</p>
    pub fn set_device_models(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_device_models(input);
        self
    }
    /// Appends an item to `NotDeviceModels`.
    ///
    /// To override the contents of this collection use [`set_not_device_models`](Self::set_not_device_models).
    ///
    /// <p>Device models that the rule <b>will not</b> match. All other device models will match.</p>
    pub fn not_device_models(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.not_device_models(input.into());
        self
    }
    /// <p>Device models that the rule <b>will not</b> match. All other device models will match.</p>
    pub fn set_not_device_models(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_not_device_models(input);
        self
    }
    /// Appends an item to `DeviceOperatingSystems`.
    ///
    /// To override the contents of this collection use [`set_device_operating_systems`](Self::set_device_operating_systems).
    ///
    /// <p>Device operating systems that the rule will match.</p>
    pub fn device_operating_systems(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_operating_systems(input.into());
        self
    }
    /// <p>Device operating systems that the rule will match.</p>
    pub fn set_device_operating_systems(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_device_operating_systems(input);
        self
    }
    /// Appends an item to `NotDeviceOperatingSystems`.
    ///
    /// To override the contents of this collection use [`set_not_device_operating_systems`](Self::set_not_device_operating_systems).
    ///
    /// <p>Device operating systems that the rule <b>will not</b> match. All other device operating systems will match.</p>
    pub fn not_device_operating_systems(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.not_device_operating_systems(input.into());
        self
    }
    /// <p>Device operating systems that the rule <b>will not</b> match. All other device operating systems will match.</p>
    pub fn set_not_device_operating_systems(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_not_device_operating_systems(input);
        self
    }
    /// Appends an item to `DeviceUserAgents`.
    ///
    /// To override the contents of this collection use [`set_device_user_agents`](Self::set_device_user_agents).
    ///
    /// <p>Device user agents that the rule will match.</p>
    pub fn device_user_agents(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.device_user_agents(input.into());
        self
    }
    /// <p>Device user agents that the rule will match.</p>
    pub fn set_device_user_agents(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_device_user_agents(input);
        self
    }
    /// Appends an item to `NotDeviceUserAgents`.
    ///
    /// To override the contents of this collection use [`set_not_device_user_agents`](Self::set_not_device_user_agents).
    ///
    /// <p>Device user agents that the rule <b>will not</b> match. All other device user agents will match.</p>
    pub fn not_device_user_agents(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.not_device_user_agents(input.into());
        self
    }
    /// <p>Device user agents that the rule <b>will not</b> match. All other device user agents will match.</p>
    pub fn set_not_device_user_agents(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_not_device_user_agents(input);
        self
    }
}
