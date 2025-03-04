// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_job_definition::_deregister_job_definition_output::DeregisterJobDefinitionOutputBuilder;

pub use crate::operation::deregister_job_definition::_deregister_job_definition_input::DeregisterJobDefinitionInputBuilder;

/// Fluent builder constructing a request to `DeregisterJobDefinition`.
///
/// <p>Deregisters an Batch job definition. Job definitions are permanently deleted after 180 days.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterJobDefinitionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::deregister_job_definition::builders::DeregisterJobDefinitionInputBuilder,
}
impl DeregisterJobDefinitionFluentBuilder {
    /// Creates a new `DeregisterJobDefinition`.
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
            crate::operation::deregister_job_definition::DeregisterJobDefinition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::deregister_job_definition::DeregisterJobDefinitionError,
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
        crate::operation::deregister_job_definition::DeregisterJobDefinitionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::deregister_job_definition::DeregisterJobDefinitionError,
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
    /// <p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister.</p>
    pub fn job_definition(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_definition(input.into());
        self
    }
    /// <p>The name and revision (<code>name:revision</code>) or full Amazon Resource Name (ARN) of the job definition to deregister.</p>
    pub fn set_job_definition(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_job_definition(input);
        self
    }
}
