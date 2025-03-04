// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_lifecycle_policy::_delete_lifecycle_policy_output::DeleteLifecyclePolicyOutputBuilder;

pub use crate::operation::delete_lifecycle_policy::_delete_lifecycle_policy_input::DeleteLifecyclePolicyInputBuilder;

/// Fluent builder constructing a request to `DeleteLifecyclePolicy`.
///
/// <p>Removes an object lifecycle policy from a container. It takes up to 20 minutes for the change to take effect.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteLifecyclePolicyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyInputBuilder,
}
impl DeleteLifecyclePolicyFluentBuilder {
    /// Creates a new `DeleteLifecyclePolicy`.
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
            crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicy,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError,
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
        crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyError,
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
    /// <p>The name of the container that holds the object lifecycle policy.</p>
    pub fn container_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.container_name(input.into());
        self
    }
    /// <p>The name of the container that holds the object lifecycle policy.</p>
    pub fn set_container_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_container_name(input);
        self
    }
}
