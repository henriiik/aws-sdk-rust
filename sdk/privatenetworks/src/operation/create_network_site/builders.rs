// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_network_site::_create_network_site_output::CreateNetworkSiteOutputBuilder;

pub use crate::operation::create_network_site::_create_network_site_input::CreateNetworkSiteInputBuilder;

/// Fluent builder constructing a request to `CreateNetworkSite`.
///
/// <p>Creates a network site.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateNetworkSiteFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_network_site::builders::CreateNetworkSiteInputBuilder,
}
impl CreateNetworkSiteFluentBuilder {
    /// Creates a new `CreateNetworkSite`.
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
            crate::operation::create_network_site::CreateNetworkSite,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_network_site::CreateNetworkSiteError,
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
        crate::operation::create_network_site::CreateNetworkSiteOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_network_site::CreateNetworkSiteError,
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
    /// <p>The name of the site. You can't change the name after you create the site.</p>
    pub fn network_site_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_site_name(input.into());
        self
    }
    /// <p>The name of the site. You can't change the name after you create the site.</p>
    pub fn set_network_site_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_network_site_name(input);
        self
    }
    /// <p>The description of the site.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the site.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the network.</p>
    pub fn network_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.network_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the network.</p>
    pub fn set_network_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_network_arn(input);
        self
    }
    /// <p>Information about the pending plan for this site.</p>
    pub fn pending_plan(mut self, input: crate::types::SitePlan) -> Self {
        self.inner = self.inner.pending_plan(input);
        self
    }
    /// <p>Information about the pending plan for this site.</p>
    pub fn set_pending_plan(mut self, input: std::option::Option<crate::types::SitePlan>) -> Self {
        self.inner = self.inner.set_pending_plan(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The Availability Zone that is the parent of this site. You can't change the Availability Zone after you create the site.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone that is the parent of this site. You can't change the Availability Zone after you create the site.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The ID of the Availability Zone that is the parent of this site. You can't change the Availability Zone after you create the site.</p>
    pub fn availability_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_id(input.into());
        self
    }
    /// <p>The ID of the Availability Zone that is the parent of this site. You can't change the Availability Zone after you create the site.</p>
    pub fn set_availability_zone_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone_id(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> The tags to apply to the network site. </p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p> The tags to apply to the network site. </p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
