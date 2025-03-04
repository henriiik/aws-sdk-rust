// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_inference_experiment::_update_inference_experiment_output::UpdateInferenceExperimentOutputBuilder;

pub use crate::operation::update_inference_experiment::_update_inference_experiment_input::UpdateInferenceExperimentInputBuilder;

/// Fluent builder constructing a request to `UpdateInferenceExperiment`.
///
/// <p> Updates an inference experiment that you created. The status of the inference experiment has to be either <code>Created</code>, <code>Running</code>. For more information on the status of an inference experiment, see <code>DescribeInferenceExperimentResponse$Status</code>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateInferenceExperimentFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_inference_experiment::builders::UpdateInferenceExperimentInputBuilder
            }
impl UpdateInferenceExperimentFluentBuilder {
    /// Creates a new `UpdateInferenceExperiment`.
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
            crate::operation::update_inference_experiment::UpdateInferenceExperiment,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_inference_experiment::UpdateInferenceExperimentError,
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
        crate::operation::update_inference_experiment::UpdateInferenceExperimentOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_inference_experiment::UpdateInferenceExperimentError,
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
    /// <p>The name of the inference experiment to be updated.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the inference experiment to be updated.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p> The duration for which the inference experiment will run. If the status of the inference experiment is <code>Created</code>, then you can update both the start and end dates. If the status of the inference experiment is <code>Running</code>, then you can update only the end date. </p>
    pub fn schedule(mut self, input: crate::types::InferenceExperimentSchedule) -> Self {
        self.inner = self.inner.schedule(input);
        self
    }
    /// <p> The duration for which the inference experiment will run. If the status of the inference experiment is <code>Created</code>, then you can update both the start and end dates. If the status of the inference experiment is <code>Running</code>, then you can update only the end date. </p>
    pub fn set_schedule(
        mut self,
        input: std::option::Option<crate::types::InferenceExperimentSchedule>,
    ) -> Self {
        self.inner = self.inner.set_schedule(input);
        self
    }
    /// <p>The description of the inference experiment.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the inference experiment.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `ModelVariants`.
    ///
    /// To override the contents of this collection use [`set_model_variants`](Self::set_model_variants).
    ///
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant, whose infrastructure configuration you want to update. </p>
    pub fn model_variants(mut self, input: crate::types::ModelVariantConfig) -> Self {
        self.inner = self.inner.model_variants(input);
        self
    }
    /// <p> An array of <code>ModelVariantConfig</code> objects. There is one for each variant, whose infrastructure configuration you want to update. </p>
    pub fn set_model_variants(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ModelVariantConfig>>,
    ) -> Self {
        self.inner = self.inner.set_model_variants(input);
        self
    }
    /// <p>The Amazon S3 location and configuration for storing inference request and response data.</p>
    pub fn data_storage_config(
        mut self,
        input: crate::types::InferenceExperimentDataStorageConfig,
    ) -> Self {
        self.inner = self.inner.data_storage_config(input);
        self
    }
    /// <p>The Amazon S3 location and configuration for storing inference request and response data.</p>
    pub fn set_data_storage_config(
        mut self,
        input: std::option::Option<crate::types::InferenceExperimentDataStorageConfig>,
    ) -> Self {
        self.inner = self.inner.set_data_storage_config(input);
        self
    }
    /// <p> The configuration of <code>ShadowMode</code> inference experiment type. Use this field to specify a production variant which takes all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant also specify the percentage of requests that Amazon SageMaker replicates. </p>
    pub fn shadow_mode_config(mut self, input: crate::types::ShadowModeConfig) -> Self {
        self.inner = self.inner.shadow_mode_config(input);
        self
    }
    /// <p> The configuration of <code>ShadowMode</code> inference experiment type. Use this field to specify a production variant which takes all the inference requests, and a shadow variant to which Amazon SageMaker replicates a percentage of the inference requests. For the shadow variant also specify the percentage of requests that Amazon SageMaker replicates. </p>
    pub fn set_shadow_mode_config(
        mut self,
        input: std::option::Option<crate::types::ShadowModeConfig>,
    ) -> Self {
        self.inner = self.inner.set_shadow_mode_config(input);
        self
    }
}
