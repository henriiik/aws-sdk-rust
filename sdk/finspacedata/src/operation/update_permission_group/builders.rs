// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_permission_group::_update_permission_group_output::UpdatePermissionGroupOutputBuilder;

pub use crate::operation::update_permission_group::_update_permission_group_input::UpdatePermissionGroupInputBuilder;

/// Fluent builder constructing a request to `UpdatePermissionGroup`.
///
/// <p>Modifies the details of a permission group. You cannot modify a <code>permissionGroupID</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdatePermissionGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_permission_group::builders::UpdatePermissionGroupInputBuilder,
}
impl UpdatePermissionGroupFluentBuilder {
    /// Creates a new `UpdatePermissionGroup`.
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
            crate::operation::update_permission_group::UpdatePermissionGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_permission_group::UpdatePermissionGroupError,
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
        crate::operation::update_permission_group::UpdatePermissionGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_permission_group::UpdatePermissionGroupError,
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
    /// <p>The unique identifier for the permission group to update.</p>
    pub fn permission_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.permission_group_id(input.into());
        self
    }
    /// <p>The unique identifier for the permission group to update.</p>
    pub fn set_permission_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_permission_group_id(input);
        self
    }
    /// <p>The name of the permission group.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the permission group.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A brief description for the permission group.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A brief description for the permission group.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `applicationPermissions`.
    ///
    /// To override the contents of this collection use [`set_application_permissions`](Self::set_application_permissions).
    ///
    /// <p>The permissions that are granted to a specific group for accessing the FinSpace application.</p> <important>
    /// <p>When assigning application permissions, be aware that the permission <code>ManageUsersAndGroups</code> allows users to grant themselves or others access to any functionality in their FinSpace environment's application. It should only be granted to trusted users.</p>
    /// </important>
    /// <ul>
    /// <li> <p> <code>CreateDataset</code> – Group members can create new datasets.</p> </li>
    /// <li> <p> <code>ManageClusters</code> – Group members can manage Apache Spark clusters from FinSpace notebooks.</p> </li>
    /// <li> <p> <code>ManageUsersAndGroups</code> – Group members can manage users and permission groups. This is a privileged permission that allows users to grant themselves or others access to any functionality in the application. It should only be granted to trusted users.</p> </li>
    /// <li> <p> <code>ManageAttributeSets</code> – Group members can manage attribute sets.</p> </li>
    /// <li> <p> <code>ViewAuditData</code> – Group members can view audit data.</p> </li>
    /// <li> <p> <code>AccessNotebooks</code> – Group members will have access to FinSpace notebooks.</p> </li>
    /// <li> <p> <code>GetTemporaryCredentials</code> – Group members can get temporary API credentials.</p> </li>
    /// </ul>
    pub fn application_permissions(mut self, input: crate::types::ApplicationPermission) -> Self {
        self.inner = self.inner.application_permissions(input);
        self
    }
    /// <p>The permissions that are granted to a specific group for accessing the FinSpace application.</p> <important>
    /// <p>When assigning application permissions, be aware that the permission <code>ManageUsersAndGroups</code> allows users to grant themselves or others access to any functionality in their FinSpace environment's application. It should only be granted to trusted users.</p>
    /// </important>
    /// <ul>
    /// <li> <p> <code>CreateDataset</code> – Group members can create new datasets.</p> </li>
    /// <li> <p> <code>ManageClusters</code> – Group members can manage Apache Spark clusters from FinSpace notebooks.</p> </li>
    /// <li> <p> <code>ManageUsersAndGroups</code> – Group members can manage users and permission groups. This is a privileged permission that allows users to grant themselves or others access to any functionality in the application. It should only be granted to trusted users.</p> </li>
    /// <li> <p> <code>ManageAttributeSets</code> – Group members can manage attribute sets.</p> </li>
    /// <li> <p> <code>ViewAuditData</code> – Group members can view audit data.</p> </li>
    /// <li> <p> <code>AccessNotebooks</code> – Group members will have access to FinSpace notebooks.</p> </li>
    /// <li> <p> <code>GetTemporaryCredentials</code> – Group members can get temporary API credentials.</p> </li>
    /// </ul>
    pub fn set_application_permissions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ApplicationPermission>>,
    ) -> Self {
        self.inner = self.inner.set_application_permissions(input);
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
}
