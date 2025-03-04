// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_image_permissions::_delete_image_permissions_output::DeleteImagePermissionsOutputBuilder;

pub use crate::operation::delete_image_permissions::_delete_image_permissions_input::DeleteImagePermissionsInputBuilder;

/// Fluent builder constructing a request to `DeleteImagePermissions`.
///
/// <p>Deletes permissions for the specified private image. After you delete permissions for an image, AWS accounts to which you previously granted these permissions can no longer use the image.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteImagePermissionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_image_permissions::builders::DeleteImagePermissionsInputBuilder,
}
impl DeleteImagePermissionsFluentBuilder {
    /// Creates a new `DeleteImagePermissions`.
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
            crate::operation::delete_image_permissions::DeleteImagePermissions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_image_permissions::DeleteImagePermissionsError,
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
        crate::operation::delete_image_permissions::DeleteImagePermissionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_image_permissions::DeleteImagePermissionsError,
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
    /// <p>The name of the private image.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the private image.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The 12-digit identifier of the AWS account for which to delete image permissions.</p>
    pub fn shared_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.shared_account_id(input.into());
        self
    }
    /// <p>The 12-digit identifier of the AWS account for which to delete image permissions.</p>
    pub fn set_shared_account_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_shared_account_id(input);
        self
    }
}
