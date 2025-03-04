// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_profile_key::_delete_profile_key_output::DeleteProfileKeyOutputBuilder;

pub use crate::operation::delete_profile_key::_delete_profile_key_input::DeleteProfileKeyInputBuilder;

/// Fluent builder constructing a request to `DeleteProfileKey`.
///
/// <p>Removes a searchable key from a customer profile.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteProfileKeyFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_profile_key::builders::DeleteProfileKeyInputBuilder,
}
impl DeleteProfileKeyFluentBuilder {
    /// Creates a new `DeleteProfileKey`.
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
            crate::operation::delete_profile_key::DeleteProfileKey,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_profile_key::DeleteProfileKeyError,
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
        crate::operation::delete_profile_key::DeleteProfileKeyOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_profile_key::DeleteProfileKeyError,
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
    /// <p>The unique identifier of a customer profile.</p>
    pub fn profile_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.profile_id(input.into());
        self
    }
    /// <p>The unique identifier of a customer profile.</p>
    pub fn set_profile_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_profile_id(input);
        self
    }
    /// <p>A searchable identifier of a customer profile.</p>
    pub fn key_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>A searchable identifier of a customer profile.</p>
    pub fn set_key_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// Appends an item to `Values`.
    ///
    /// To override the contents of this collection use [`set_values`](Self::set_values).
    ///
    /// <p>A list of key values.</p>
    pub fn values(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.values(input.into());
        self
    }
    /// <p>A list of key values.</p>
    pub fn set_values(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_values(input);
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
}
