// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_curated_environment_images::_list_curated_environment_images_output::ListCuratedEnvironmentImagesOutputBuilder;

pub use crate::operation::list_curated_environment_images::_list_curated_environment_images_input::ListCuratedEnvironmentImagesInputBuilder;

/// Fluent builder constructing a request to `ListCuratedEnvironmentImages`.
///
/// <p>Gets information about Docker images that are managed by CodeBuild.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListCuratedEnvironmentImagesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_curated_environment_images::builders::ListCuratedEnvironmentImagesInputBuilder
            }
impl ListCuratedEnvironmentImagesFluentBuilder {
    /// Creates a new `ListCuratedEnvironmentImages`.
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
            crate::operation::list_curated_environment_images::ListCuratedEnvironmentImages,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_curated_environment_images::ListCuratedEnvironmentImagesError,
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
        crate::operation::list_curated_environment_images::ListCuratedEnvironmentImagesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_curated_environment_images::ListCuratedEnvironmentImagesError,
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
}
