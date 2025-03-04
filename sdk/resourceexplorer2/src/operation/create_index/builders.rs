// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_index::_create_index_output::CreateIndexOutputBuilder;

pub use crate::operation::create_index::_create_index_input::CreateIndexInputBuilder;

/// Fluent builder constructing a request to `CreateIndex`.
///
/// <p>Turns on Amazon Web Services Resource Explorer in the Amazon Web Services Region in which you called this operation by creating an index. Resource Explorer begins discovering the resources in this Region and stores the details about the resources in the index so that they can be queried by using the <code>Search</code> operation. You can create only one index in a Region.</p> <note>
/// <p>This operation creates only a <i>local</i> index. To promote the local index in one Amazon Web Services Region into the aggregator index for the Amazon Web Services account, use the <code>UpdateIndexType</code> operation. For more information, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html">Turning on cross-Region search by creating an aggregator index</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p>
/// </note>
/// <p>For more details about what happens when you turn on Resource Explorer in an Amazon Web Services Region, see <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-service-activate.html">Turn on Resource Explorer to index your resources in an Amazon Web Services Region</a> in the <i>Amazon Web Services Resource Explorer User Guide</i>.</p>
/// <p>If this is the first Amazon Web Services Region in which you've created an index for Resource Explorer, then this operation also <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/security_iam_service-linked-roles.html">creates a service-linked role</a> in your Amazon Web Services account that allows Resource Explorer to enumerate your resources to populate the index.</p>
/// <ul>
/// <li> <p> <b>Action</b>: <code>resource-explorer-2:CreateIndex</code> </p> <p> <b>Resource</b>: The ARN of the index (as it will exist after the operation completes) in the Amazon Web Services Region and account in which you're trying to create the index. Use the wildcard character (<code>*</code>) at the end of the string to match the eventual UUID. For example, the following <code>Resource</code> element restricts the role or user to creating an index in only the <code>us-east-2</code> Region of the specified account.</p> <p> <code>"Resource": "arn:aws:resource-explorer-2:us-west-2:<i>&lt;account-id&gt;</i>:index/*"</code> </p> <p>Alternatively, you can use <code>"Resource": "*"</code> to allow the role or user to create an index in any Region.</p> </li>
/// <li> <p> <b>Action</b>: <code>iam:CreateServiceLinkedRole</code> </p> <p> <b>Resource</b>: No specific resource (*). </p> <p>This permission is required only the first time you create an index to turn on Resource Explorer in the account. Resource Explorer uses this to create the <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/security_iam_service-linked-roles.html">service-linked role needed to index the resources in your account</a>. Resource Explorer uses the same service-linked role for all additional indexes you create afterwards.</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateIndexFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_index::builders::CreateIndexInputBuilder,
}
impl CreateIndexFluentBuilder {
    /// Creates a new `CreateIndex`.
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
            crate::operation::create_index::CreateIndex,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::create_index::CreateIndexError>,
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
        crate::operation::create_index::CreateIndexOutput,
        aws_smithy_http::result::SdkError<crate::operation::create_index::CreateIndexError>,
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
    /// <p>This value helps ensure idempotency. Resource Explorer uses this value to prevent the accidental creation of duplicate versions. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type value</a> to ensure the uniqueness of your views.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>This value helps ensure idempotency. Resource Explorer uses this value to prevent the accidental creation of duplicate versions. We recommend that you generate a <a href="https://wikipedia.org/wiki/Universally_unique_identifier">UUID-type value</a> to ensure the uniqueness of your views.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The specified tags are attached only to the index created in this Amazon Web Services Region. The tags aren't attached to any of the resources listed in the index.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The specified tags are attached only to the index created in this Amazon Web Services Region. The tags aren't attached to any of the resources listed in the index.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
