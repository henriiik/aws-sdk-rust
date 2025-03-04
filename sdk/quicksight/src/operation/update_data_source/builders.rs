// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_data_source::_update_data_source_output::UpdateDataSourceOutputBuilder;

pub use crate::operation::update_data_source::_update_data_source_input::UpdateDataSourceInputBuilder;

/// Fluent builder constructing a request to `UpdateDataSource`.
///
/// <p>Updates a data source.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDataSourceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_data_source::builders::UpdateDataSourceInputBuilder,
}
impl UpdateDataSourceFluentBuilder {
    /// Creates a new `UpdateDataSource`.
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
            crate::operation::update_data_source::UpdateDataSource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
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
        crate::operation::update_data_source::UpdateDataSourceOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_data_source::UpdateDataSourceError,
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
    /// <p>The Amazon Web Services account ID.</p>
    pub fn aws_account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID.</p>
    pub fn set_aws_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the data source. This ID is unique per Amazon Web Services Region for each Amazon Web Services account. </p>
    pub fn data_source_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.data_source_id(input.into());
        self
    }
    /// <p>The ID of the data source. This ID is unique per Amazon Web Services Region for each Amazon Web Services account. </p>
    pub fn set_data_source_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_data_source_id(input);
        self
    }
    /// <p>A display name for the data source.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A display name for the data source.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The parameters that Amazon QuickSight uses to connect to your underlying source.</p>
    pub fn data_source_parameters(mut self, input: crate::types::DataSourceParameters) -> Self {
        self.inner = self.inner.data_source_parameters(input);
        self
    }
    /// <p>The parameters that Amazon QuickSight uses to connect to your underlying source.</p>
    pub fn set_data_source_parameters(
        mut self,
        input: std::option::Option<crate::types::DataSourceParameters>,
    ) -> Self {
        self.inner = self.inner.set_data_source_parameters(input);
        self
    }
    /// <p>The credentials that Amazon QuickSight that uses to connect to your underlying source. Currently, only credentials based on user name and password are supported.</p>
    pub fn credentials(mut self, input: crate::types::DataSourceCredentials) -> Self {
        self.inner = self.inner.credentials(input);
        self
    }
    /// <p>The credentials that Amazon QuickSight that uses to connect to your underlying source. Currently, only credentials based on user name and password are supported.</p>
    pub fn set_credentials(
        mut self,
        input: std::option::Option<crate::types::DataSourceCredentials>,
    ) -> Self {
        self.inner = self.inner.set_credentials(input);
        self
    }
    /// <p>Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source.</p>
    pub fn vpc_connection_properties(
        mut self,
        input: crate::types::VpcConnectionProperties,
    ) -> Self {
        self.inner = self.inner.vpc_connection_properties(input);
        self
    }
    /// <p>Use this parameter only when you want Amazon QuickSight to use a VPC connection when connecting to your underlying source.</p>
    pub fn set_vpc_connection_properties(
        mut self,
        input: std::option::Option<crate::types::VpcConnectionProperties>,
    ) -> Self {
        self.inner = self.inner.set_vpc_connection_properties(input);
        self
    }
    /// <p>Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source.</p>
    pub fn ssl_properties(mut self, input: crate::types::SslProperties) -> Self {
        self.inner = self.inner.ssl_properties(input);
        self
    }
    /// <p>Secure Socket Layer (SSL) properties that apply when Amazon QuickSight connects to your underlying source.</p>
    pub fn set_ssl_properties(
        mut self,
        input: std::option::Option<crate::types::SslProperties>,
    ) -> Self {
        self.inner = self.inner.set_ssl_properties(input);
        self
    }
}
