// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_topics_detection_job::_describe_topics_detection_job_output::DescribeTopicsDetectionJobOutputBuilder;

pub use crate::operation::describe_topics_detection_job::_describe_topics_detection_job_input::DescribeTopicsDetectionJobInputBuilder;

/// Fluent builder constructing a request to `DescribeTopicsDetectionJob`.
///
/// <p>Gets the properties associated with a topic detection job. Use this operation to get the status of a detection job.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTopicsDetectionJobFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_topics_detection_job::builders::DescribeTopicsDetectionJobInputBuilder
            }
impl DescribeTopicsDetectionJobFluentBuilder {
    /// Creates a new `DescribeTopicsDetectionJob`.
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
            crate::operation::describe_topics_detection_job::DescribeTopicsDetectionJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_topics_detection_job::DescribeTopicsDetectionJobError,
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
        crate::operation::describe_topics_detection_job::DescribeTopicsDetectionJobOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_topics_detection_job::DescribeTopicsDetectionJobError,
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
    /// <p>The identifier assigned by the user to the detection job.</p>
    pub fn job_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The identifier assigned by the user to the detection job.</p>
    pub fn set_job_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
}
