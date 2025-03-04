// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_account_settings::_update_account_settings_output::UpdateAccountSettingsOutputBuilder;

pub use crate::operation::update_account_settings::_update_account_settings_input::UpdateAccountSettingsInputBuilder;

/// Fluent builder constructing a request to `UpdateAccountSettings`.
///
/// <p>Update Proton settings that are used for multiple services in the Amazon Web Services account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAccountSettingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_account_settings::builders::UpdateAccountSettingsInputBuilder,
}
impl UpdateAccountSettingsFluentBuilder {
    /// Creates a new `UpdateAccountSettings`.
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
            crate::operation::update_account_settings::UpdateAccountSettings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
        crate::operation::update_account_settings::UpdateAccountSettingsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_account_settings::UpdateAccountSettingsError,
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
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Assumed by Proton for Amazon Web Services-managed provisioning, and by customer-owned automation for self-managed provisioning.</p>
    /// <p>To remove a previously configured ARN, specify an empty string.</p>
    pub fn pipeline_service_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.pipeline_service_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Assumed by Proton for Amazon Web Services-managed provisioning, and by customer-owned automation for self-managed provisioning.</p>
    /// <p>To remove a previously configured ARN, specify an empty string.</p>
    pub fn set_pipeline_service_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_service_role_arn(input);
        self
    }
    /// <p>A linked repository for pipeline provisioning. Specify it if you have environments configured for self-managed provisioning with services that include pipelines. A linked repository is a repository that has been registered with Proton. For more information, see <code>CreateRepository</code>.</p>
    /// <p>To remove a previously configured repository, set <code>deletePipelineProvisioningRepository</code> to <code>true</code>, and don't set <code>pipelineProvisioningRepository</code>.</p>
    pub fn pipeline_provisioning_repository(
        mut self,
        input: crate::types::RepositoryBranchInput,
    ) -> Self {
        self.inner = self.inner.pipeline_provisioning_repository(input);
        self
    }
    /// <p>A linked repository for pipeline provisioning. Specify it if you have environments configured for self-managed provisioning with services that include pipelines. A linked repository is a repository that has been registered with Proton. For more information, see <code>CreateRepository</code>.</p>
    /// <p>To remove a previously configured repository, set <code>deletePipelineProvisioningRepository</code> to <code>true</code>, and don't set <code>pipelineProvisioningRepository</code>.</p>
    pub fn set_pipeline_provisioning_repository(
        mut self,
        input: std::option::Option<crate::types::RepositoryBranchInput>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_provisioning_repository(input);
        self
    }
    /// <p>Set to <code>true</code> to remove a configured pipeline repository from the account settings. Don't set this field if you are updating the configured pipeline repository.</p>
    pub fn delete_pipeline_provisioning_repository(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_pipeline_provisioning_repository(input);
        self
    }
    /// <p>Set to <code>true</code> to remove a configured pipeline repository from the account settings. Don't set this field if you are updating the configured pipeline repository.</p>
    pub fn set_delete_pipeline_provisioning_repository(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.inner = self
            .inner
            .set_delete_pipeline_provisioning_repository(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Proton assumes this role for CodeBuild-based provisioning.</p>
    pub fn pipeline_codebuild_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.pipeline_codebuild_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the service role you want to use for provisioning pipelines. Proton assumes this role for CodeBuild-based provisioning.</p>
    pub fn set_pipeline_codebuild_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_pipeline_codebuild_role_arn(input);
        self
    }
}
