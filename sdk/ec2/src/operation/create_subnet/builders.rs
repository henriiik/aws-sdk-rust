// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_subnet::_create_subnet_output::CreateSubnetOutputBuilder;

pub use crate::operation::create_subnet::_create_subnet_input::CreateSubnetInputBuilder;

/// Fluent builder constructing a request to `CreateSubnet`.
///
/// <p>Creates a subnet in the specified VPC. For an IPv4 only subnet, specify an IPv4 CIDR block. If the VPC has an IPv6 CIDR block, you can create an IPv6 only subnet or a dual stack subnet instead. For an IPv6 only subnet, specify an IPv6 CIDR block. For a dual stack subnet, specify both an IPv4 CIDR block and an IPv6 CIDR block.</p>
/// <p>A subnet CIDR block must not overlap the CIDR block of an existing subnet in the VPC. After you create a subnet, you can't change its CIDR block.</p>
/// <p>The allowed size for an IPv4 subnet is between a /28 netmask (16 IP addresses) and a /16 netmask (65,536 IP addresses). Amazon Web Services reserves both the first four and the last IPv4 address in each subnet's CIDR block. They're not available for your use.</p>
/// <p>If you've associated an IPv6 CIDR block with your VPC, you can associate an IPv6 CIDR block with a subnet when you create it. The allowed block size for an IPv6 subnet is a /64 netmask.</p>
/// <p>If you add more than one subnet to a VPC, they're set up in a star topology with a logical router in the middle.</p>
/// <p>When you stop an instance in a subnet, it retains its private IPv4 address. It's therefore possible to have a subnet with no running instances (they're all stopped), but no remaining IP addresses available.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/configure-subnets.html">Subnets</a> in the <i>Amazon Virtual Private Cloud User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateSubnetFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_subnet::builders::CreateSubnetInputBuilder,
}
impl CreateSubnetFluentBuilder {
    /// Creates a new `CreateSubnet`.
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
            crate::operation::create_subnet::CreateSubnet,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_subnet::CreateSubnetError>,
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
        crate::operation::create_subnet::CreateSubnetOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_subnet::CreateSubnetError>,
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
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the subnet.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to assign to the subnet.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions">Available Regions</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions">Available Regions</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn availability_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_id(input.into());
        self
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn set_availability_zone_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone_id(input);
        self
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cidr_block(input.into());
        self
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn set_cidr_block(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cidr_block(input);
        self
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. The subnet size must use a /64 prefix length.</p>
    /// <p>This parameter is required for an IPv6 only subnet.</p>
    pub fn ipv6_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ipv6_cidr_block(input.into());
        self
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. The subnet size must use a /64 prefix length.</p>
    /// <p>This parameter is required for an IPv6 only subnet.</p>
    pub fn set_ipv6_cidr_block(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ipv6_cidr_block(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn outpost_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn set_outpost_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn ipv6_native(mut self, input: bool) -> Self {
        self.inner = self.inner.ipv6_native(input);
        self
    }
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn set_ipv6_native(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_ipv6_native(input);
        self
    }
}
