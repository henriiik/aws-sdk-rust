// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_organization_admin_account::_disable_organization_admin_account_output::DisableOrganizationAdminAccountOutputBuilder;

pub use crate::operation::disable_organization_admin_account::_disable_organization_admin_account_input::DisableOrganizationAdminAccountInputBuilder;

/// Fluent builder constructing a request to `DisableOrganizationAdminAccount`.
///
/// <p>Removes the Detective administrator account in the current Region. Deletes the organization behavior graph.</p>
/// <p>Can only be called by the organization management account.</p>
/// <p>Removing the Detective administrator account does not affect the delegated administrator account for Detective in Organizations.</p>
/// <p>To remove the delegated administrator account in Organizations, use the Organizations API. Removing the delegated administrator account also removes the Detective administrator account in all Regions, except for Regions where the Detective administrator account is the organization management account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableOrganizationAdminAccountFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disable_organization_admin_account::builders::DisableOrganizationAdminAccountInputBuilder
            }
impl DisableOrganizationAdminAccountFluentBuilder {
    /// Creates a new `DisableOrganizationAdminAccount`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::disable_organization_admin_account::DisableOrganizationAdminAccount, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::disable_organization_admin_account::DisableOrganizationAdminAccountError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::disable_organization_admin_account::DisableOrganizationAdminAccountOutput, aws_smithy_http::result::SdkError<crate::operation::disable_organization_admin_account::DisableOrganizationAdminAccountError>>
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
}
