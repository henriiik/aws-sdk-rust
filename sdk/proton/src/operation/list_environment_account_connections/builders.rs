// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_environment_account_connections::_list_environment_account_connections_output::ListEnvironmentAccountConnectionsOutputBuilder;

pub use crate::operation::list_environment_account_connections::_list_environment_account_connections_input::ListEnvironmentAccountConnectionsInputBuilder;

/// Fluent builder constructing a request to `ListEnvironmentAccountConnections`.
///
/// <p>View a list of environment account connections.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/proton/latest/userguide/ag-env-account-connections.html">Environment account connections</a> in the <i>Proton User guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListEnvironmentAccountConnectionsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::list_environment_account_connections::builders::ListEnvironmentAccountConnectionsInputBuilder
            }
impl ListEnvironmentAccountConnectionsFluentBuilder {
    /// Creates a new `ListEnvironmentAccountConnections`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::list_environment_account_connections::ListEnvironmentAccountConnections, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::list_environment_account_connections::ListEnvironmentAccountConnectionsError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::list_environment_account_connections::ListEnvironmentAccountConnectionsOutput, aws_smithy_http::result::SdkError<crate::operation::list_environment_account_connections::ListEnvironmentAccountConnectionsError>>
                     {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_environment_account_connections::paginator::ListEnvironmentAccountConnectionsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_environment_account_connections::paginator::ListEnvironmentAccountConnectionsPaginator{
        crate::operation::list_environment_account_connections::paginator::ListEnvironmentAccountConnectionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The type of account making the <code>ListEnvironmentAccountConnections</code> request.</p>
    pub fn requested_by(
        mut self,
        input: crate::types::EnvironmentAccountConnectionRequesterAccountType,
    ) -> Self {
        self.inner = self.inner.requested_by(input);
        self
    }
    /// <p>The type of account making the <code>ListEnvironmentAccountConnections</code> request.</p>
    pub fn set_requested_by(
        mut self,
        input: std::option::Option<crate::types::EnvironmentAccountConnectionRequesterAccountType>,
    ) -> Self {
        self.inner = self.inner.set_requested_by(input);
        self
    }
    /// <p>The environment name that's associated with each listed environment account connection.</p>
    pub fn environment_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>The environment name that's associated with each listed environment account connection.</p>
    pub fn set_environment_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
    /// Appends an item to `statuses`.
    ///
    /// To override the contents of this collection use [`set_statuses`](Self::set_statuses).
    ///
    /// <p>The status details for each listed environment account connection.</p>
    pub fn statuses(mut self, input: crate::types::EnvironmentAccountConnectionStatus) -> Self {
        self.inner = self.inner.statuses(input);
        self
    }
    /// <p>The status details for each listed environment account connection.</p>
    pub fn set_statuses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::EnvironmentAccountConnectionStatus>>,
    ) -> Self {
        self.inner = self.inner.set_statuses(input);
        self
    }
    /// <p>A token that indicates the location of the next environment account connection in the array of environment account connections, after the list of environment account connections that was previously requested.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates the location of the next environment account connection in the array of environment account connections, after the list of environment account connections that was previously requested.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of environment account connections to list.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of environment account connections to list.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
