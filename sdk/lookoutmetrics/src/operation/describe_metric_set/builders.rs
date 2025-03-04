// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_metric_set::_describe_metric_set_output::DescribeMetricSetOutputBuilder;

pub use crate::operation::describe_metric_set::_describe_metric_set_input::DescribeMetricSetInputBuilder;

/// Fluent builder constructing a request to `DescribeMetricSet`.
///
/// <p>Describes a dataset.</p>
/// <p>Amazon Lookout for Metrics API actions are eventually consistent. If you do a read operation on a resource immediately after creating or modifying it, use retries to allow time for the write operation to complete.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeMetricSetFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_metric_set::builders::DescribeMetricSetInputBuilder,
}
impl DescribeMetricSetFluentBuilder {
    /// Creates a new `DescribeMetricSet`.
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
            crate::operation::describe_metric_set::DescribeMetricSet,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_set::DescribeMetricSetError,
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
        crate::operation::describe_metric_set::DescribeMetricSetOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_metric_set::DescribeMetricSetError,
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
    /// <p>The ARN of the dataset.</p>
    pub fn metric_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.metric_set_arn(input.into());
        self
    }
    /// <p>The ARN of the dataset.</p>
    pub fn set_metric_set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_metric_set_arn(input);
        self
    }
}
