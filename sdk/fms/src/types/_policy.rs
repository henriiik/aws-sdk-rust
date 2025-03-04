// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An Firewall Manager policy.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct Policy {
    /// <p>The ID of the Firewall Manager policy.</p>
    #[doc(hidden)]
    pub policy_id: std::option::Option<std::string::String>,
    /// <p>The name of the Firewall Manager policy.</p>
    #[doc(hidden)]
    pub policy_name: std::option::Option<std::string::String>,
    /// <p>A unique identifier for each update to the policy. When issuing a <code>PutPolicy</code> request, the <code>PolicyUpdateToken</code> in the request must match the <code>PolicyUpdateToken</code> of the current policy version. To get the <code>PolicyUpdateToken</code> of the current policy version, use a <code>GetPolicy</code> request.</p>
    #[doc(hidden)]
    pub policy_update_token: std::option::Option<std::string::String>,
    /// <p>Details about the security service that is being used to protect the resources.</p>
    #[doc(hidden)]
    pub security_service_policy_data: std::option::Option<crate::types::SecurityServicePolicyData>,
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services Resource Types Reference</a>. To apply this policy to multiple resource types, specify a resource type of <code>ResourceTypeList</code> and then specify the resource types in a <code>ResourceTypeList</code>.</p>
    /// <p>For WAF and Shield Advanced, resource types include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code>, <code>AWS::ElasticLoadBalancing::LoadBalancer</code>, <code>AWS::EC2::EIP</code>, and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an Network Firewall policy or DNS Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<std::string::String>,
    /// <p>An array of <code>ResourceType</code> objects. Use this only to specify multiple resource types. To specify a single resource type, use <code>ResourceType</code>.</p>
    #[doc(hidden)]
    pub resource_type_list: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>An array of <code>ResourceTag</code> objects.</p>
    #[doc(hidden)]
    pub resource_tags: std::option::Option<std::vec::Vec<crate::types::ResourceTag>>,
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not in scope of the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are in scope of the policy.</p>
    #[doc(hidden)]
    pub exclude_resource_tags: bool,
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    #[doc(hidden)]
    pub remediation_enabled: bool,
    /// <p>Indicates whether Firewall Manager should automatically remove protections from resources that leave the policy scope and clean up resources that Firewall Manager is managing for accounts when those accounts leave policy scope. For example, Firewall Manager will disassociate a Firewall Manager managed web ACL from a protected customer resource when the customer resource leaves policy scope. </p>
    /// <p>By default, Firewall Manager doesn't remove protections or delete Firewall Manager managed resources. </p>
    /// <p>This option is not available for Shield Advanced or WAF Classic policies.</p>
    #[doc(hidden)]
    pub delete_unused_fm_managed_resources: bool,
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to include in the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub include_map: std::option::Option<
        std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    >,
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to exclude from the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub exclude_map: std::option::Option<
        std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    >,
    /// <p>The unique identifiers of the resource sets used by the policy.</p>
    #[doc(hidden)]
    pub resource_set_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The definition of the Network Firewall firewall policy.</p>
    #[doc(hidden)]
    pub policy_description: std::option::Option<std::string::String>,
}
impl Policy {
    /// <p>The ID of the Firewall Manager policy.</p>
    pub fn policy_id(&self) -> std::option::Option<&str> {
        self.policy_id.as_deref()
    }
    /// <p>The name of the Firewall Manager policy.</p>
    pub fn policy_name(&self) -> std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>A unique identifier for each update to the policy. When issuing a <code>PutPolicy</code> request, the <code>PolicyUpdateToken</code> in the request must match the <code>PolicyUpdateToken</code> of the current policy version. To get the <code>PolicyUpdateToken</code> of the current policy version, use a <code>GetPolicy</code> request.</p>
    pub fn policy_update_token(&self) -> std::option::Option<&str> {
        self.policy_update_token.as_deref()
    }
    /// <p>Details about the security service that is being used to protect the resources.</p>
    pub fn security_service_policy_data(
        &self,
    ) -> std::option::Option<&crate::types::SecurityServicePolicyData> {
        self.security_service_policy_data.as_ref()
    }
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services Resource Types Reference</a>. To apply this policy to multiple resource types, specify a resource type of <code>ResourceTypeList</code> and then specify the resource types in a <code>ResourceTypeList</code>.</p>
    /// <p>For WAF and Shield Advanced, resource types include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code>, <code>AWS::ElasticLoadBalancing::LoadBalancer</code>, <code>AWS::EC2::EIP</code>, and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an Network Firewall policy or DNS Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    pub fn resource_type(&self) -> std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>An array of <code>ResourceType</code> objects. Use this only to specify multiple resource types. To specify a single resource type, use <code>ResourceType</code>.</p>
    pub fn resource_type_list(&self) -> std::option::Option<&[std::string::String]> {
        self.resource_type_list.as_deref()
    }
    /// <p>An array of <code>ResourceTag</code> objects.</p>
    pub fn resource_tags(&self) -> std::option::Option<&[crate::types::ResourceTag]> {
        self.resource_tags.as_deref()
    }
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not in scope of the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are in scope of the policy.</p>
    pub fn exclude_resource_tags(&self) -> bool {
        self.exclude_resource_tags
    }
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    pub fn remediation_enabled(&self) -> bool {
        self.remediation_enabled
    }
    /// <p>Indicates whether Firewall Manager should automatically remove protections from resources that leave the policy scope and clean up resources that Firewall Manager is managing for accounts when those accounts leave policy scope. For example, Firewall Manager will disassociate a Firewall Manager managed web ACL from a protected customer resource when the customer resource leaves policy scope. </p>
    /// <p>By default, Firewall Manager doesn't remove protections or delete Firewall Manager managed resources. </p>
    /// <p>This option is not available for Shield Advanced or WAF Classic policies.</p>
    pub fn delete_unused_fm_managed_resources(&self) -> bool {
        self.delete_unused_fm_managed_resources
    }
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to include in the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn include_map(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    > {
        self.include_map.as_ref()
    }
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to exclude from the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn exclude_map(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    > {
        self.exclude_map.as_ref()
    }
    /// <p>The unique identifiers of the resource sets used by the policy.</p>
    pub fn resource_set_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.resource_set_ids.as_deref()
    }
    /// <p>The definition of the Network Firewall firewall policy.</p>
    pub fn policy_description(&self) -> std::option::Option<&str> {
        self.policy_description.as_deref()
    }
}
impl Policy {
    /// Creates a new builder-style object to manufacture [`Policy`](crate::types::Policy).
    pub fn builder() -> crate::types::builders::PolicyBuilder {
        crate::types::builders::PolicyBuilder::default()
    }
}

/// A builder for [`Policy`](crate::types::Policy).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct PolicyBuilder {
    pub(crate) policy_id: std::option::Option<std::string::String>,
    pub(crate) policy_name: std::option::Option<std::string::String>,
    pub(crate) policy_update_token: std::option::Option<std::string::String>,
    pub(crate) security_service_policy_data:
        std::option::Option<crate::types::SecurityServicePolicyData>,
    pub(crate) resource_type: std::option::Option<std::string::String>,
    pub(crate) resource_type_list: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) resource_tags: std::option::Option<std::vec::Vec<crate::types::ResourceTag>>,
    pub(crate) exclude_resource_tags: std::option::Option<bool>,
    pub(crate) remediation_enabled: std::option::Option<bool>,
    pub(crate) delete_unused_fm_managed_resources: std::option::Option<bool>,
    pub(crate) include_map: std::option::Option<
        std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    >,
    pub(crate) exclude_map: std::option::Option<
        std::collections::HashMap<
            crate::types::CustomerPolicyScopeIdType,
            std::vec::Vec<std::string::String>,
        >,
    >,
    pub(crate) resource_set_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) policy_description: std::option::Option<std::string::String>,
}
impl PolicyBuilder {
    /// <p>The ID of the Firewall Manager policy.</p>
    pub fn policy_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_id = Some(input.into());
        self
    }
    /// <p>The ID of the Firewall Manager policy.</p>
    pub fn set_policy_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_id = input;
        self
    }
    /// <p>The name of the Firewall Manager policy.</p>
    pub fn policy_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_name = Some(input.into());
        self
    }
    /// <p>The name of the Firewall Manager policy.</p>
    pub fn set_policy_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>A unique identifier for each update to the policy. When issuing a <code>PutPolicy</code> request, the <code>PolicyUpdateToken</code> in the request must match the <code>PolicyUpdateToken</code> of the current policy version. To get the <code>PolicyUpdateToken</code> of the current policy version, use a <code>GetPolicy</code> request.</p>
    pub fn policy_update_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_update_token = Some(input.into());
        self
    }
    /// <p>A unique identifier for each update to the policy. When issuing a <code>PutPolicy</code> request, the <code>PolicyUpdateToken</code> in the request must match the <code>PolicyUpdateToken</code> of the current policy version. To get the <code>PolicyUpdateToken</code> of the current policy version, use a <code>GetPolicy</code> request.</p>
    pub fn set_policy_update_token(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.policy_update_token = input;
        self
    }
    /// <p>Details about the security service that is being used to protect the resources.</p>
    pub fn security_service_policy_data(
        mut self,
        input: crate::types::SecurityServicePolicyData,
    ) -> Self {
        self.security_service_policy_data = Some(input);
        self
    }
    /// <p>Details about the security service that is being used to protect the resources.</p>
    pub fn set_security_service_policy_data(
        mut self,
        input: std::option::Option<crate::types::SecurityServicePolicyData>,
    ) -> Self {
        self.security_service_policy_data = input;
        self
    }
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services Resource Types Reference</a>. To apply this policy to multiple resource types, specify a resource type of <code>ResourceTypeList</code> and then specify the resource types in a <code>ResourceTypeList</code>.</p>
    /// <p>For WAF and Shield Advanced, resource types include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code>, <code>AWS::ElasticLoadBalancing::LoadBalancer</code>, <code>AWS::EC2::EIP</code>, and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an Network Firewall policy or DNS Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    pub fn resource_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_type = Some(input.into());
        self
    }
    /// <p>The type of resource protected by or in scope of the policy. This is in the format shown in the <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-template-resource-type-ref.html">Amazon Web Services Resource Types Reference</a>. To apply this policy to multiple resource types, specify a resource type of <code>ResourceTypeList</code> and then specify the resource types in a <code>ResourceTypeList</code>.</p>
    /// <p>For WAF and Shield Advanced, resource types include <code>AWS::ElasticLoadBalancingV2::LoadBalancer</code>, <code>AWS::ElasticLoadBalancing::LoadBalancer</code>, <code>AWS::EC2::EIP</code>, and <code>AWS::CloudFront::Distribution</code>. For a security group common policy, valid values are <code>AWS::EC2::NetworkInterface</code> and <code>AWS::EC2::Instance</code>. For a security group content audit policy, valid values are <code>AWS::EC2::SecurityGroup</code>, <code>AWS::EC2::NetworkInterface</code>, and <code>AWS::EC2::Instance</code>. For a security group usage audit policy, the value is <code>AWS::EC2::SecurityGroup</code>. For an Network Firewall policy or DNS Firewall policy, the value is <code>AWS::EC2::VPC</code>.</p>
    pub fn set_resource_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `resource_type_list`.
    ///
    /// To override the contents of this collection use [`set_resource_type_list`](Self::set_resource_type_list).
    ///
    /// <p>An array of <code>ResourceType</code> objects. Use this only to specify multiple resource types. To specify a single resource type, use <code>ResourceType</code>.</p>
    pub fn resource_type_list(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resource_type_list.unwrap_or_default();
        v.push(input.into());
        self.resource_type_list = Some(v);
        self
    }
    /// <p>An array of <code>ResourceType</code> objects. Use this only to specify multiple resource types. To specify a single resource type, use <code>ResourceType</code>.</p>
    pub fn set_resource_type_list(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resource_type_list = input;
        self
    }
    /// Appends an item to `resource_tags`.
    ///
    /// To override the contents of this collection use [`set_resource_tags`](Self::set_resource_tags).
    ///
    /// <p>An array of <code>ResourceTag</code> objects.</p>
    pub fn resource_tags(mut self, input: crate::types::ResourceTag) -> Self {
        let mut v = self.resource_tags.unwrap_or_default();
        v.push(input);
        self.resource_tags = Some(v);
        self
    }
    /// <p>An array of <code>ResourceTag</code> objects.</p>
    pub fn set_resource_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ResourceTag>>,
    ) -> Self {
        self.resource_tags = input;
        self
    }
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not in scope of the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are in scope of the policy.</p>
    pub fn exclude_resource_tags(mut self, input: bool) -> Self {
        self.exclude_resource_tags = Some(input);
        self
    }
    /// <p>If set to <code>True</code>, resources with the tags that are specified in the <code>ResourceTag</code> array are not in scope of the policy. If set to <code>False</code>, and the <code>ResourceTag</code> array is not null, only resources with the specified tags are in scope of the policy.</p>
    pub fn set_exclude_resource_tags(mut self, input: std::option::Option<bool>) -> Self {
        self.exclude_resource_tags = input;
        self
    }
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    pub fn remediation_enabled(mut self, input: bool) -> Self {
        self.remediation_enabled = Some(input);
        self
    }
    /// <p>Indicates if the policy should be automatically applied to new resources.</p>
    pub fn set_remediation_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.remediation_enabled = input;
        self
    }
    /// <p>Indicates whether Firewall Manager should automatically remove protections from resources that leave the policy scope and clean up resources that Firewall Manager is managing for accounts when those accounts leave policy scope. For example, Firewall Manager will disassociate a Firewall Manager managed web ACL from a protected customer resource when the customer resource leaves policy scope. </p>
    /// <p>By default, Firewall Manager doesn't remove protections or delete Firewall Manager managed resources. </p>
    /// <p>This option is not available for Shield Advanced or WAF Classic policies.</p>
    pub fn delete_unused_fm_managed_resources(mut self, input: bool) -> Self {
        self.delete_unused_fm_managed_resources = Some(input);
        self
    }
    /// <p>Indicates whether Firewall Manager should automatically remove protections from resources that leave the policy scope and clean up resources that Firewall Manager is managing for accounts when those accounts leave policy scope. For example, Firewall Manager will disassociate a Firewall Manager managed web ACL from a protected customer resource when the customer resource leaves policy scope. </p>
    /// <p>By default, Firewall Manager doesn't remove protections or delete Firewall Manager managed resources. </p>
    /// <p>This option is not available for Shield Advanced or WAF Classic policies.</p>
    pub fn set_delete_unused_fm_managed_resources(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.delete_unused_fm_managed_resources = input;
        self
    }
    /// Adds a key-value pair to `include_map`.
    ///
    /// To override the contents of this collection use [`set_include_map`](Self::set_include_map).
    ///
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to include in the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn include_map(
        mut self,
        k: crate::types::CustomerPolicyScopeIdType,
        v: std::vec::Vec<std::string::String>,
    ) -> Self {
        let mut hash_map = self.include_map.unwrap_or_default();
        hash_map.insert(k, v);
        self.include_map = Some(hash_map);
        self
    }
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to include in the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn set_include_map(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<
                crate::types::CustomerPolicyScopeIdType,
                std::vec::Vec<std::string::String>,
            >,
        >,
    ) -> Self {
        self.include_map = input;
        self
    }
    /// Adds a key-value pair to `exclude_map`.
    ///
    /// To override the contents of this collection use [`set_exclude_map`](Self::set_exclude_map).
    ///
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to exclude from the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn exclude_map(
        mut self,
        k: crate::types::CustomerPolicyScopeIdType,
        v: std::vec::Vec<std::string::String>,
    ) -> Self {
        let mut hash_map = self.exclude_map.unwrap_or_default();
        hash_map.insert(k, v);
        self.exclude_map = Some(hash_map);
        self
    }
    /// <p>Specifies the Amazon Web Services account IDs and Organizations organizational units (OUs) to exclude from the policy. Specifying an OU is the equivalent of specifying all accounts in the OU and in any of its child OUs, including any child OUs and accounts that are added at a later time.</p>
    /// <p>You can specify inclusions or exclusions, but not both. If you specify an <code>IncludeMap</code>, Firewall Manager applies the policy to all accounts specified by the <code>IncludeMap</code>, and does not evaluate any <code>ExcludeMap</code> specifications. If you do not specify an <code>IncludeMap</code>, then Firewall Manager applies the policy to all accounts except for those specified by the <code>ExcludeMap</code>.</p>
    /// <p>You can specify account IDs, OUs, or a combination: </p>
    /// <ul>
    /// <li> <p>Specify account IDs by setting the key to <code>ACCOUNT</code>. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”]}</code>.</p> </li>
    /// <li> <p>Specify OUs by setting the key to <code>ORG_UNIT</code>. For example, the following is a valid map: <code>{“ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// <li> <p>Specify accounts and OUs together in a single map, separated with a comma. For example, the following is a valid map: <code>{“ACCOUNT” : [“accountID1”, “accountID2”], “ORG_UNIT” : [“ouid111”, “ouid112”]}</code>.</p> </li>
    /// </ul>
    pub fn set_exclude_map(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<
                crate::types::CustomerPolicyScopeIdType,
                std::vec::Vec<std::string::String>,
            >,
        >,
    ) -> Self {
        self.exclude_map = input;
        self
    }
    /// Appends an item to `resource_set_ids`.
    ///
    /// To override the contents of this collection use [`set_resource_set_ids`](Self::set_resource_set_ids).
    ///
    /// <p>The unique identifiers of the resource sets used by the policy.</p>
    pub fn resource_set_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.resource_set_ids.unwrap_or_default();
        v.push(input.into());
        self.resource_set_ids = Some(v);
        self
    }
    /// <p>The unique identifiers of the resource sets used by the policy.</p>
    pub fn set_resource_set_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.resource_set_ids = input;
        self
    }
    /// <p>The definition of the Network Firewall firewall policy.</p>
    pub fn policy_description(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy_description = Some(input.into());
        self
    }
    /// <p>The definition of the Network Firewall firewall policy.</p>
    pub fn set_policy_description(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.policy_description = input;
        self
    }
    /// Consumes the builder and constructs a [`Policy`](crate::types::Policy).
    pub fn build(self) -> crate::types::Policy {
        crate::types::Policy {
            policy_id: self.policy_id,
            policy_name: self.policy_name,
            policy_update_token: self.policy_update_token,
            security_service_policy_data: self.security_service_policy_data,
            resource_type: self.resource_type,
            resource_type_list: self.resource_type_list,
            resource_tags: self.resource_tags,
            exclude_resource_tags: self.exclude_resource_tags.unwrap_or_default(),
            remediation_enabled: self.remediation_enabled.unwrap_or_default(),
            delete_unused_fm_managed_resources: self
                .delete_unused_fm_managed_resources
                .unwrap_or_default(),
            include_map: self.include_map,
            exclude_map: self.exclude_map,
            resource_set_ids: self.resource_set_ids,
            policy_description: self.policy_description,
        }
    }
}
