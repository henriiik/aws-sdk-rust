// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_rule::_enable_rule_output::EnableRuleOutputBuilder;

pub use crate::operation::enable_rule::_enable_rule_input::EnableRuleInputBuilder;

/// Fluent builder constructing a request to `EnableRule`.
///
/// <p>Enables the specified rule. If the rule does not exist, the operation fails.</p>
/// <p>When you enable a rule, incoming events might not immediately start matching to a newly enabled rule. Allow a short period of time for changes to take effect.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct EnableRuleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_rule::builders::EnableRuleInputBuilder,
}
impl EnableRuleFluentBuilder {
    /// Creates a new `EnableRule`.
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
            crate::operation::enable_rule::EnableRule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::enable_rule::EnableRuleError>,
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
        crate::operation::enable_rule::EnableRuleOutput,
        aws_smithy_http::result::SdkError<crate::operation::enable_rule::EnableRuleError>,
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
    /// <p>The name of the rule.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the rule.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    pub fn event_bus_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.event_bus_name(input.into());
        self
    }
    /// <p>The name or ARN of the event bus associated with the rule. If you omit this, the default event bus is used.</p>
    pub fn set_event_bus_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_event_bus_name(input);
        self
    }
}
