// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_alarm_model_versions::_list_alarm_model_versions_output::ListAlarmModelVersionsOutputBuilder;

pub use crate::operation::list_alarm_model_versions::_list_alarm_model_versions_input::ListAlarmModelVersionsInputBuilder;

/// Fluent builder constructing a request to `ListAlarmModelVersions`.
///
/// <p>Lists all the versions of an alarm model. The operation returns only the metadata associated with each alarm model version.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListAlarmModelVersionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::list_alarm_model_versions::builders::ListAlarmModelVersionsInputBuilder,
}
impl ListAlarmModelVersionsFluentBuilder {
    /// Creates a new `ListAlarmModelVersions`.
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
            crate::operation::list_alarm_model_versions::ListAlarmModelVersions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_alarm_model_versions::ListAlarmModelVersionsError,
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
        crate::operation::list_alarm_model_versions::ListAlarmModelVersionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_alarm_model_versions::ListAlarmModelVersionsError,
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
    /// <p>The name of the alarm model.</p>
    pub fn alarm_model_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.alarm_model_name(input.into());
        self
    }
    /// <p>The name of the alarm model.</p>
    pub fn set_alarm_model_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_alarm_model_name(input);
        self
    }
    /// <p>The token that you can use to return the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token that you can use to return the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to be returned per request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to be returned per request.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
