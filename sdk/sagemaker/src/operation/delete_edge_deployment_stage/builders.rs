// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_edge_deployment_stage::_delete_edge_deployment_stage_output::DeleteEdgeDeploymentStageOutputBuilder;

pub use crate::operation::delete_edge_deployment_stage::_delete_edge_deployment_stage_input::DeleteEdgeDeploymentStageInputBuilder;

/// Fluent builder constructing a request to `DeleteEdgeDeploymentStage`.
///
/// <p>Delete a stage in an edge deployment plan if (and only if) the stage is inactive.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEdgeDeploymentStageFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::delete_edge_deployment_stage::builders::DeleteEdgeDeploymentStageInputBuilder
            }
impl DeleteEdgeDeploymentStageFluentBuilder {
    /// Creates a new `DeleteEdgeDeploymentStage`.
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
            crate::operation::delete_edge_deployment_stage::DeleteEdgeDeploymentStage,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_edge_deployment_stage::DeleteEdgeDeploymentStageError,
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
        crate::operation::delete_edge_deployment_stage::DeleteEdgeDeploymentStageOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_edge_deployment_stage::DeleteEdgeDeploymentStageError,
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
    /// <p>The name of the edge deployment plan from which the stage will be deleted.</p>
    pub fn edge_deployment_plan_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.edge_deployment_plan_name(input.into());
        self
    }
    /// <p>The name of the edge deployment plan from which the stage will be deleted.</p>
    pub fn set_edge_deployment_plan_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_edge_deployment_plan_name(input);
        self
    }
    /// <p>The name of the stage.</p>
    pub fn stage_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.stage_name(input.into());
        self
    }
    /// <p>The name of the stage.</p>
    pub fn set_stage_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_stage_name(input);
        self
    }
}
