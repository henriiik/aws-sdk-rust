// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_exclusions::_describe_exclusions_output::DescribeExclusionsOutputBuilder;

pub use crate::operation::describe_exclusions::_describe_exclusions_input::DescribeExclusionsInputBuilder;

/// Fluent builder constructing a request to `DescribeExclusions`.
///
/// <p>Describes the exclusions that are specified by the exclusions' ARNs.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeExclusionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_exclusions::builders::DescribeExclusionsInputBuilder,
}
impl DescribeExclusionsFluentBuilder {
    /// Creates a new `DescribeExclusions`.
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
            crate::operation::describe_exclusions::DescribeExclusions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_exclusions::DescribeExclusionsError,
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
        crate::operation::describe_exclusions::DescribeExclusionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_exclusions::DescribeExclusionsError,
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
    /// Appends an item to `exclusionArns`.
    ///
    /// To override the contents of this collection use [`set_exclusion_arns`](Self::set_exclusion_arns).
    ///
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    pub fn exclusion_arns(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.exclusion_arns(input.into());
        self
    }
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    pub fn set_exclusion_arns(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_exclusion_arns(input);
        self
    }
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    pub fn locale(mut self, input: crate::types::Locale) -> Self {
        self.inner = self.inner.locale(input);
        self
    }
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    pub fn set_locale(mut self, input: std::option::Option<crate::types::Locale>) -> Self {
        self.inner = self.inner.set_locale(input);
        self
    }
}
