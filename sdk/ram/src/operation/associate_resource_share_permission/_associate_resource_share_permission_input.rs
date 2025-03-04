// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AssociateResourceSharePermissionInput {
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share to which you want to add or replace permissions.</p>
    #[doc(hidden)]
    pub resource_share_arn: std::option::Option<std::string::String>,
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the RAM permission to associate with the resource share. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    #[doc(hidden)]
    pub permission_arn: std::option::Option<std::string::String>,
    /// <p>Specifies whether the specified permission should replace or add to the existing permission associated with the resource share. Use <code>true</code> to replace the current permissions. Use <code>false</code> to add the permission to the current permission. The default value is <code>false</code>.</p> <note>
    /// <p>A resource share can have only one permission per resource type. If a resource share already has a permission for the specified resource type and you don't set <code>replace</code> to <code>true</code> then the operation returns an error. This helps prevent accidental overwriting of a permission.</p>
    /// </note>
    #[doc(hidden)]
    pub replace: std::option::Option<bool>,
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Specifies the version of the RAM permission to associate with the resource share. If you don't specify this parameter, the operation uses the version designated as the default. You can use the <code>ListPermissionVersions</code> operation to discover the available versions of a permission.</p>
    #[doc(hidden)]
    pub permission_version: std::option::Option<i32>,
}
impl AssociateResourceSharePermissionInput {
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share to which you want to add or replace permissions.</p>
    pub fn resource_share_arn(&self) -> std::option::Option<&str> {
        self.resource_share_arn.as_deref()
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the RAM permission to associate with the resource share. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    pub fn permission_arn(&self) -> std::option::Option<&str> {
        self.permission_arn.as_deref()
    }
    /// <p>Specifies whether the specified permission should replace or add to the existing permission associated with the resource share. Use <code>true</code> to replace the current permissions. Use <code>false</code> to add the permission to the current permission. The default value is <code>false</code>.</p> <note>
    /// <p>A resource share can have only one permission per resource type. If a resource share already has a permission for the specified resource type and you don't set <code>replace</code> to <code>true</code> then the operation returns an error. This helps prevent accidental overwriting of a permission.</p>
    /// </note>
    pub fn replace(&self) -> std::option::Option<bool> {
        self.replace
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Specifies the version of the RAM permission to associate with the resource share. If you don't specify this parameter, the operation uses the version designated as the default. You can use the <code>ListPermissionVersions</code> operation to discover the available versions of a permission.</p>
    pub fn permission_version(&self) -> std::option::Option<i32> {
        self.permission_version
    }
}
impl AssociateResourceSharePermissionInput {
    /// Creates a new builder-style object to manufacture [`AssociateResourceSharePermissionInput`](crate::operation::associate_resource_share_permission::AssociateResourceSharePermissionInput).
    pub fn builder() -> crate::operation::associate_resource_share_permission::builders::AssociateResourceSharePermissionInputBuilder{
        crate::operation::associate_resource_share_permission::builders::AssociateResourceSharePermissionInputBuilder::default()
    }
}

/// A builder for [`AssociateResourceSharePermissionInput`](crate::operation::associate_resource_share_permission::AssociateResourceSharePermissionInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct AssociateResourceSharePermissionInputBuilder {
    pub(crate) resource_share_arn: std::option::Option<std::string::String>,
    pub(crate) permission_arn: std::option::Option<std::string::String>,
    pub(crate) replace: std::option::Option<bool>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) permission_version: std::option::Option<i32>,
}
impl AssociateResourceSharePermissionInputBuilder {
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share to which you want to add or replace permissions.</p>
    pub fn resource_share_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_share_arn = Some(input.into());
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the resource share to which you want to add or replace permissions.</p>
    pub fn set_resource_share_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.resource_share_arn = input;
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the RAM permission to associate with the resource share. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    pub fn permission_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.permission_arn = Some(input.into());
        self
    }
    /// <p>Specifies the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resoure Name (ARN)</a> of the RAM permission to associate with the resource share. To find the ARN for a permission, use either the <code>ListPermissions</code> operation or go to the <a href="https://console.aws.amazon.com/ram/home#Permissions:">Permissions library</a> page in the RAM console and then choose the name of the permission. The ARN is displayed on the detail page.</p>
    pub fn set_permission_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.permission_arn = input;
        self
    }
    /// <p>Specifies whether the specified permission should replace or add to the existing permission associated with the resource share. Use <code>true</code> to replace the current permissions. Use <code>false</code> to add the permission to the current permission. The default value is <code>false</code>.</p> <note>
    /// <p>A resource share can have only one permission per resource type. If a resource share already has a permission for the specified resource type and you don't set <code>replace</code> to <code>true</code> then the operation returns an error. This helps prevent accidental overwriting of a permission.</p>
    /// </note>
    pub fn replace(mut self, input: bool) -> Self {
        self.replace = Some(input);
        self
    }
    /// <p>Specifies whether the specified permission should replace or add to the existing permission associated with the resource share. Use <code>true</code> to replace the current permissions. Use <code>false</code> to add the permission to the current permission. The default value is <code>false</code>.</p> <note>
    /// <p>A resource share can have only one permission per resource type. If a resource share already has a permission for the specified resource type and you don't set <code>replace</code> to <code>true</code> then the operation returns an error. This helps prevent accidental overwriting of a permission.</p>
    /// </note>
    pub fn set_replace(mut self, input: std::option::Option<bool>) -> Self {
        self.replace = input;
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Specifies a unique, case-sensitive identifier that you provide to ensure the idempotency of the request. This lets you safely retry the request without accidentally performing the same operation a second time. Passing the same value to a later call to an operation requires that you also pass the same value for all other parameters. We recommend that you use a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID type of value.</a>.</p>
    /// <p>If you don't provide this value, then Amazon Web Services generates a random one for you.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Specifies the version of the RAM permission to associate with the resource share. If you don't specify this parameter, the operation uses the version designated as the default. You can use the <code>ListPermissionVersions</code> operation to discover the available versions of a permission.</p>
    pub fn permission_version(mut self, input: i32) -> Self {
        self.permission_version = Some(input);
        self
    }
    /// <p>Specifies the version of the RAM permission to associate with the resource share. If you don't specify this parameter, the operation uses the version designated as the default. You can use the <code>ListPermissionVersions</code> operation to discover the available versions of a permission.</p>
    pub fn set_permission_version(mut self, input: std::option::Option<i32>) -> Self {
        self.permission_version = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateResourceSharePermissionInput`](crate::operation::associate_resource_share_permission::AssociateResourceSharePermissionInput).
    pub fn build(self) -> Result<crate::operation::associate_resource_share_permission::AssociateResourceSharePermissionInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::associate_resource_share_permission::AssociateResourceSharePermissionInput {
                resource_share_arn: self.resource_share_arn
                ,
                permission_arn: self.permission_arn
                ,
                replace: self.replace
                ,
                client_token: self.client_token
                ,
                permission_version: self.permission_version
                ,
            }
        )
    }
}
