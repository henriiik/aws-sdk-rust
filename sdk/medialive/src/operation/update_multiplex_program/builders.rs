// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_multiplex_program::_update_multiplex_program_output::UpdateMultiplexProgramOutputBuilder;

pub use crate::operation::update_multiplex_program::_update_multiplex_program_input::UpdateMultiplexProgramInputBuilder;

/// Fluent builder constructing a request to `UpdateMultiplexProgram`.
///
/// Update a program in a multiplex.
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMultiplexProgramFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_multiplex_program::builders::UpdateMultiplexProgramInputBuilder,
}
impl UpdateMultiplexProgramFluentBuilder {
    /// Creates a new `UpdateMultiplexProgram`.
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
            crate::operation::update_multiplex_program::UpdateMultiplexProgram,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_multiplex_program::UpdateMultiplexProgramError,
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
        crate::operation::update_multiplex_program::UpdateMultiplexProgramOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_multiplex_program::UpdateMultiplexProgramError,
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
    /// The ID of the multiplex of the program to update.
    pub fn multiplex_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.multiplex_id(input.into());
        self
    }
    /// The ID of the multiplex of the program to update.
    pub fn set_multiplex_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_multiplex_id(input);
        self
    }
    /// The new settings for a multiplex program.
    pub fn multiplex_program_settings(
        mut self,
        input: crate::types::MultiplexProgramSettings,
    ) -> Self {
        self.inner = self.inner.multiplex_program_settings(input);
        self
    }
    /// The new settings for a multiplex program.
    pub fn set_multiplex_program_settings(
        mut self,
        input: std::option::Option<crate::types::MultiplexProgramSettings>,
    ) -> Self {
        self.inner = self.inner.set_multiplex_program_settings(input);
        self
    }
    /// The name of the program to update.
    pub fn program_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.program_name(input.into());
        self
    }
    /// The name of the program to update.
    pub fn set_program_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_program_name(input);
        self
    }
}
