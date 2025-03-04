// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bot::_update_bot_output::UpdateBotOutputBuilder;

pub use crate::operation::update_bot::_update_bot_input::UpdateBotInputBuilder;

/// Fluent builder constructing a request to `UpdateBot`.
///
/// <p>Updates the configuration of an existing bot. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateBotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bot::builders::UpdateBotInputBuilder,
}
impl UpdateBotFluentBuilder {
    /// Creates a new `UpdateBot`.
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
            crate::operation::update_bot::UpdateBot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_bot::UpdateBotError>,
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
        crate::operation::update_bot::UpdateBotOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_bot::UpdateBotError>,
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
    /// <p>The unique identifier of the bot to update. This identifier is returned by the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_CreateBot.html">CreateBot</a> operation.</p>
    pub fn bot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot to update. This identifier is returned by the <a href="https://docs.aws.amazon.com/lexv2/latest/APIReference/API_CreateBot.html">CreateBot</a> operation.</p>
    pub fn set_bot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The new name of the bot. The name must be unique in the account that creates the bot.</p>
    pub fn bot_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bot_name(input.into());
        self
    }
    /// <p>The new name of the bot. The name must be unique in the account that creates the bot.</p>
    pub fn set_bot_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bot_name(input);
        self
    }
    /// <p>A description of the bot.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the bot.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the bot.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the bot.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>Provides information on additional privacy protections Amazon Lex should use with the bot's data.</p>
    pub fn data_privacy(mut self, input: crate::types::DataPrivacy) -> Self {
        self.inner = self.inner.data_privacy(input);
        self
    }
    /// <p>Provides information on additional privacy protections Amazon Lex should use with the bot's data.</p>
    pub fn set_data_privacy(
        mut self,
        input: std::option::Option<crate::types::DataPrivacy>,
    ) -> Self {
        self.inner = self.inner.set_data_privacy(input);
        self
    }
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot.</p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    pub fn idle_session_ttl_in_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.idle_session_ttl_in_seconds(input);
        self
    }
    /// <p>The time, in seconds, that Amazon Lex should keep information about a user's conversation with the bot.</p>
    /// <p>A user interaction remains active for the amount of time specified. If no conversation occurs during this time, the session expires and Amazon Lex deletes any data provided before the timeout.</p>
    /// <p>You can specify between 60 (1 minute) and 86,400 (24 hours) seconds.</p>
    pub fn set_idle_session_ttl_in_seconds(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_idle_session_ttl_in_seconds(input);
        self
    }
    /// <p>The type of the bot to be updated.</p>
    pub fn bot_type(mut self, input: crate::types::BotType) -> Self {
        self.inner = self.inner.bot_type(input);
        self
    }
    /// <p>The type of the bot to be updated.</p>
    pub fn set_bot_type(mut self, input: std::option::Option<crate::types::BotType>) -> Self {
        self.inner = self.inner.set_bot_type(input);
        self
    }
    /// Appends an item to `botMembers`.
    ///
    /// To override the contents of this collection use [`set_bot_members`](Self::set_bot_members).
    ///
    /// <p>The list of bot members in the network associated with the update action.</p>
    pub fn bot_members(mut self, input: crate::types::BotMember) -> Self {
        self.inner = self.inner.bot_members(input);
        self
    }
    /// <p>The list of bot members in the network associated with the update action.</p>
    pub fn set_bot_members(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::BotMember>>,
    ) -> Self {
        self.inner = self.inner.set_bot_members(input);
        self
    }
}
