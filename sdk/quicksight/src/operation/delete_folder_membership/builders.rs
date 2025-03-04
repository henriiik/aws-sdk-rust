// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_folder_membership::_delete_folder_membership_output::DeleteFolderMembershipOutputBuilder;

pub use crate::operation::delete_folder_membership::_delete_folder_membership_input::DeleteFolderMembershipInputBuilder;

/// Fluent builder constructing a request to `DeleteFolderMembership`.
///
/// <p>Removes an asset, such as a dashboard, analysis, or dataset, from a folder.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteFolderMembershipFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_folder_membership::builders::DeleteFolderMembershipInputBuilder,
}
impl DeleteFolderMembershipFluentBuilder {
    /// Creates a new `DeleteFolderMembership`.
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
            crate::operation::delete_folder_membership::DeleteFolderMembership,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_folder_membership::DeleteFolderMembershipError,
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
        crate::operation::delete_folder_membership::DeleteFolderMembershipOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_folder_membership::DeleteFolderMembershipError,
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
    /// <p>The ID for the Amazon Web Services account that contains the folder.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID for the Amazon Web Services account that contains the folder.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The Folder ID.</p>
    pub fn folder_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.folder_id(input.into());
        self
    }
    /// <p>The Folder ID.</p>
    pub fn set_folder_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_folder_id(input);
        self
    }
    /// <p>The ID of the asset (the dashboard, analysis, or dataset) that you want to delete.</p>
    pub fn member_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.member_id(input.into());
        self
    }
    /// <p>The ID of the asset (the dashboard, analysis, or dataset) that you want to delete.</p>
    pub fn set_member_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_member_id(input);
        self
    }
    /// <p>The type of the member, including <code>DASHBOARD</code>, <code>ANALYSIS</code>, and <code>DATASET</code> </p>
    pub fn member_type(mut self, input: crate::types::MemberType) -> Self {
        self.inner = self.inner.member_type(input);
        self
    }
    /// <p>The type of the member, including <code>DASHBOARD</code>, <code>ANALYSIS</code>, and <code>DATASET</code> </p>
    pub fn set_member_type(mut self, input: std::option::Option<crate::types::MemberType>) -> Self {
        self.inner = self.inner.set_member_type(input);
        self
    }
}
