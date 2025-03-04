// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateConnectorProfileInput {
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    #[doc(hidden)]
    pub connector_profile_name: std::option::Option<std::string::String>,
    /// <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    #[doc(hidden)]
    pub kms_arn: std::option::Option<std::string::String>,
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    #[doc(hidden)]
    pub connector_type: std::option::Option<crate::types::ConnectorType>,
    /// <p>The label of the connector. The label is unique for each <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if calling for CUSTOMCONNECTOR connector type/.</p>
    #[doc(hidden)]
    pub connector_label: std::option::Option<std::string::String>,
    /// <p> Indicates the connection mode and specifies whether it is public or private. Private flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure without exposing it to the public internet. </p>
    #[doc(hidden)]
    pub connection_mode: std::option::Option<crate::types::ConnectionMode>,
    /// <p> Defines the connector-specific configuration and credentials. </p>
    #[doc(hidden)]
    pub connector_profile_config: std::option::Option<crate::types::ConnectorProfileConfig>,
}
impl CreateConnectorProfileInput {
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    pub fn connector_profile_name(&self) -> std::option::Option<&str> {
        self.connector_profile_name.as_deref()
    }
    /// <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    pub fn kms_arn(&self) -> std::option::Option<&str> {
        self.kms_arn.as_deref()
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(&self) -> std::option::Option<&crate::types::ConnectorType> {
        self.connector_type.as_ref()
    }
    /// <p>The label of the connector. The label is unique for each <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if calling for CUSTOMCONNECTOR connector type/.</p>
    pub fn connector_label(&self) -> std::option::Option<&str> {
        self.connector_label.as_deref()
    }
    /// <p> Indicates the connection mode and specifies whether it is public or private. Private flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure without exposing it to the public internet. </p>
    pub fn connection_mode(&self) -> std::option::Option<&crate::types::ConnectionMode> {
        self.connection_mode.as_ref()
    }
    /// <p> Defines the connector-specific configuration and credentials. </p>
    pub fn connector_profile_config(
        &self,
    ) -> std::option::Option<&crate::types::ConnectorProfileConfig> {
        self.connector_profile_config.as_ref()
    }
}
impl CreateConnectorProfileInput {
    /// Creates a new builder-style object to manufacture [`CreateConnectorProfileInput`](crate::operation::create_connector_profile::CreateConnectorProfileInput).
    pub fn builder(
    ) -> crate::operation::create_connector_profile::builders::CreateConnectorProfileInputBuilder
    {
        crate::operation::create_connector_profile::builders::CreateConnectorProfileInputBuilder::default()
    }
}

/// A builder for [`CreateConnectorProfileInput`](crate::operation::create_connector_profile::CreateConnectorProfileInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct CreateConnectorProfileInputBuilder {
    pub(crate) connector_profile_name: std::option::Option<std::string::String>,
    pub(crate) kms_arn: std::option::Option<std::string::String>,
    pub(crate) connector_type: std::option::Option<crate::types::ConnectorType>,
    pub(crate) connector_label: std::option::Option<std::string::String>,
    pub(crate) connection_mode: std::option::Option<crate::types::ConnectionMode>,
    pub(crate) connector_profile_config: std::option::Option<crate::types::ConnectorProfileConfig>,
}
impl CreateConnectorProfileInputBuilder {
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    pub fn connector_profile_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.connector_profile_name = Some(input.into());
        self
    }
    /// <p> The name of the connector profile. The name is unique for each <code>ConnectorProfile</code> in your Amazon Web Services account. </p>
    pub fn set_connector_profile_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.connector_profile_name = input;
        self
    }
    /// <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    pub fn kms_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.kms_arn = Some(input.into());
        self
    }
    /// <p> The ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key. </p>
    pub fn set_kms_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.kms_arn = input;
        self
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn connector_type(mut self, input: crate::types::ConnectorType) -> Self {
        self.connector_type = Some(input);
        self
    }
    /// <p> The type of connector, such as Salesforce, Amplitude, and so on. </p>
    pub fn set_connector_type(
        mut self,
        input: std::option::Option<crate::types::ConnectorType>,
    ) -> Self {
        self.connector_type = input;
        self
    }
    /// <p>The label of the connector. The label is unique for each <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if calling for CUSTOMCONNECTOR connector type/.</p>
    pub fn connector_label(mut self, input: impl Into<std::string::String>) -> Self {
        self.connector_label = Some(input.into());
        self
    }
    /// <p>The label of the connector. The label is unique for each <code>ConnectorRegistration</code> in your Amazon Web Services account. Only needed if calling for CUSTOMCONNECTOR connector type/.</p>
    pub fn set_connector_label(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.connector_label = input;
        self
    }
    /// <p> Indicates the connection mode and specifies whether it is public or private. Private flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure without exposing it to the public internet. </p>
    pub fn connection_mode(mut self, input: crate::types::ConnectionMode) -> Self {
        self.connection_mode = Some(input);
        self
    }
    /// <p> Indicates the connection mode and specifies whether it is public or private. Private flows use Amazon Web Services PrivateLink to route data over Amazon Web Services infrastructure without exposing it to the public internet. </p>
    pub fn set_connection_mode(
        mut self,
        input: std::option::Option<crate::types::ConnectionMode>,
    ) -> Self {
        self.connection_mode = input;
        self
    }
    /// <p> Defines the connector-specific configuration and credentials. </p>
    pub fn connector_profile_config(mut self, input: crate::types::ConnectorProfileConfig) -> Self {
        self.connector_profile_config = Some(input);
        self
    }
    /// <p> Defines the connector-specific configuration and credentials. </p>
    pub fn set_connector_profile_config(
        mut self,
        input: std::option::Option<crate::types::ConnectorProfileConfig>,
    ) -> Self {
        self.connector_profile_config = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateConnectorProfileInput`](crate::operation::create_connector_profile::CreateConnectorProfileInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_connector_profile::CreateConnectorProfileInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::create_connector_profile::CreateConnectorProfileInput {
                connector_profile_name: self.connector_profile_name,
                kms_arn: self.kms_arn,
                connector_type: self.connector_type,
                connector_label: self.connector_label,
                connection_mode: self.connection_mode,
                connector_profile_config: self.connector_profile_config,
            },
        )
    }
}
