// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_ip_restriction::_describe_ip_restriction_output::DescribeIpRestrictionOutputBuilder;

pub use crate::operation::describe_ip_restriction::_describe_ip_restriction_input::DescribeIpRestrictionInputBuilder;

/// Fluent builder constructing a request to `DescribeIpRestriction`.
///
/// <p>Provides a summary and status of IP rules.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeIpRestrictionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_ip_restriction::builders::DescribeIpRestrictionInputBuilder,
}
impl DescribeIpRestrictionFluentBuilder {
    /// Creates a new `DescribeIpRestriction`.
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
            crate::operation::describe_ip_restriction::DescribeIpRestriction,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_ip_restriction::DescribeIpRestrictionError,
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
        crate::operation::describe_ip_restriction::DescribeIpRestrictionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_ip_restriction::DescribeIpRestrictionError,
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
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the IP rules.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
}
