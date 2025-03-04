// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_crawler_schedule::_update_crawler_schedule_output::UpdateCrawlerScheduleOutputBuilder;

pub use crate::operation::update_crawler_schedule::_update_crawler_schedule_input::UpdateCrawlerScheduleInputBuilder;

/// Fluent builder constructing a request to `UpdateCrawlerSchedule`.
///
/// <p>Updates the schedule of a crawler using a <code>cron</code> expression. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateCrawlerScheduleFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_crawler_schedule::builders::UpdateCrawlerScheduleInputBuilder,
}
impl UpdateCrawlerScheduleFluentBuilder {
    /// Creates a new `UpdateCrawlerSchedule`.
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
            crate::operation::update_crawler_schedule::UpdateCrawlerSchedule,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_crawler_schedule::UpdateCrawlerScheduleError,
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
        crate::operation::update_crawler_schedule::UpdateCrawlerScheduleOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_crawler_schedule::UpdateCrawlerScheduleError,
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
    /// <p>The name of the crawler whose schedule to update.</p>
    pub fn crawler_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.crawler_name(input.into());
        self
    }
    /// <p>The name of the crawler whose schedule to update.</p>
    pub fn set_crawler_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_crawler_name(input);
        self
    }
    /// <p>The updated <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    pub fn schedule(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.schedule(input.into());
        self
    }
    /// <p>The updated <code>cron</code> expression used to specify the schedule (see <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-data-warehouse-schedule.html">Time-Based Schedules for Jobs and Crawlers</a>. For example, to run something every day at 12:15 UTC, you would specify: <code>cron(15 12 * * ? *)</code>.</p>
    pub fn set_schedule(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_schedule(input);
        self
    }
}
