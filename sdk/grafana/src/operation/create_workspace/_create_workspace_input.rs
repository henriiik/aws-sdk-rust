// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateWorkspaceInput {
    /// <p>Specifies whether the workspace can access Amazon Web Services resources in this Amazon Web Services account only, or whether it can also access Amazon Web Services resources in other accounts in the same organization. If you specify <code>ORGANIZATION</code>, you must specify which organizational units the workspace can access in the <code>workspaceOrganizationalUnits</code> parameter.</p>
    #[doc(hidden)]
    pub account_access_type: std::option::Option<crate::types::AccountAccessType>,
    /// <p>A unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>The name of an IAM role that already exists to use with Organizations to access Amazon Web Services data sources and notification channels in other accounts in an organization.</p>
    #[doc(hidden)]
    pub organization_role_name: std::option::Option<std::string::String>,
    /// <p>When creating a workspace through the Amazon Web Services API, CLI or Amazon Web Services CloudFormation, you must manage IAM roles and provision the permissions that the workspace needs to use Amazon Web Services data sources and notification channels.</p>
    /// <p>You must also specify a <code>workspaceRoleArn</code> for a role that you will manage for the workspace to use when accessing those datasources and notification channels.</p>
    /// <p>The ability for Amazon Managed Grafana to create and update IAM roles on behalf of the user is supported only in the Amazon Managed Grafana console, where this value may be set to <code>SERVICE_MANAGED</code>.</p> <note>
    /// <p>Use only the <code>CUSTOMER_MANAGED</code> permission type when creating a workspace with the API, CLI or Amazon Web Services CloudFormation. </p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-manage-permissions.html">Amazon Managed Grafana permissions and policies for Amazon Web Services data sources and notification channels</a>.</p>
    #[doc(hidden)]
    pub permission_type: std::option::Option<crate::types::PermissionType>,
    /// <p>The name of the CloudFormation stack set to use to generate IAM roles to be used for this workspace.</p>
    #[doc(hidden)]
    pub stack_set_name: std::option::Option<std::string::String>,
    /// <p>This parameter is for internal use only, and should not be used.</p>
    #[doc(hidden)]
    pub workspace_data_sources: std::option::Option<std::vec::Vec<crate::types::DataSourceType>>,
    /// <p>A description for the workspace. This is used only to help you identify this workspace.</p>
    /// <p>Pattern: <code>^[\\p{L}\\p{Z}\\p{N}\\p{P}]{0,2048}$</code> </p>
    #[doc(hidden)]
    pub workspace_description: std::option::Option<std::string::String>,
    /// <p>The name for the workspace. It does not have to be unique.</p>
    #[doc(hidden)]
    pub workspace_name: std::option::Option<std::string::String>,
    /// <p>Specify the Amazon Web Services notification channels that you plan to use in this workspace. Specifying these data sources here enables Amazon Managed Grafana to create IAM roles and permissions that allow Amazon Managed Grafana to use these channels.</p>
    #[doc(hidden)]
    pub workspace_notification_destinations:
        std::option::Option<std::vec::Vec<crate::types::NotificationDestinationType>>,
    /// <p>Specifies the organizational units that this workspace is allowed to use data sources from, if this workspace is in an account that is part of an organization.</p>
    #[doc(hidden)]
    pub workspace_organizational_units: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Specified the IAM role that grants permissions to the Amazon Web Services resources that the workspace will view data from, including both data sources and notification channels. You are responsible for managing the permissions for this role as new data sources or notification channels are added. </p>
    #[doc(hidden)]
    pub workspace_role_arn: std::option::Option<std::string::String>,
    /// <p>Specifies whether this workspace uses SAML 2.0, IAM Identity Center (successor to Single Sign-On), or both to authenticate users for using the Grafana console within a workspace. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/authentication-in-AMG.html">User authentication in Amazon Managed Grafana</a>.</p>
    #[doc(hidden)]
    pub authentication_providers:
        std::option::Option<std::vec::Vec<crate::types::AuthenticationProviderTypes>>,
    /// <p>The list of tags associated with the workspace.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    /// <p>The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to.</p>
    #[doc(hidden)]
    pub vpc_configuration: std::option::Option<crate::types::VpcConfiguration>,
    /// <p>The configuration string for the workspace that you create. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    #[doc(hidden)]
    pub configuration: std::option::Option<std::string::String>,
    /// <p>Configuration for network access to your workspace.</p>
    /// <p>When this is configured, only listed IP addresses and VPC endpoints will be able to access your workspace. Standard Grafana authentication and authorization will still be required.</p>
    /// <p>If this is not configured, or is removed, then all IP addresses and VPC endpoints will be allowed. Standard Grafana authentication and authorization will still be required.</p>
    #[doc(hidden)]
    pub network_access_control: std::option::Option<crate::types::NetworkAccessConfiguration>,
}
impl CreateWorkspaceInput {
    /// <p>Specifies whether the workspace can access Amazon Web Services resources in this Amazon Web Services account only, or whether it can also access Amazon Web Services resources in other accounts in the same organization. If you specify <code>ORGANIZATION</code>, you must specify which organizational units the workspace can access in the <code>workspaceOrganizationalUnits</code> parameter.</p>
    pub fn account_access_type(&self) -> std::option::Option<&crate::types::AccountAccessType> {
        self.account_access_type.as_ref()
    }
    /// <p>A unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The name of an IAM role that already exists to use with Organizations to access Amazon Web Services data sources and notification channels in other accounts in an organization.</p>
    pub fn organization_role_name(&self) -> std::option::Option<&str> {
        self.organization_role_name.as_deref()
    }
    /// <p>When creating a workspace through the Amazon Web Services API, CLI or Amazon Web Services CloudFormation, you must manage IAM roles and provision the permissions that the workspace needs to use Amazon Web Services data sources and notification channels.</p>
    /// <p>You must also specify a <code>workspaceRoleArn</code> for a role that you will manage for the workspace to use when accessing those datasources and notification channels.</p>
    /// <p>The ability for Amazon Managed Grafana to create and update IAM roles on behalf of the user is supported only in the Amazon Managed Grafana console, where this value may be set to <code>SERVICE_MANAGED</code>.</p> <note>
    /// <p>Use only the <code>CUSTOMER_MANAGED</code> permission type when creating a workspace with the API, CLI or Amazon Web Services CloudFormation. </p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-manage-permissions.html">Amazon Managed Grafana permissions and policies for Amazon Web Services data sources and notification channels</a>.</p>
    pub fn permission_type(&self) -> std::option::Option<&crate::types::PermissionType> {
        self.permission_type.as_ref()
    }
    /// <p>The name of the CloudFormation stack set to use to generate IAM roles to be used for this workspace.</p>
    pub fn stack_set_name(&self) -> std::option::Option<&str> {
        self.stack_set_name.as_deref()
    }
    /// <p>This parameter is for internal use only, and should not be used.</p>
    pub fn workspace_data_sources(&self) -> std::option::Option<&[crate::types::DataSourceType]> {
        self.workspace_data_sources.as_deref()
    }
    /// <p>A description for the workspace. This is used only to help you identify this workspace.</p>
    /// <p>Pattern: <code>^[\\p{L}\\p{Z}\\p{N}\\p{P}]{0,2048}$</code> </p>
    pub fn workspace_description(&self) -> std::option::Option<&str> {
        self.workspace_description.as_deref()
    }
    /// <p>The name for the workspace. It does not have to be unique.</p>
    pub fn workspace_name(&self) -> std::option::Option<&str> {
        self.workspace_name.as_deref()
    }
    /// <p>Specify the Amazon Web Services notification channels that you plan to use in this workspace. Specifying these data sources here enables Amazon Managed Grafana to create IAM roles and permissions that allow Amazon Managed Grafana to use these channels.</p>
    pub fn workspace_notification_destinations(
        &self,
    ) -> std::option::Option<&[crate::types::NotificationDestinationType]> {
        self.workspace_notification_destinations.as_deref()
    }
    /// <p>Specifies the organizational units that this workspace is allowed to use data sources from, if this workspace is in an account that is part of an organization.</p>
    pub fn workspace_organizational_units(&self) -> std::option::Option<&[std::string::String]> {
        self.workspace_organizational_units.as_deref()
    }
    /// <p>Specified the IAM role that grants permissions to the Amazon Web Services resources that the workspace will view data from, including both data sources and notification channels. You are responsible for managing the permissions for this role as new data sources or notification channels are added. </p>
    pub fn workspace_role_arn(&self) -> std::option::Option<&str> {
        self.workspace_role_arn.as_deref()
    }
    /// <p>Specifies whether this workspace uses SAML 2.0, IAM Identity Center (successor to Single Sign-On), or both to authenticate users for using the Grafana console within a workspace. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/authentication-in-AMG.html">User authentication in Amazon Managed Grafana</a>.</p>
    pub fn authentication_providers(
        &self,
    ) -> std::option::Option<&[crate::types::AuthenticationProviderTypes]> {
        self.authentication_providers.as_deref()
    }
    /// <p>The list of tags associated with the workspace.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
    /// <p>The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to.</p>
    pub fn vpc_configuration(&self) -> std::option::Option<&crate::types::VpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
    /// <p>The configuration string for the workspace that you create. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn configuration(&self) -> std::option::Option<&str> {
        self.configuration.as_deref()
    }
    /// <p>Configuration for network access to your workspace.</p>
    /// <p>When this is configured, only listed IP addresses and VPC endpoints will be able to access your workspace. Standard Grafana authentication and authorization will still be required.</p>
    /// <p>If this is not configured, or is removed, then all IP addresses and VPC endpoints will be allowed. Standard Grafana authentication and authorization will still be required.</p>
    pub fn network_access_control(
        &self,
    ) -> std::option::Option<&crate::types::NetworkAccessConfiguration> {
        self.network_access_control.as_ref()
    }
}
impl std::fmt::Debug for CreateWorkspaceInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateWorkspaceInput");
        formatter.field("account_access_type", &self.account_access_type);
        formatter.field("client_token", &self.client_token);
        formatter.field("organization_role_name", &"*** Sensitive Data Redacted ***");
        formatter.field("permission_type", &self.permission_type);
        formatter.field("stack_set_name", &self.stack_set_name);
        formatter.field("workspace_data_sources", &self.workspace_data_sources);
        formatter.field("workspace_description", &"*** Sensitive Data Redacted ***");
        formatter.field("workspace_name", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "workspace_notification_destinations",
            &self.workspace_notification_destinations,
        );
        formatter.field(
            "workspace_organizational_units",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("workspace_role_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("authentication_providers", &self.authentication_providers);
        formatter.field("tags", &self.tags);
        formatter.field("vpc_configuration", &self.vpc_configuration);
        formatter.field("configuration", &self.configuration);
        formatter.field("network_access_control", &self.network_access_control);
        formatter.finish()
    }
}
impl CreateWorkspaceInput {
    /// Creates a new builder-style object to manufacture [`CreateWorkspaceInput`](crate::operation::create_workspace::CreateWorkspaceInput).
    pub fn builder() -> crate::operation::create_workspace::builders::CreateWorkspaceInputBuilder {
        crate::operation::create_workspace::builders::CreateWorkspaceInputBuilder::default()
    }
}

/// A builder for [`CreateWorkspaceInput`](crate::operation::create_workspace::CreateWorkspaceInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
pub struct CreateWorkspaceInputBuilder {
    pub(crate) account_access_type: std::option::Option<crate::types::AccountAccessType>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) organization_role_name: std::option::Option<std::string::String>,
    pub(crate) permission_type: std::option::Option<crate::types::PermissionType>,
    pub(crate) stack_set_name: std::option::Option<std::string::String>,
    pub(crate) workspace_data_sources:
        std::option::Option<std::vec::Vec<crate::types::DataSourceType>>,
    pub(crate) workspace_description: std::option::Option<std::string::String>,
    pub(crate) workspace_name: std::option::Option<std::string::String>,
    pub(crate) workspace_notification_destinations:
        std::option::Option<std::vec::Vec<crate::types::NotificationDestinationType>>,
    pub(crate) workspace_organizational_units:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) workspace_role_arn: std::option::Option<std::string::String>,
    pub(crate) authentication_providers:
        std::option::Option<std::vec::Vec<crate::types::AuthenticationProviderTypes>>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    pub(crate) vpc_configuration: std::option::Option<crate::types::VpcConfiguration>,
    pub(crate) configuration: std::option::Option<std::string::String>,
    pub(crate) network_access_control:
        std::option::Option<crate::types::NetworkAccessConfiguration>,
}
impl CreateWorkspaceInputBuilder {
    /// <p>Specifies whether the workspace can access Amazon Web Services resources in this Amazon Web Services account only, or whether it can also access Amazon Web Services resources in other accounts in the same organization. If you specify <code>ORGANIZATION</code>, you must specify which organizational units the workspace can access in the <code>workspaceOrganizationalUnits</code> parameter.</p>
    pub fn account_access_type(mut self, input: crate::types::AccountAccessType) -> Self {
        self.account_access_type = Some(input);
        self
    }
    /// <p>Specifies whether the workspace can access Amazon Web Services resources in this Amazon Web Services account only, or whether it can also access Amazon Web Services resources in other accounts in the same organization. If you specify <code>ORGANIZATION</code>, you must specify which organizational units the workspace can access in the <code>workspaceOrganizationalUnits</code> parameter.</p>
    pub fn set_account_access_type(
        mut self,
        input: std::option::Option<crate::types::AccountAccessType>,
    ) -> Self {
        self.account_access_type = input;
        self
    }
    /// <p>A unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive, user-provided identifier to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The name of an IAM role that already exists to use with Organizations to access Amazon Web Services data sources and notification channels in other accounts in an organization.</p>
    pub fn organization_role_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.organization_role_name = Some(input.into());
        self
    }
    /// <p>The name of an IAM role that already exists to use with Organizations to access Amazon Web Services data sources and notification channels in other accounts in an organization.</p>
    pub fn set_organization_role_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.organization_role_name = input;
        self
    }
    /// <p>When creating a workspace through the Amazon Web Services API, CLI or Amazon Web Services CloudFormation, you must manage IAM roles and provision the permissions that the workspace needs to use Amazon Web Services data sources and notification channels.</p>
    /// <p>You must also specify a <code>workspaceRoleArn</code> for a role that you will manage for the workspace to use when accessing those datasources and notification channels.</p>
    /// <p>The ability for Amazon Managed Grafana to create and update IAM roles on behalf of the user is supported only in the Amazon Managed Grafana console, where this value may be set to <code>SERVICE_MANAGED</code>.</p> <note>
    /// <p>Use only the <code>CUSTOMER_MANAGED</code> permission type when creating a workspace with the API, CLI or Amazon Web Services CloudFormation. </p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-manage-permissions.html">Amazon Managed Grafana permissions and policies for Amazon Web Services data sources and notification channels</a>.</p>
    pub fn permission_type(mut self, input: crate::types::PermissionType) -> Self {
        self.permission_type = Some(input);
        self
    }
    /// <p>When creating a workspace through the Amazon Web Services API, CLI or Amazon Web Services CloudFormation, you must manage IAM roles and provision the permissions that the workspace needs to use Amazon Web Services data sources and notification channels.</p>
    /// <p>You must also specify a <code>workspaceRoleArn</code> for a role that you will manage for the workspace to use when accessing those datasources and notification channels.</p>
    /// <p>The ability for Amazon Managed Grafana to create and update IAM roles on behalf of the user is supported only in the Amazon Managed Grafana console, where this value may be set to <code>SERVICE_MANAGED</code>.</p> <note>
    /// <p>Use only the <code>CUSTOMER_MANAGED</code> permission type when creating a workspace with the API, CLI or Amazon Web Services CloudFormation. </p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-manage-permissions.html">Amazon Managed Grafana permissions and policies for Amazon Web Services data sources and notification channels</a>.</p>
    pub fn set_permission_type(
        mut self,
        input: std::option::Option<crate::types::PermissionType>,
    ) -> Self {
        self.permission_type = input;
        self
    }
    /// <p>The name of the CloudFormation stack set to use to generate IAM roles to be used for this workspace.</p>
    pub fn stack_set_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.stack_set_name = Some(input.into());
        self
    }
    /// <p>The name of the CloudFormation stack set to use to generate IAM roles to be used for this workspace.</p>
    pub fn set_stack_set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.stack_set_name = input;
        self
    }
    /// Appends an item to `workspace_data_sources`.
    ///
    /// To override the contents of this collection use [`set_workspace_data_sources`](Self::set_workspace_data_sources).
    ///
    /// <p>This parameter is for internal use only, and should not be used.</p>
    pub fn workspace_data_sources(mut self, input: crate::types::DataSourceType) -> Self {
        let mut v = self.workspace_data_sources.unwrap_or_default();
        v.push(input);
        self.workspace_data_sources = Some(v);
        self
    }
    /// <p>This parameter is for internal use only, and should not be used.</p>
    pub fn set_workspace_data_sources(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::DataSourceType>>,
    ) -> Self {
        self.workspace_data_sources = input;
        self
    }
    /// <p>A description for the workspace. This is used only to help you identify this workspace.</p>
    /// <p>Pattern: <code>^[\\p{L}\\p{Z}\\p{N}\\p{P}]{0,2048}$</code> </p>
    pub fn workspace_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.workspace_description = Some(input.into());
        self
    }
    /// <p>A description for the workspace. This is used only to help you identify this workspace.</p>
    /// <p>Pattern: <code>^[\\p{L}\\p{Z}\\p{N}\\p{P}]{0,2048}$</code> </p>
    pub fn set_workspace_description(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.workspace_description = input;
        self
    }
    /// <p>The name for the workspace. It does not have to be unique.</p>
    pub fn workspace_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.workspace_name = Some(input.into());
        self
    }
    /// <p>The name for the workspace. It does not have to be unique.</p>
    pub fn set_workspace_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.workspace_name = input;
        self
    }
    /// Appends an item to `workspace_notification_destinations`.
    ///
    /// To override the contents of this collection use [`set_workspace_notification_destinations`](Self::set_workspace_notification_destinations).
    ///
    /// <p>Specify the Amazon Web Services notification channels that you plan to use in this workspace. Specifying these data sources here enables Amazon Managed Grafana to create IAM roles and permissions that allow Amazon Managed Grafana to use these channels.</p>
    pub fn workspace_notification_destinations(
        mut self,
        input: crate::types::NotificationDestinationType,
    ) -> Self {
        let mut v = self.workspace_notification_destinations.unwrap_or_default();
        v.push(input);
        self.workspace_notification_destinations = Some(v);
        self
    }
    /// <p>Specify the Amazon Web Services notification channels that you plan to use in this workspace. Specifying these data sources here enables Amazon Managed Grafana to create IAM roles and permissions that allow Amazon Managed Grafana to use these channels.</p>
    pub fn set_workspace_notification_destinations(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::NotificationDestinationType>>,
    ) -> Self {
        self.workspace_notification_destinations = input;
        self
    }
    /// Appends an item to `workspace_organizational_units`.
    ///
    /// To override the contents of this collection use [`set_workspace_organizational_units`](Self::set_workspace_organizational_units).
    ///
    /// <p>Specifies the organizational units that this workspace is allowed to use data sources from, if this workspace is in an account that is part of an organization.</p>
    pub fn workspace_organizational_units(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.workspace_organizational_units.unwrap_or_default();
        v.push(input.into());
        self.workspace_organizational_units = Some(v);
        self
    }
    /// <p>Specifies the organizational units that this workspace is allowed to use data sources from, if this workspace is in an account that is part of an organization.</p>
    pub fn set_workspace_organizational_units(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.workspace_organizational_units = input;
        self
    }
    /// <p>Specified the IAM role that grants permissions to the Amazon Web Services resources that the workspace will view data from, including both data sources and notification channels. You are responsible for managing the permissions for this role as new data sources or notification channels are added. </p>
    pub fn workspace_role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.workspace_role_arn = Some(input.into());
        self
    }
    /// <p>Specified the IAM role that grants permissions to the Amazon Web Services resources that the workspace will view data from, including both data sources and notification channels. You are responsible for managing the permissions for this role as new data sources or notification channels are added. </p>
    pub fn set_workspace_role_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.workspace_role_arn = input;
        self
    }
    /// Appends an item to `authentication_providers`.
    ///
    /// To override the contents of this collection use [`set_authentication_providers`](Self::set_authentication_providers).
    ///
    /// <p>Specifies whether this workspace uses SAML 2.0, IAM Identity Center (successor to Single Sign-On), or both to authenticate users for using the Grafana console within a workspace. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/authentication-in-AMG.html">User authentication in Amazon Managed Grafana</a>.</p>
    pub fn authentication_providers(
        mut self,
        input: crate::types::AuthenticationProviderTypes,
    ) -> Self {
        let mut v = self.authentication_providers.unwrap_or_default();
        v.push(input);
        self.authentication_providers = Some(v);
        self
    }
    /// <p>Specifies whether this workspace uses SAML 2.0, IAM Identity Center (successor to Single Sign-On), or both to authenticate users for using the Grafana console within a workspace. For more information, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/authentication-in-AMG.html">User authentication in Amazon Managed Grafana</a>.</p>
    pub fn set_authentication_providers(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AuthenticationProviderTypes>>,
    ) -> Self {
        self.authentication_providers = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The list of tags associated with the workspace.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>The list of tags associated with the workspace.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to.</p>
    pub fn vpc_configuration(mut self, input: crate::types::VpcConfiguration) -> Self {
        self.vpc_configuration = Some(input);
        self
    }
    /// <p>The configuration settings for an Amazon VPC that contains data sources for your Grafana workspace to connect to.</p>
    pub fn set_vpc_configuration(
        mut self,
        input: std::option::Option<crate::types::VpcConfiguration>,
    ) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// <p>The configuration string for the workspace that you create. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn configuration(mut self, input: impl Into<std::string::String>) -> Self {
        self.configuration = Some(input.into());
        self
    }
    /// <p>The configuration string for the workspace that you create. For more information about the format and configuration options available, see <a href="https://docs.aws.amazon.com/grafana/latest/userguide/AMG-configure-workspace.html">Working in your Grafana workspace</a>.</p>
    pub fn set_configuration(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.configuration = input;
        self
    }
    /// <p>Configuration for network access to your workspace.</p>
    /// <p>When this is configured, only listed IP addresses and VPC endpoints will be able to access your workspace. Standard Grafana authentication and authorization will still be required.</p>
    /// <p>If this is not configured, or is removed, then all IP addresses and VPC endpoints will be allowed. Standard Grafana authentication and authorization will still be required.</p>
    pub fn network_access_control(
        mut self,
        input: crate::types::NetworkAccessConfiguration,
    ) -> Self {
        self.network_access_control = Some(input);
        self
    }
    /// <p>Configuration for network access to your workspace.</p>
    /// <p>When this is configured, only listed IP addresses and VPC endpoints will be able to access your workspace. Standard Grafana authentication and authorization will still be required.</p>
    /// <p>If this is not configured, or is removed, then all IP addresses and VPC endpoints will be allowed. Standard Grafana authentication and authorization will still be required.</p>
    pub fn set_network_access_control(
        mut self,
        input: std::option::Option<crate::types::NetworkAccessConfiguration>,
    ) -> Self {
        self.network_access_control = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateWorkspaceInput`](crate::operation::create_workspace::CreateWorkspaceInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_workspace::CreateWorkspaceInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_workspace::CreateWorkspaceInput {
            account_access_type: self.account_access_type,
            client_token: self.client_token,
            organization_role_name: self.organization_role_name,
            permission_type: self.permission_type,
            stack_set_name: self.stack_set_name,
            workspace_data_sources: self.workspace_data_sources,
            workspace_description: self.workspace_description,
            workspace_name: self.workspace_name,
            workspace_notification_destinations: self.workspace_notification_destinations,
            workspace_organizational_units: self.workspace_organizational_units,
            workspace_role_arn: self.workspace_role_arn,
            authentication_providers: self.authentication_providers,
            tags: self.tags,
            vpc_configuration: self.vpc_configuration,
            configuration: self.configuration,
            network_access_control: self.network_access_control,
        })
    }
}
impl std::fmt::Debug for CreateWorkspaceInputBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateWorkspaceInputBuilder");
        formatter.field("account_access_type", &self.account_access_type);
        formatter.field("client_token", &self.client_token);
        formatter.field("organization_role_name", &"*** Sensitive Data Redacted ***");
        formatter.field("permission_type", &self.permission_type);
        formatter.field("stack_set_name", &self.stack_set_name);
        formatter.field("workspace_data_sources", &self.workspace_data_sources);
        formatter.field("workspace_description", &"*** Sensitive Data Redacted ***");
        formatter.field("workspace_name", &"*** Sensitive Data Redacted ***");
        formatter.field(
            "workspace_notification_destinations",
            &self.workspace_notification_destinations,
        );
        formatter.field(
            "workspace_organizational_units",
            &"*** Sensitive Data Redacted ***",
        );
        formatter.field("workspace_role_arn", &"*** Sensitive Data Redacted ***");
        formatter.field("authentication_providers", &self.authentication_providers);
        formatter.field("tags", &self.tags);
        formatter.field("vpc_configuration", &self.vpc_configuration);
        formatter.field("configuration", &self.configuration);
        formatter.field("network_access_control", &self.network_access_control);
        formatter.finish()
    }
}
