// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_aws_organizations_access::_disable_aws_organizations_access_output::DisableAwsOrganizationsAccessOutputBuilder;

pub use crate::operation::disable_aws_organizations_access::_disable_aws_organizations_access_input::DisableAwsOrganizationsAccessInputBuilder;

/// Fluent builder constructing a request to `DisableAWSOrganizationsAccess`.
///
/// <p>Disable portfolio sharing through the Organizations service. This command will not delete your current shares, but prevents you from creating new shares throughout your organization. Current shares are not kept in sync with your organization structure if the structure changes after calling this API. Only the management account in the organization can call this API.</p>
/// <p>You cannot call this API if there are active delegated administrators in the organization.</p>
/// <p>Note that a delegated administrator is not authorized to invoke <code>DisableAWSOrganizationsAccess</code>.</p> <important>
/// <p>If you share an Service Catalog portfolio in an organization within Organizations, and then disable Organizations access for Service Catalog, the portfolio access permissions will not sync with the latest changes to the organization structure. Specifically, accounts that you removed from the organization after disabling Service Catalog access will retain access to the previously shared portfolio.</p>
/// </important>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DisableAWSOrganizationsAccessFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::disable_aws_organizations_access::builders::DisableAwsOrganizationsAccessInputBuilder
            }
impl DisableAWSOrganizationsAccessFluentBuilder {
    /// Creates a new `DisableAWSOrganizationsAccess`.
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
            crate::operation::disable_aws_organizations_access::DisableAWSOrganizationsAccess,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_aws_organizations_access::DisableAWSOrganizationsAccessError,
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
        crate::operation::disable_aws_organizations_access::DisableAwsOrganizationsAccessOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::disable_aws_organizations_access::DisableAWSOrganizationsAccessError,
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
}
