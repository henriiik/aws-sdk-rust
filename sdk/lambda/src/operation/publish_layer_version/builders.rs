// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::publish_layer_version::_publish_layer_version_output::PublishLayerVersionOutputBuilder;

pub use crate::operation::publish_layer_version::_publish_layer_version_input::PublishLayerVersionInputBuilder;

/// Fluent builder constructing a request to `PublishLayerVersion`.
///
/// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">Lambda layer</a> from a ZIP archive. Each time you call <code>PublishLayerVersion</code> with the same layer name, a new version is created.</p>
/// <p>Add layers to your function with <code>CreateFunction</code> or <code>UpdateFunctionConfiguration</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PublishLayerVersionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::publish_layer_version::builders::PublishLayerVersionInputBuilder,
}
impl PublishLayerVersionFluentBuilder {
    /// Creates a new `PublishLayerVersion`.
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
            crate::operation::publish_layer_version::PublishLayerVersion,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::publish_layer_version::PublishLayerVersionError,
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
        crate::operation::publish_layer_version::PublishLayerVersionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::publish_layer_version::PublishLayerVersionError,
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
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn layer_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.layer_name(input.into());
        self
    }
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    pub fn set_layer_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_layer_name(input);
        self
    }
    /// <p>The description of the version.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the version.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The function layer archive.</p>
    pub fn content(mut self, input: crate::types::LayerVersionContentInput) -> Self {
        self.inner = self.inner.content(input);
        self
    }
    /// <p>The function layer archive.</p>
    pub fn set_content(
        mut self,
        input: std::option::Option<crate::types::LayerVersionContentInput>,
    ) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// Appends an item to `CompatibleRuntimes`.
    ///
    /// To override the contents of this collection use [`set_compatible_runtimes`](Self::set_compatible_runtimes).
    ///
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    pub fn compatible_runtimes(mut self, input: crate::types::Runtime) -> Self {
        self.inner = self.inner.compatible_runtimes(input);
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <code>ListLayers</code> and <code>ListLayerVersions</code>.</p>
    pub fn set_compatible_runtimes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Runtime>>,
    ) -> Self {
        self.inner = self.inner.set_compatible_runtimes(input);
        self
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li> <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p> </li>
    /// <li> <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p> </li>
    /// <li> <p>The full text of the license.</p> </li>
    /// </ul>
    pub fn license_info(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.license_info(input.into());
        self
    }
    /// <p>The layer's software license. It can be any of the following:</p>
    /// <ul>
    /// <li> <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p> </li>
    /// <li> <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p> </li>
    /// <li> <p>The full text of the license.</p> </li>
    /// </ul>
    pub fn set_license_info(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_license_info(input);
        self
    }
    /// Appends an item to `CompatibleArchitectures`.
    ///
    /// To override the contents of this collection use [`set_compatible_architectures`](Self::set_compatible_architectures).
    ///
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub fn compatible_architectures(mut self, input: crate::types::Architecture) -> Self {
        self.inner = self.inner.compatible_architectures(input);
        self
    }
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/foundation-arch.html">instruction set architectures</a>.</p>
    pub fn set_compatible_architectures(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Architecture>>,
    ) -> Self {
        self.inner = self.inner.set_compatible_architectures(input);
        self
    }
}
