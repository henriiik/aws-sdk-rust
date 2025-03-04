// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_configuration_set_tracking_options::_update_configuration_set_tracking_options_output::UpdateConfigurationSetTrackingOptionsOutputBuilder;

pub use crate::operation::update_configuration_set_tracking_options::_update_configuration_set_tracking_options_input::UpdateConfigurationSetTrackingOptionsInputBuilder;

/// Fluent builder constructing a request to `UpdateConfigurationSetTrackingOptions`.
///
/// <p>Modifies an association between a configuration set and a custom domain for open and click event tracking. </p>
/// <p>By default, images and links used for tracking open and click events are hosted on domains operated by Amazon SES. You can configure a subdomain of your own to handle these events. For information about using custom domains, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Amazon SES Developer Guide</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConfigurationSetTrackingOptionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_configuration_set_tracking_options::builders::UpdateConfigurationSetTrackingOptionsInputBuilder
            }
impl UpdateConfigurationSetTrackingOptionsFluentBuilder {
    /// Creates a new `UpdateConfigurationSetTrackingOptions`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptions, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptionsError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptionsOutput, aws_smithy_http::result::SdkError<crate::operation::update_configuration_set_tracking_options::UpdateConfigurationSetTrackingOptionsError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// <p>The name of the configuration set for which you want to update the custom tracking domain.</p>
    pub fn configuration_set_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.configuration_set_name(input.into());
        self
    }
    /// <p>The name of the configuration set for which you want to update the custom tracking domain.</p>
    pub fn set_configuration_set_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_configuration_set_name(input);
        self
    }
    /// <p>A domain that is used to redirect email recipients to an Amazon SES-operated domain. This domain captures open and click events generated by Amazon SES emails.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <i>Amazon SES Developer Guide</i>.</p>
    pub fn tracking_options(mut self, input: crate::types::TrackingOptions) -> Self {
        self.inner = self.inner.tracking_options(input);
        self
    }
    /// <p>A domain that is used to redirect email recipients to an Amazon SES-operated domain. This domain captures open and click events generated by Amazon SES emails.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/configure-custom-open-click-domains.html">Configuring Custom Domains to Handle Open and Click Tracking</a> in the <i>Amazon SES Developer Guide</i>.</p>
    pub fn set_tracking_options(
        mut self,
        input: std::option::Option<crate::types::TrackingOptions>,
    ) -> Self {
        self.inner = self.inner.set_tracking_options(input);
        self
    }
}
