// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_user_hierarchy_group_name::_update_user_hierarchy_group_name_output::UpdateUserHierarchyGroupNameOutputBuilder;

pub use crate::operation::update_user_hierarchy_group_name::_update_user_hierarchy_group_name_input::UpdateUserHierarchyGroupNameInputBuilder;

/// Fluent builder constructing a request to `UpdateUserHierarchyGroupName`.
///
/// <p>Updates the name of the user hierarchy group. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateUserHierarchyGroupNameFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::update_user_hierarchy_group_name::builders::UpdateUserHierarchyGroupNameInputBuilder
            }
impl UpdateUserHierarchyGroupNameFluentBuilder {
    /// Creates a new `UpdateUserHierarchyGroupName`.
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
            crate::operation::update_user_hierarchy_group_name::UpdateUserHierarchyGroupName,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_user_hierarchy_group_name::UpdateUserHierarchyGroupNameError,
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
        crate::operation::update_user_hierarchy_group_name::UpdateUserHierarchyGroupNameOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_user_hierarchy_group_name::UpdateUserHierarchyGroupNameError,
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
    /// <p>The name of the hierarchy group. Must not be more than 100 characters.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the hierarchy group. Must not be more than 100 characters.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The identifier of the hierarchy group.</p>
    pub fn hierarchy_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.hierarchy_group_id(input.into());
        self
    }
    /// <p>The identifier of the hierarchy group.</p>
    pub fn set_hierarchy_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_hierarchy_group_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
}
