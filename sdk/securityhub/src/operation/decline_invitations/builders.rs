// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::decline_invitations::_decline_invitations_output::DeclineInvitationsOutputBuilder;

pub use crate::operation::decline_invitations::_decline_invitations_input::DeclineInvitationsInputBuilder;

/// Fluent builder constructing a request to `DeclineInvitations`.
///
/// <p>Declines invitations to become a member account.</p>
/// <p>A prospective member account uses this operation to decline an invitation to become a member.</p>
/// <p>This operation is only called by member accounts that aren't part of an organization. Organization accounts don't receive invitations.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeclineInvitationsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::decline_invitations::builders::DeclineInvitationsInputBuilder,
}
impl DeclineInvitationsFluentBuilder {
    /// Creates a new `DeclineInvitations`.
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
            crate::operation::decline_invitations::DeclineInvitations,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::decline_invitations::DeclineInvitationsError,
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
        crate::operation::decline_invitations::DeclineInvitationsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::decline_invitations::DeclineInvitationsError,
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
    /// Appends an item to `AccountIds`.
    ///
    /// To override the contents of this collection use [`set_account_ids`](Self::set_account_ids).
    ///
    /// <p>The list of prospective member account IDs for which to decline an invitation.</p>
    pub fn account_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_ids(input.into());
        self
    }
    /// <p>The list of prospective member account IDs for which to decline an invitation.</p>
    pub fn set_account_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_account_ids(input);
        self
    }
}
