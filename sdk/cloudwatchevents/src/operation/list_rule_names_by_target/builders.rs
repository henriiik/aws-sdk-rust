// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_rule_names_by_target::_list_rule_names_by_target_output::ListRuleNamesByTargetOutputBuilder;

pub use crate::operation::list_rule_names_by_target::_list_rule_names_by_target_input::ListRuleNamesByTargetInputBuilder;

/// Fluent builder constructing a request to `ListRuleNamesByTarget`.
///
/// <p>Lists the rules for the specified target. You can see which of the rules in Amazon EventBridge can invoke a specific target in your account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRuleNamesByTargetFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_rule_names_by_target::builders::ListRuleNamesByTargetInputBuilder,
}
impl ListRuleNamesByTargetFluentBuilder {
    /// Creates a new `ListRuleNamesByTarget`.
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
            crate::operation::list_rule_names_by_target::ListRuleNamesByTarget,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_rule_names_by_target::ListRuleNamesByTargetError,
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
        crate::operation::list_rule_names_by_target::ListRuleNamesByTargetOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_rule_names_by_target::ListRuleNamesByTargetError,
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
    /// <p>The Amazon Resource Name (ARN) of the target resource.</p>
    pub fn target_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the target resource.</p>
    pub fn set_target_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The name or ARN of the event bus to list rules for. If you omit this, the default event bus is used.</p>
    pub fn event_bus_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.event_bus_name(input.into());
        self
    }
    /// <p>The name or ARN of the event bus to list rules for. If you omit this, the default event bus is used.</p>
    pub fn set_event_bus_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_event_bus_name(input);
        self
    }
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned by a previous call to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of results to return.</p>
    pub fn set_limit(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
}
