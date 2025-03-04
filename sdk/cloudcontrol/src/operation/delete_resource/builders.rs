// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_resource::_delete_resource_output::DeleteResourceOutputBuilder;

pub use crate::operation::delete_resource::_delete_resource_input::DeleteResourceInputBuilder;

/// Fluent builder constructing a request to `DeleteResource`.
///
/// <p>Deletes the specified resource. For details, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations-delete.html">Deleting a resource</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
/// <p>After you have initiated a resource deletion request, you can monitor the progress of your request by calling <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/APIReference/API_GetResourceRequestStatus.html">GetResourceRequestStatus</a> using the <code>RequestToken</code> of the <code>ProgressEvent</code> returned by <code>DeleteResource</code>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourceFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_resource::builders::DeleteResourceInputBuilder,
}
impl DeleteResourceFluentBuilder {
    /// Creates a new `DeleteResource`.
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
            crate::operation::delete_resource::DeleteResource,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_resource::DeleteResourceError>,
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
        crate::operation::delete_resource::DeleteResourceOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_resource::DeleteResourceError>,
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
    /// <p>The name of the resource type.</p>
    pub fn type_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.type_name(input.into());
        self
    }
    /// <p>The name of the resource type.</p>
    pub fn set_type_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_type_name(input);
        self
    }
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    pub fn type_version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.type_version_id(input.into());
        self
    }
    /// <p>For private resource types, the type version to use in this resource operation. If you do not specify a resource version, CloudFormation uses the default version.</p>
    pub fn set_type_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_type_version_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role for Cloud Control API to use when performing this resource operation. The role specified must have the permissions required for this operation. The necessary permissions for each event handler are defined in the <code> <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html#schema-properties-handlers">handlers</a> </code> section of the <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-schema.html">resource type definition schema</a>.</p>
    /// <p>If you do not specify a role, Cloud Control API uses a temporary session created using your Amazon Web Services user credentials.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-permissions">Specifying credentials</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique identifier to ensure the idempotency of the resource request. As a best practice, specify this token to ensure idempotency, so that Amazon Web Services Cloud Control API can accurately distinguish between request retries and new resource requests. You might retry a resource request to ensure that it was successfully received.</p>
    /// <p>A client token is valid for 36 hours once used. After that, a resource request with the same client token is treated as a new request.</p>
    /// <p>If you do not specify a client token, one is generated for inclusion in the request.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-operations.html#resource-operations-idempotency">Ensuring resource operation requests are unique</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The identifier for the resource.</p>
    /// <p>You can specify the primary identifier, or any secondary identifier defined for the resource type in its resource schema. You can only specify one identifier. Primary identifiers can be specified as a string or JSON; secondary identifiers must be specified as JSON.</p>
    /// <p>For compound primary identifiers (that is, one that consists of multiple resource properties strung together), to specify the primary identifier as a string, list the property values <i>in the order they are specified</i> in the primary identifier definition, separated by <code>|</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-identifier.html">Identifying resources</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The identifier for the resource.</p>
    /// <p>You can specify the primary identifier, or any secondary identifier defined for the resource type in its resource schema. You can only specify one identifier. Primary identifiers can be specified as a string or JSON; secondary identifiers must be specified as JSON.</p>
    /// <p>For compound primary identifiers (that is, one that consists of multiple resource properties strung together), to specify the primary identifier as a string, list the property values <i>in the order they are specified</i> in the primary identifier definition, separated by <code>|</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cloudcontrolapi/latest/userguide/resource-identifier.html">Identifying resources</a> in the <i>Amazon Web Services Cloud Control API User Guide</i>.</p>
    pub fn set_identifier(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
}
