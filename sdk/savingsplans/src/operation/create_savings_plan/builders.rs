// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_savings_plan::_create_savings_plan_output::CreateSavingsPlanOutputBuilder;

pub use crate::operation::create_savings_plan::_create_savings_plan_input::CreateSavingsPlanInputBuilder;

/// Fluent builder constructing a request to `CreateSavingsPlan`.
///
/// <p>Creates a Savings Plan.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateSavingsPlanFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_savings_plan::builders::CreateSavingsPlanInputBuilder,
}
impl CreateSavingsPlanFluentBuilder {
    /// Creates a new `CreateSavingsPlan`.
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
            crate::operation::create_savings_plan::CreateSavingsPlan,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_savings_plan::CreateSavingsPlanError,
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
        crate::operation::create_savings_plan::CreateSavingsPlanOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_savings_plan::CreateSavingsPlanError,
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
    /// <p>The ID of the offering.</p>
    pub fn savings_plan_offering_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.savings_plan_offering_id(input.into());
        self
    }
    /// <p>The ID of the offering.</p>
    pub fn set_savings_plan_offering_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_savings_plan_offering_id(input);
        self
    }
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    pub fn commitment(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.commitment(input.into());
        self
    }
    /// <p>The hourly commitment, in USD. This is a value between 0.001 and 1 million. You cannot specify more than five digits after the decimal point.</p>
    pub fn set_commitment(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_commitment(input);
        self
    }
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    pub fn upfront_payment_amount(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.upfront_payment_amount(input.into());
        self
    }
    /// <p>The up-front payment amount. This is a whole number between 50 and 99 percent of the total value of the Savings Plan. This parameter is supported only if the payment option is <code>Partial Upfront</code>.</p>
    pub fn set_upfront_payment_amount(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_upfront_payment_amount(input);
        self
    }
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    pub fn purchase_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.purchase_time(input);
        self
    }
    /// <p>The time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ).</p>
    pub fn set_purchase_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_purchase_time(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tags.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>One or more tags.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
