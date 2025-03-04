// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_vector_enrichment_job::_start_vector_enrichment_job_output::StartVectorEnrichmentJobOutputBuilder;

pub use crate::operation::start_vector_enrichment_job::_start_vector_enrichment_job_input::StartVectorEnrichmentJobInputBuilder;

/// Fluent builder constructing a request to `StartVectorEnrichmentJob`.
///
/// <p>Creates a Vector Enrichment job for the supplied job type. Currently, there are two supported job types: reverse geocoding and map matching.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartVectorEnrichmentJobFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::start_vector_enrichment_job::builders::StartVectorEnrichmentJobInputBuilder
            }
impl StartVectorEnrichmentJobFluentBuilder {
    /// Creates a new `StartVectorEnrichmentJob`.
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
            crate::operation::start_vector_enrichment_job::StartVectorEnrichmentJob,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::start_vector_enrichment_job::StartVectorEnrichmentJobError,
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
        crate::operation::start_vector_enrichment_job::StartVectorEnrichmentJobOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::start_vector_enrichment_job::StartVectorEnrichmentJobError,
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
    /// <p>The name of the Vector Enrichment job.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the Vector Enrichment job.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique token that guarantees that the call to this API is idempotent.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The Key Management Service key ID for server-side encryption.</p>
    pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The Key Management Service key ID for server-side encryption.</p>
    pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>Input configuration information for the Vector Enrichment job.</p>
    pub fn input_config(mut self, input: crate::types::VectorEnrichmentJobInputConfig) -> Self {
        self.inner = self.inner.input_config(input);
        self
    }
    /// <p>Input configuration information for the Vector Enrichment job.</p>
    pub fn set_input_config(
        mut self,
        input: std::option::Option<crate::types::VectorEnrichmentJobInputConfig>,
    ) -> Self {
        self.inner = self.inner.set_input_config(input);
        self
    }
    /// <p>An object containing information about the job configuration.</p>
    pub fn job_config(mut self, input: crate::types::VectorEnrichmentJobConfig) -> Self {
        self.inner = self.inner.job_config(input);
        self
    }
    /// <p>An object containing information about the job configuration.</p>
    pub fn set_job_config(
        mut self,
        input: std::option::Option<crate::types::VectorEnrichmentJobConfig>,
    ) -> Self {
        self.inner = self.inner.set_job_config(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    pub fn execution_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that you specified for the job.</p>
    pub fn set_execution_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Each tag consists of a key and a value.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Each tag consists of a key and a value.</p>
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
