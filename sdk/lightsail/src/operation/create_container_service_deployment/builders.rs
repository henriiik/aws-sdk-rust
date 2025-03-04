// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_container_service_deployment::_create_container_service_deployment_output::CreateContainerServiceDeploymentOutputBuilder;

pub use crate::operation::create_container_service_deployment::_create_container_service_deployment_input::CreateContainerServiceDeploymentInputBuilder;

/// Fluent builder constructing a request to `CreateContainerServiceDeployment`.
///
/// <p>Creates a deployment for your Amazon Lightsail container service.</p>
/// <p>A deployment specifies the containers that will be launched on the container service and their settings, such as the ports to open, the environment variables to apply, and the launch command to run. It also specifies the container that will serve as the public endpoint of the deployment and its settings, such as the HTTP or HTTPS port to use, and the health check configuration.</p>
/// <p>You can deploy containers to your container service using container images from a public registry such as Amazon ECR Public, or from your local machine. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-creating-container-images">Creating container images for your Amazon Lightsail container services</a> in the <i>Amazon Lightsail Developer Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateContainerServiceDeploymentFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_container_service_deployment::builders::CreateContainerServiceDeploymentInputBuilder
            }
impl CreateContainerServiceDeploymentFluentBuilder {
    /// Creates a new `CreateContainerServiceDeployment`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_container_service_deployment::CreateContainerServiceDeployment, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentError>
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentOutput, aws_smithy_http::result::SdkError<crate::operation::create_container_service_deployment::CreateContainerServiceDeploymentError>>
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
    /// <p>The name of the container service for which to create the deployment.</p>
    pub fn service_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.service_name(input.into());
        self
    }
    /// <p>The name of the container service for which to create the deployment.</p>
    pub fn set_service_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_service_name(input);
        self
    }
    /// Adds a key-value pair to `containers`.
    ///
    /// To override the contents of this collection use [`set_containers`](Self::set_containers).
    ///
    /// <p>An object that describes the settings of the containers that will be launched on the container service.</p>
    pub fn containers(
        mut self,
        k: impl Into<std::string::String>,
        v: crate::types::Container,
    ) -> Self {
        self.inner = self.inner.containers(k.into(), v);
        self
    }
    /// <p>An object that describes the settings of the containers that will be launched on the container service.</p>
    pub fn set_containers(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, crate::types::Container>,
        >,
    ) -> Self {
        self.inner = self.inner.set_containers(input);
        self
    }
    /// <p>An object that describes the settings of the public endpoint for the container service.</p>
    pub fn public_endpoint(mut self, input: crate::types::EndpointRequest) -> Self {
        self.inner = self.inner.public_endpoint(input);
        self
    }
    /// <p>An object that describes the settings of the public endpoint for the container service.</p>
    pub fn set_public_endpoint(
        mut self,
        input: std::option::Option<crate::types::EndpointRequest>,
    ) -> Self {
        self.inner = self.inner.set_public_endpoint(input);
        self
    }
}
