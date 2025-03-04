// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_dataset_group::_describe_dataset_group_output::DescribeDatasetGroupOutputBuilder;

pub use crate::operation::describe_dataset_group::_describe_dataset_group_input::DescribeDatasetGroupInputBuilder;

/// Fluent builder constructing a request to `DescribeDatasetGroup`.
///
/// <p>Describes a dataset group created using the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDatasetGroup.html">CreateDatasetGroup</a> operation.</p>
/// <p>In addition to listing the parameters provided in the <code>CreateDatasetGroup</code> request, this operation includes the following properties:</p>
/// <ul>
/// <li> <p> <code>DatasetArns</code> - The datasets belonging to the group.</p> </li>
/// <li> <p> <code>CreationTime</code> </p> </li>
/// <li> <p> <code>LastModificationTime</code> </p> </li>
/// <li> <p> <code>Status</code> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeDatasetGroupFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_dataset_group::builders::DescribeDatasetGroupInputBuilder,
}
impl DescribeDatasetGroupFluentBuilder {
    /// Creates a new `DescribeDatasetGroup`.
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
            crate::operation::describe_dataset_group::DescribeDatasetGroup,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_dataset_group::DescribeDatasetGroupError,
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
        crate::operation::describe_dataset_group::DescribeDatasetGroupOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_dataset_group::DescribeDatasetGroupError,
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
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    pub fn dataset_group_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.dataset_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the dataset group.</p>
    pub fn set_dataset_group_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_dataset_group_arn(input);
        self
    }
}
