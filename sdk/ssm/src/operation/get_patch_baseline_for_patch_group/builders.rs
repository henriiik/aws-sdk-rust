// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_patch_baseline_for_patch_group::_get_patch_baseline_for_patch_group_output::GetPatchBaselineForPatchGroupOutputBuilder;

pub use crate::operation::get_patch_baseline_for_patch_group::_get_patch_baseline_for_patch_group_input::GetPatchBaselineForPatchGroupInputBuilder;

/// Fluent builder constructing a request to `GetPatchBaselineForPatchGroup`.
///
/// <p>Retrieves the patch baseline that should be used for the specified patch group.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetPatchBaselineForPatchGroupFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_patch_baseline_for_patch_group::builders::GetPatchBaselineForPatchGroupInputBuilder
            }
impl GetPatchBaselineForPatchGroupFluentBuilder {
    /// Creates a new `GetPatchBaselineForPatchGroup`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::get_patch_baseline_for_patch_group::GetPatchBaselineForPatchGroup, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::get_patch_baseline_for_patch_group::GetPatchBaselineForPatchGroupError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::get_patch_baseline_for_patch_group::GetPatchBaselineForPatchGroupOutput, aws_smithy_http::result::SdkError<crate::operation::get_patch_baseline_for_patch_group::GetPatchBaselineForPatchGroupError>>
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
    /// <p>The name of the patch group whose patch baseline should be retrieved.</p>
    pub fn patch_group(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.patch_group(input.into());
        self
    }
    /// <p>The name of the patch group whose patch baseline should be retrieved.</p>
    pub fn set_patch_group(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_patch_group(input);
        self
    }
    /// <p>Returns the operating system rule specified for patch groups using the patch baseline.</p>
    pub fn operating_system(mut self, input: crate::types::OperatingSystem) -> Self {
        self.inner = self.inner.operating_system(input);
        self
    }
    /// <p>Returns the operating system rule specified for patch groups using the patch baseline.</p>
    pub fn set_operating_system(
        mut self,
        input: std::option::Option<crate::types::OperatingSystem>,
    ) -> Self {
        self.inner = self.inner.set_operating_system(input);
        self
    }
}
