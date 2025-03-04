// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_refresh_schedule::_describe_refresh_schedule_output::DescribeRefreshScheduleOutputBuilder;

pub use crate::operation::describe_refresh_schedule::_describe_refresh_schedule_input::DescribeRefreshScheduleInputBuilder;

/// Fluent builder constructing a request to `DescribeRefreshSchedule`.
///
/// <p>Provides a summary of a refresh schedule.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeRefreshScheduleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::describe_refresh_schedule::builders::DescribeRefreshScheduleInputBuilder,
}
impl DescribeRefreshScheduleFluentBuilder {
    /// Creates a new `DescribeRefreshSchedule`.
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
            crate::operation::describe_refresh_schedule::DescribeRefreshSchedule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_refresh_schedule::DescribeRefreshScheduleError,
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
        crate::operation::describe_refresh_schedule::DescribeRefreshScheduleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_refresh_schedule::DescribeRefreshScheduleError,
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
    /// <p>The Amazon Web Services account ID.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the dataset.</p>
    pub fn data_set_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.data_set_id(input.into());
        self
    }
    /// <p>The ID of the dataset.</p>
    pub fn set_data_set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_data_set_id(input);
        self
    }
    /// <p>The ID of the refresh schedule.</p>
    pub fn schedule_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.schedule_id(input.into());
        self
    }
    /// <p>The ID of the refresh schedule.</p>
    pub fn set_schedule_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_schedule_id(input);
        self
    }
}
