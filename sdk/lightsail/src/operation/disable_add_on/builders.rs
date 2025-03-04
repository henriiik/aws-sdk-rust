// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_add_on::_disable_add_on_output::DisableAddOnOutputBuilder;

pub use crate::operation::disable_add_on::_disable_add_on_input::DisableAddOnInputBuilder;

/// Fluent builder constructing a request to `DisableAddOn`.
///
/// <p>Disables an add-on for an Amazon Lightsail resource. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-configuring-automatic-snapshots">Amazon Lightsail Developer Guide</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableAddOnFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_add_on::builders::DisableAddOnInputBuilder,
}
impl DisableAddOnFluentBuilder {
    /// Creates a new `DisableAddOn`.
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
            crate::operation::disable_add_on::DisableAddOn,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::disable_add_on::DisableAddOnError>,
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
        crate::operation::disable_add_on::DisableAddOnOutput,
        aws_smithy_http::result::SdkError<crate::operation::disable_add_on::DisableAddOnError>,
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
    /// <p>The add-on type to disable.</p>
    pub fn add_on_type(mut self, input: crate::types::AddOnType) -> Self {
        self.inner = self.inner.add_on_type(input);
        self
    }
    /// <p>The add-on type to disable.</p>
    pub fn set_add_on_type(mut self, input: std::option::Option<crate::types::AddOnType>) -> Self {
        self.inner = self.inner.set_add_on_type(input);
        self
    }
    /// <p>The name of the source resource for which to disable the add-on.</p>
    pub fn resource_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.resource_name(input.into());
        self
    }
    /// <p>The name of the source resource for which to disable the add-on.</p>
    pub fn set_resource_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_resource_name(input);
        self
    }
}
