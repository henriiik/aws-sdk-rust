// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_firewall_rule::_delete_firewall_rule_output::DeleteFirewallRuleOutputBuilder;

pub use crate::operation::delete_firewall_rule::_delete_firewall_rule_input::DeleteFirewallRuleInputBuilder;

/// Fluent builder constructing a request to `DeleteFirewallRule`.
///
/// <p>Deletes the specified firewall rule.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFirewallRuleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_firewall_rule::builders::DeleteFirewallRuleInputBuilder,
}
impl DeleteFirewallRuleFluentBuilder {
    /// Creates a new `DeleteFirewallRule`.
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
            crate::operation::delete_firewall_rule::DeleteFirewallRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_firewall_rule::DeleteFirewallRuleError,
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
        crate::operation::delete_firewall_rule::DeleteFirewallRuleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_firewall_rule::DeleteFirewallRuleError,
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
    /// <p>The unique identifier of the firewall rule group that you want to delete the rule from. </p>
    pub fn firewall_rule_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.firewall_rule_group_id(input.into());
        self
    }
    /// <p>The unique identifier of the firewall rule group that you want to delete the rule from. </p>
    pub fn set_firewall_rule_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_rule_group_id(input);
        self
    }
    /// <p>The ID of the domain list that's used in the rule. </p>
    pub fn firewall_domain_list_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.firewall_domain_list_id(input.into());
        self
    }
    /// <p>The ID of the domain list that's used in the rule. </p>
    pub fn set_firewall_domain_list_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_firewall_domain_list_id(input);
        self
    }
}
