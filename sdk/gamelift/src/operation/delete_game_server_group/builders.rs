// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_game_server_group::_delete_game_server_group_output::DeleteGameServerGroupOutputBuilder;

pub use crate::operation::delete_game_server_group::_delete_game_server_group_input::DeleteGameServerGroupInputBuilder;

/// Fluent builder constructing a request to `DeleteGameServerGroup`.
///
/// <p> <b>This operation is used with the GameLift FleetIQ solution and game server groups.</b> </p>
/// <p>Terminates a game server group and permanently deletes the game server group record. You have several options for how these resources are impacted when deleting the game server group. Depending on the type of delete operation selected, this operation might affect these resources:</p>
/// <ul>
/// <li> <p>The game server group</p> </li>
/// <li> <p>The corresponding Auto Scaling group</p> </li>
/// <li> <p>All game servers that are currently running in the group</p> </li>
/// </ul>
/// <p>To delete a game server group, identify the game server group to delete and specify the type of delete operation to initiate. Game server groups can only be deleted if they are in <code>ACTIVE</code> or <code>ERROR</code> status.</p>
/// <p>If the delete request is successful, a series of operations are kicked off. The game server group status is changed to <code>DELETE_SCHEDULED</code>, which prevents new game servers from being registered and stops automatic scaling activity. Once all game servers in the game server group are deregistered, GameLift FleetIQ can begin deleting resources. If any of the delete operations fail, the game server group is placed in <code>ERROR</code> status.</p>
/// <p>GameLift FleetIQ emits delete events to Amazon CloudWatch.</p>
/// <p> <b>Learn more</b> </p>
/// <p> <a href="https://docs.aws.amazon.com/gamelift/latest/fleetiqguide/gsg-intro.html">GameLift FleetIQ Guide</a> </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteGameServerGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_game_server_group::builders::DeleteGameServerGroupInputBuilder,
}
impl DeleteGameServerGroupFluentBuilder {
    /// Creates a new `DeleteGameServerGroup`.
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
            crate::operation::delete_game_server_group::DeleteGameServerGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_game_server_group::DeleteGameServerGroupError,
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
        crate::operation::delete_game_server_group::DeleteGameServerGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_game_server_group::DeleteGameServerGroupError,
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
    /// <p>A unique identifier for the game server group. Use either the name or ARN value.</p>
    pub fn game_server_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.game_server_group_name(input.into());
        self
    }
    /// <p>A unique identifier for the game server group. Use either the name or ARN value.</p>
    pub fn set_game_server_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_game_server_group_name(input);
        self
    }
    /// <p>The type of delete to perform. Options include the following:</p>
    /// <ul>
    /// <li> <p> <code>SAFE_DELETE</code> – (default) Terminates the game server group and Amazon EC2 Auto Scaling group only when it has no game servers that are in <code>UTILIZED</code> status.</p> </li>
    /// <li> <p> <code>FORCE_DELETE</code> – Terminates the game server group, including all active game servers regardless of their utilization status, and the Amazon EC2 Auto Scaling group. </p> </li>
    /// <li> <p> <code>RETAIN</code> – Does a safe delete of the game server group but retains the Amazon EC2 Auto Scaling group as is.</p> </li>
    /// </ul>
    pub fn delete_option(mut self, input: crate::types::GameServerGroupDeleteOption) -> Self {
        self.inner = self.inner.delete_option(input);
        self
    }
    /// <p>The type of delete to perform. Options include the following:</p>
    /// <ul>
    /// <li> <p> <code>SAFE_DELETE</code> – (default) Terminates the game server group and Amazon EC2 Auto Scaling group only when it has no game servers that are in <code>UTILIZED</code> status.</p> </li>
    /// <li> <p> <code>FORCE_DELETE</code> – Terminates the game server group, including all active game servers regardless of their utilization status, and the Amazon EC2 Auto Scaling group. </p> </li>
    /// <li> <p> <code>RETAIN</code> – Does a safe delete of the game server group but retains the Amazon EC2 Auto Scaling group as is.</p> </li>
    /// </ul>
    pub fn set_delete_option(
        mut self,
        input: std::option::Option<crate::types::GameServerGroupDeleteOption>,
    ) -> Self {
        self.inner = self.inner.set_delete_option(input);
        self
    }
}
