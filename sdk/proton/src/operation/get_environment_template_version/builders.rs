// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_environment_template_version::_get_environment_template_version_output::GetEnvironmentTemplateVersionOutputBuilder;

pub use crate::operation::get_environment_template_version::_get_environment_template_version_input::GetEnvironmentTemplateVersionInputBuilder;

/// Fluent builder constructing a request to `GetEnvironmentTemplateVersion`.
///
/// <p>Get detailed data for a major or minor version of an environment template.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetEnvironmentTemplateVersionFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_environment_template_version::builders::GetEnvironmentTemplateVersionInputBuilder
            }
impl GetEnvironmentTemplateVersionFluentBuilder {
    /// Creates a new `GetEnvironmentTemplateVersion`.
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
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
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
        crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
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
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn template_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn set_template_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn major_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.major_version(input.into());
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn set_major_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_major_version(input);
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn minor_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.minor_version(input.into());
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn set_minor_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_minor_version(input);
        self
    }
}
