// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connection::_update_connection_output::UpdateConnectionOutputBuilder;

pub use crate::operation::update_connection::_update_connection_input::UpdateConnectionInputBuilder;

/// Fluent builder constructing a request to `UpdateConnection`.
///
/// <p>Updates the Direct Connect dedicated connection configuration.</p>
/// <p>You can update the following parameters for a connection:</p>
/// <ul>
/// <li> <p>The connection name</p> </li>
/// <li> <p>The connection's MAC Security (MACsec) encryption mode.</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateConnectionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connection::builders::UpdateConnectionInputBuilder,
}
impl UpdateConnectionFluentBuilder {
    /// Creates a new `UpdateConnection`.
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
            crate::operation::update_connection::UpdateConnection,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_connection::UpdateConnectionError,
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
        crate::operation::update_connection::UpdateConnectionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_connection::UpdateConnectionError,
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
    /// <p>The ID of the dedicated connection.</p>
    /// <p>You can use <code>DescribeConnections</code> to retrieve the connection ID.</p>
    pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_id(input.into());
        self
    }
    /// <p>The ID of the dedicated connection.</p>
    /// <p>You can use <code>DescribeConnections</code> to retrieve the connection ID.</p>
    pub fn set_connection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_connection_id(input);
        self
    }
    /// <p>The name of the connection.</p>
    pub fn connection_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.connection_name(input.into());
        self
    }
    /// <p>The name of the connection.</p>
    pub fn set_connection_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_connection_name(input);
        self
    }
    /// <p>The connection MAC Security (MACsec) encryption mode.</p>
    /// <p>The valid values are <code>no_encrypt</code>, <code>should_encrypt</code>, and <code>must_encrypt</code>.</p>
    pub fn encryption_mode(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.encryption_mode(input.into());
        self
    }
    /// <p>The connection MAC Security (MACsec) encryption mode.</p>
    /// <p>The valid values are <code>no_encrypt</code>, <code>should_encrypt</code>, and <code>must_encrypt</code>.</p>
    pub fn set_encryption_mode(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_encryption_mode(input);
        self
    }
}
