// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_code_repository::_update_code_repository_output::UpdateCodeRepositoryOutputBuilder;

pub use crate::operation::update_code_repository::_update_code_repository_input::UpdateCodeRepositoryInputBuilder;

/// Fluent builder constructing a request to `UpdateCodeRepository`.
///
/// <p>Updates the specified Git repository with the specified values.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateCodeRepositoryFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_code_repository::builders::UpdateCodeRepositoryInputBuilder,
}
impl UpdateCodeRepositoryFluentBuilder {
    /// Creates a new `UpdateCodeRepository`.
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
            crate::operation::update_code_repository::UpdateCodeRepository,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_code_repository::UpdateCodeRepositoryError,
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
        crate::operation::update_code_repository::UpdateCodeRepositoryOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_code_repository::UpdateCodeRepositoryError,
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
    /// <p>The name of the Git repository to update.</p>
    pub fn code_repository_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.code_repository_name(input.into());
        self
    }
    /// <p>The name of the Git repository to update.</p>
    pub fn set_code_repository_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_code_repository_name(input);
        self
    }
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p>
    /// <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    pub fn git_config(mut self, input: crate::types::GitConfigForUpdate) -> Self {
        self.inner = self.inner.git_config(input);
        self
    }
    /// <p>The configuration of the git repository, including the URL and the Amazon Resource Name (ARN) of the Amazon Web Services Secrets Manager secret that contains the credentials used to access the repository. The secret must have a staging label of <code>AWSCURRENT</code> and must be in the following format:</p>
    /// <p> <code>{"username": <i>UserName</i>, "password": <i>Password</i>}</code> </p>
    pub fn set_git_config(
        mut self,
        input: std::option::Option<crate::types::GitConfigForUpdate>,
    ) -> Self {
        self.inner = self.inner.set_git_config(input);
        self
    }
}
