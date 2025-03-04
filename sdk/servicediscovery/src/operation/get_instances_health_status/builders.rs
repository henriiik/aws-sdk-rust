// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_instances_health_status::_get_instances_health_status_output::GetInstancesHealthStatusOutputBuilder;

pub use crate::operation::get_instances_health_status::_get_instances_health_status_input::GetInstancesHealthStatusInputBuilder;

/// Fluent builder constructing a request to `GetInstancesHealthStatus`.
///
/// <p>Gets the current health status (<code>Healthy</code>, <code>Unhealthy</code>, or <code>Unknown</code>) of one or more instances that are associated with a specified service.</p> <note>
/// <p>There's a brief delay between when you register an instance and when the health status for the instance is available. </p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetInstancesHealthStatusFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_instances_health_status::builders::GetInstancesHealthStatusInputBuilder
            }
impl GetInstancesHealthStatusFluentBuilder {
    /// Creates a new `GetInstancesHealthStatus`.
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
            crate::operation::get_instances_health_status::GetInstancesHealthStatus,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_instances_health_status::GetInstancesHealthStatusError,
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
        crate::operation::get_instances_health_status::GetInstancesHealthStatusOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_instances_health_status::GetInstancesHealthStatusError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_instances_health_status::paginator::GetInstancesHealthStatusPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::get_instances_health_status::paginator::GetInstancesHealthStatusPaginator
    {
        crate::operation::get_instances_health_status::paginator::GetInstancesHealthStatusPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the service that the instance is associated with.</p>
    pub fn service_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_id(input.into());
        self
    }
    /// <p>The ID of the service that the instance is associated with.</p>
    pub fn set_service_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service_id(input);
        self
    }
    /// Appends an item to `Instances`.
    ///
    /// To override the contents of this collection use [`set_instances`](Self::set_instances).
    ///
    /// <p>An array that contains the IDs of all the instances that you want to get the health status for.</p>
    /// <p>If you omit <code>Instances</code>, Cloud Map returns the health status for all the instances that are associated with the specified service.</p> <note>
    /// <p>To get the IDs for the instances that you've registered by using a specified service, submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListInstances.html">ListInstances</a> request.</p>
    /// </note>
    pub fn instances(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.instances(input.into());
        self
    }
    /// <p>An array that contains the IDs of all the instances that you want to get the health status for.</p>
    /// <p>If you omit <code>Instances</code>, Cloud Map returns the health status for all the instances that are associated with the specified service.</p> <note>
    /// <p>To get the IDs for the instances that you've registered by using a specified service, submit a <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_ListInstances.html">ListInstances</a> request.</p>
    /// </note>
    pub fn set_instances(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_instances(input);
        self
    }
    /// <p>The maximum number of instances that you want Cloud Map to return in the response to a <code>GetInstancesHealthStatus</code> request. If you don't specify a value for <code>MaxResults</code>, Cloud Map returns up to 100 instances.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of instances that you want Cloud Map to return in the response to a <code>GetInstancesHealthStatus</code> request. If you don't specify a value for <code>MaxResults</code>, Cloud Map returns up to 100 instances.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>For the first <code>GetInstancesHealthStatus</code> request, omit this value.</p>
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>For the first <code>GetInstancesHealthStatus</code> request, omit this value.</p>
    /// <p>If more than <code>MaxResults</code> instances match the specified criteria, you can submit another <code>GetInstancesHealthStatus</code> request to get the next group of results. Specify the value of <code>NextToken</code> from the previous response in the next request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
}
