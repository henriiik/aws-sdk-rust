// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_retained_messages::_list_retained_messages_output::ListRetainedMessagesOutputBuilder;

pub use crate::operation::list_retained_messages::_list_retained_messages_input::ListRetainedMessagesInputBuilder;

/// Fluent builder constructing a request to `ListRetainedMessages`.
///
/// <p>Lists summary information about the retained messages stored for the account.</p>
/// <p>This action returns only the topic names of the retained messages. It doesn't return any message payloads. Although this action doesn't return a message payload, it can still incur messaging costs.</p>
/// <p>To get the message payload of a retained message, call <a href="https://docs.aws.amazon.com/iot/latest/apireference/API_iotdata_GetRetainedMessage.html">GetRetainedMessage</a> with the topic name of the retained message.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiotfleethubfordevicemanagement.html#awsiotfleethubfordevicemanagement-actions-as-permissions">ListRetainedMessages</a> action.</p>
/// <p>For more information about messaging costs, see <a href="http://aws.amazon.com/iot-core/pricing/#Messaging">Amazon Web Services IoT Core pricing - Messaging</a>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListRetainedMessagesFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_retained_messages::builders::ListRetainedMessagesInputBuilder,
}
impl ListRetainedMessagesFluentBuilder {
    /// Creates a new `ListRetainedMessages`.
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
            crate::operation::list_retained_messages::ListRetainedMessages,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_retained_messages::ListRetainedMessagesError,
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
        crate::operation::list_retained_messages::ListRetainedMessagesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_retained_messages::ListRetainedMessagesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_retained_messages::paginator::ListRetainedMessagesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_retained_messages::paginator::ListRetainedMessagesPaginator {
        crate::operation::list_retained_messages::paginator::ListRetainedMessagesPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
