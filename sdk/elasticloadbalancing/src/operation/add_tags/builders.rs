// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_tags::_add_tags_output::AddTagsOutputBuilder;

pub use crate::operation::add_tags::_add_tags_input::AddTagsInputBuilder;

/// Fluent builder constructing a request to `AddTags`.
///
/// <p>Adds the specified tags to the specified load balancer. Each load balancer can have a maximum of 10 tags.</p>
/// <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the load balancer, <code>AddTags</code> updates its value.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/elasticloadbalancing/latest/classic/add-remove-tags.html">Tag Your Classic Load Balancer</a> in the <i>Classic Load Balancers Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AddTagsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_tags::builders::AddTagsInputBuilder,
}
impl AddTagsFluentBuilder {
    /// Creates a new `AddTags`.
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
            crate::operation::add_tags::AddTags,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::add_tags::AddTagsError>,
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
        crate::operation::add_tags::AddTagsOutput,
        aws_smithy_http::result::SdkError<crate::operation::add_tags::AddTagsError>,
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
    /// Appends an item to `LoadBalancerNames`.
    ///
    /// To override the contents of this collection use [`set_load_balancer_names`](Self::set_load_balancer_names).
    ///
    /// <p>The name of the load balancer. You can specify one load balancer only.</p>
    pub fn load_balancer_names(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.load_balancer_names(input.into());
        self
    }
    /// <p>The name of the load balancer. You can specify one load balancer only.</p>
    pub fn set_load_balancer_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_load_balancer_names(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
