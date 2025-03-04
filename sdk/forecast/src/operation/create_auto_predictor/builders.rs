// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_auto_predictor::_create_auto_predictor_output::CreateAutoPredictorOutputBuilder;

pub use crate::operation::create_auto_predictor::_create_auto_predictor_input::CreateAutoPredictorInputBuilder;

/// Fluent builder constructing a request to `CreateAutoPredictor`.
///
/// <p>Creates an Amazon Forecast predictor.</p>
/// <p>Amazon Forecast creates predictors with AutoPredictor, which involves applying the optimal combination of algorithms to each time series in your datasets. You can use <code>CreateAutoPredictor</code> to create new predictors or upgrade/retrain existing predictors.</p>
/// <p> <b>Creating new predictors</b> </p>
/// <p>The following parameters are required when creating a new predictor:</p>
/// <ul>
/// <li> <p> <code>PredictorName</code> - A unique name for the predictor.</p> </li>
/// <li> <p> <code>DatasetGroupArn</code> - The ARN of the dataset group used to train the predictor.</p> </li>
/// <li> <p> <code>ForecastFrequency</code> - The granularity of your forecasts (hourly, daily, weekly, etc).</p> </li>
/// <li> <p> <code>ForecastHorizon</code> - The number of time-steps that the model predicts. The forecast horizon is also called the prediction length.</p> </li>
/// </ul>
/// <p>When creating a new predictor, do not specify a value for <code>ReferencePredictorArn</code>.</p>
/// <p> <b>Upgrading and retraining predictors</b> </p>
/// <p>The following parameters are required when retraining or upgrading a predictor:</p>
/// <ul>
/// <li> <p> <code>PredictorName</code> - A unique name for the predictor.</p> </li>
/// <li> <p> <code>ReferencePredictorArn</code> - The ARN of the predictor to retrain or upgrade.</p> </li>
/// </ul>
/// <p>When upgrading or retraining a predictor, only specify values for the <code>ReferencePredictorArn</code> and <code>PredictorName</code>. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateAutoPredictorFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_auto_predictor::builders::CreateAutoPredictorInputBuilder,
}
impl CreateAutoPredictorFluentBuilder {
    /// Creates a new `CreateAutoPredictor`.
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
            crate::operation::create_auto_predictor::CreateAutoPredictor,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_auto_predictor::CreateAutoPredictorError,
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
        crate::operation::create_auto_predictor::CreateAutoPredictorOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_auto_predictor::CreateAutoPredictorError,
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
    /// <p>A unique name for the predictor</p>
    pub fn predictor_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.predictor_name(input.into());
        self
    }
    /// <p>A unique name for the predictor</p>
    pub fn set_predictor_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_predictor_name(input);
        self
    }
    /// <p>The number of time-steps that the model predicts. The forecast horizon is also called the prediction length.</p>
    /// <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/4 of the TARGET_TIME_SERIES dataset length. If you are retraining an existing AutoPredictor, then the maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the TARGET_TIME_SERIES dataset length.</p>
    /// <p>If you are upgrading to an AutoPredictor or retraining an existing AutoPredictor, you cannot update the forecast horizon parameter. You can meet this requirement by providing longer time-series in the dataset.</p>
    pub fn forecast_horizon(mut self, input: i32) -> Self {
        self.inner = self.inner.forecast_horizon(input);
        self
    }
    /// <p>The number of time-steps that the model predicts. The forecast horizon is also called the prediction length.</p>
    /// <p>The maximum forecast horizon is the lesser of 500 time-steps or 1/4 of the TARGET_TIME_SERIES dataset length. If you are retraining an existing AutoPredictor, then the maximum forecast horizon is the lesser of 500 time-steps or 1/3 of the TARGET_TIME_SERIES dataset length.</p>
    /// <p>If you are upgrading to an AutoPredictor or retraining an existing AutoPredictor, you cannot update the forecast horizon parameter. You can meet this requirement by providing longer time-series in the dataset.</p>
    pub fn set_forecast_horizon(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_forecast_horizon(input);
        self
    }
    /// Appends an item to `ForecastTypes`.
    ///
    /// To override the contents of this collection use [`set_forecast_types`](Self::set_forecast_types).
    ///
    /// <p>The forecast types used to train a predictor. You can specify up to five forecast types. Forecast types can be quantiles from 0.01 to 0.99, by increments of 0.01 or higher. You can also specify the mean forecast with <code>mean</code>.</p>
    pub fn forecast_types(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.forecast_types(input.into());
        self
    }
    /// <p>The forecast types used to train a predictor. You can specify up to five forecast types. Forecast types can be quantiles from 0.01 to 0.99, by increments of 0.01 or higher. You can also specify the mean forecast with <code>mean</code>.</p>
    pub fn set_forecast_types(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_forecast_types(input);
        self
    }
    /// Appends an item to `ForecastDimensions`.
    ///
    /// To override the contents of this collection use [`set_forecast_dimensions`](Self::set_forecast_dimensions).
    ///
    /// <p>An array of dimension (field) names that specify how to group the generated forecast.</p>
    /// <p>For example, if you are generating forecasts for item sales across all your stores, and your dataset contains a <code>store_id</code> field, you would specify <code>store_id</code> as a dimension to group sales forecasts for each store.</p>
    pub fn forecast_dimensions(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.forecast_dimensions(input.into());
        self
    }
    /// <p>An array of dimension (field) names that specify how to group the generated forecast.</p>
    /// <p>For example, if you are generating forecasts for item sales across all your stores, and your dataset contains a <code>store_id</code> field, you would specify <code>store_id</code> as a dimension to group sales forecasts for each store.</p>
    pub fn set_forecast_dimensions(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_forecast_dimensions(input);
        self
    }
    /// <p>The frequency of predictions in a forecast.</p>
    /// <p>Valid intervals are an integer followed by Y (Year), M (Month), W (Week), D (Day), H (Hour), and min (Minute). For example, "1D" indicates every day and "15min" indicates every 15 minutes. You cannot specify a value that would overlap with the next larger frequency. That means, for example, you cannot specify a frequency of 60 minutes, because that is equivalent to 1 hour. The valid values for each frequency are the following:</p>
    /// <ul>
    /// <li> <p>Minute - 1-59</p> </li>
    /// <li> <p>Hour - 1-23</p> </li>
    /// <li> <p>Day - 1-6</p> </li>
    /// <li> <p>Week - 1-4</p> </li>
    /// <li> <p>Month - 1-11</p> </li>
    /// <li> <p>Year - 1</p> </li>
    /// </ul>
    /// <p>Thus, if you want every other week forecasts, specify "2W". Or, if you want quarterly forecasts, you specify "3M".</p>
    /// <p>The frequency must be greater than or equal to the TARGET_TIME_SERIES dataset frequency.</p>
    /// <p>When a RELATED_TIME_SERIES dataset is provided, the frequency must be equal to the RELATED_TIME_SERIES dataset frequency.</p>
    pub fn forecast_frequency(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.forecast_frequency(input.into());
        self
    }
    /// <p>The frequency of predictions in a forecast.</p>
    /// <p>Valid intervals are an integer followed by Y (Year), M (Month), W (Week), D (Day), H (Hour), and min (Minute). For example, "1D" indicates every day and "15min" indicates every 15 minutes. You cannot specify a value that would overlap with the next larger frequency. That means, for example, you cannot specify a frequency of 60 minutes, because that is equivalent to 1 hour. The valid values for each frequency are the following:</p>
    /// <ul>
    /// <li> <p>Minute - 1-59</p> </li>
    /// <li> <p>Hour - 1-23</p> </li>
    /// <li> <p>Day - 1-6</p> </li>
    /// <li> <p>Week - 1-4</p> </li>
    /// <li> <p>Month - 1-11</p> </li>
    /// <li> <p>Year - 1</p> </li>
    /// </ul>
    /// <p>Thus, if you want every other week forecasts, specify "2W". Or, if you want quarterly forecasts, you specify "3M".</p>
    /// <p>The frequency must be greater than or equal to the TARGET_TIME_SERIES dataset frequency.</p>
    /// <p>When a RELATED_TIME_SERIES dataset is provided, the frequency must be equal to the RELATED_TIME_SERIES dataset frequency.</p>
    pub fn set_forecast_frequency(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_forecast_frequency(input);
        self
    }
    /// <p>The data configuration for your dataset group and any additional datasets.</p>
    pub fn data_config(mut self, input: crate::types::DataConfig) -> Self {
        self.inner = self.inner.data_config(input);
        self
    }
    /// <p>The data configuration for your dataset group and any additional datasets.</p>
    pub fn set_data_config(mut self, input: std::option::Option<crate::types::DataConfig>) -> Self {
        self.inner = self.inner.set_data_config(input);
        self
    }
    /// <p>An Key Management Service (KMS) key and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key. You can specify this optional object in the <code>CreateDataset</code> and <code>CreatePredictor</code> requests.</p>
    pub fn encryption_config(mut self, input: crate::types::EncryptionConfig) -> Self {
        self.inner = self.inner.encryption_config(input);
        self
    }
    /// <p>An Key Management Service (KMS) key and an Identity and Access Management (IAM) role that Amazon Forecast can assume to access the key. You can specify this optional object in the <code>CreateDataset</code> and <code>CreatePredictor</code> requests.</p>
    pub fn set_encryption_config(
        mut self,
        input: std::option::Option<crate::types::EncryptionConfig>,
    ) -> Self {
        self.inner = self.inner.set_encryption_config(input);
        self
    }
    /// <p>The ARN of the predictor to retrain or upgrade. This parameter is only used when retraining or upgrading a predictor. When creating a new predictor, do not specify a value for this parameter.</p>
    /// <p>When upgrading or retraining a predictor, only specify values for the <code>ReferencePredictorArn</code> and <code>PredictorName</code>. The value for <code>PredictorName</code> must be a unique predictor name.</p>
    pub fn reference_predictor_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.reference_predictor_arn(input.into());
        self
    }
    /// <p>The ARN of the predictor to retrain or upgrade. This parameter is only used when retraining or upgrading a predictor. When creating a new predictor, do not specify a value for this parameter.</p>
    /// <p>When upgrading or retraining a predictor, only specify values for the <code>ReferencePredictorArn</code> and <code>PredictorName</code>. The value for <code>PredictorName</code> must be a unique predictor name.</p>
    pub fn set_reference_predictor_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_reference_predictor_arn(input);
        self
    }
    /// <p>The accuracy metric used to optimize the predictor.</p>
    pub fn optimization_metric(mut self, input: crate::types::OptimizationMetric) -> Self {
        self.inner = self.inner.optimization_metric(input);
        self
    }
    /// <p>The accuracy metric used to optimize the predictor.</p>
    pub fn set_optimization_metric(
        mut self,
        input: std::option::Option<crate::types::OptimizationMetric>,
    ) -> Self {
        self.inner = self.inner.set_optimization_metric(input);
        self
    }
    /// <p>Create an Explainability resource for the predictor.</p>
    pub fn explain_predictor(mut self, input: bool) -> Self {
        self.inner = self.inner.explain_predictor(input);
        self
    }
    /// <p>Create an Explainability resource for the predictor.</p>
    pub fn set_explain_predictor(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_explain_predictor(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Optional metadata to help you categorize and organize your predictors. Each tag consists of a key and an optional value, both of which you define. Tag keys and values are case sensitive.</p>
    /// <p>The following restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>For each resource, each tag key must be unique and each tag key must have one value.</p> </li>
    /// <li> <p>Maximum number of tags per resource: 50.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Accepted characters: all letters and numbers, spaces representable in UTF-8, and + - = . _ : / @. If your tagging schema is used across other services and resources, the character restrictions of those services also apply. </p> </li>
    /// <li> <p>Key prefixes cannot include any upper or lowercase combination of <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit. You cannot edit or delete tag keys with this prefix.</p> </li>
    /// </ul>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Optional metadata to help you categorize and organize your predictors. Each tag consists of a key and an optional value, both of which you define. Tag keys and values are case sensitive.</p>
    /// <p>The following restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>For each resource, each tag key must be unique and each tag key must have one value.</p> </li>
    /// <li> <p>Maximum number of tags per resource: 50.</p> </li>
    /// <li> <p>Maximum key length: 128 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Maximum value length: 256 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Accepted characters: all letters and numbers, spaces representable in UTF-8, and + - = . _ : / @. If your tagging schema is used across other services and resources, the character restrictions of those services also apply. </p> </li>
    /// <li> <p>Key prefixes cannot include any upper or lowercase combination of <code>aws:</code> or <code>AWS:</code>. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit. You cannot edit or delete tag keys with this prefix.</p> </li>
    /// </ul>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The configuration details for predictor monitoring. Provide a name for the monitor resource to enable predictor monitoring.</p>
    /// <p>Predictor monitoring allows you to see how your predictor's performance changes over time. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/predictor-monitoring.html">Predictor Monitoring</a>.</p>
    pub fn monitor_config(mut self, input: crate::types::MonitorConfig) -> Self {
        self.inner = self.inner.monitor_config(input);
        self
    }
    /// <p>The configuration details for predictor monitoring. Provide a name for the monitor resource to enable predictor monitoring.</p>
    /// <p>Predictor monitoring allows you to see how your predictor's performance changes over time. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/predictor-monitoring.html">Predictor Monitoring</a>.</p>
    pub fn set_monitor_config(
        mut self,
        input: std::option::Option<crate::types::MonitorConfig>,
    ) -> Self {
        self.inner = self.inner.set_monitor_config(input);
        self
    }
    /// <p>The time boundary Forecast uses to align and aggregate any data that doesn't align with your forecast frequency. Provide the unit of time and the time boundary as a key value pair. For more information on specifying a time boundary, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#specifying-time-boundary">Specifying a Time Boundary</a>. If you don't provide a time boundary, Forecast uses a set of <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#default-time-boundaries">Default Time Boundaries</a>.</p>
    pub fn time_alignment_boundary(mut self, input: crate::types::TimeAlignmentBoundary) -> Self {
        self.inner = self.inner.time_alignment_boundary(input);
        self
    }
    /// <p>The time boundary Forecast uses to align and aggregate any data that doesn't align with your forecast frequency. Provide the unit of time and the time boundary as a key value pair. For more information on specifying a time boundary, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#specifying-time-boundary">Specifying a Time Boundary</a>. If you don't provide a time boundary, Forecast uses a set of <a href="https://docs.aws.amazon.com/forecast/latest/dg/data-aggregation.html#default-time-boundaries">Default Time Boundaries</a>.</p>
    pub fn set_time_alignment_boundary(
        mut self,
        input: std::option::Option<crate::types::TimeAlignmentBoundary>,
    ) -> Self {
        self.inner = self.inner.set_time_alignment_boundary(input);
        self
    }
}
