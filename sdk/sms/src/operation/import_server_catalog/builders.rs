// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_server_catalog::_import_server_catalog_output::ImportServerCatalogOutputBuilder;

pub use crate::operation::import_server_catalog::_import_server_catalog_input::ImportServerCatalogInputBuilder;

/// Fluent builder constructing a request to `ImportServerCatalog`.
///
/// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers to import.</p>
/// <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ImportServerCatalogFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_server_catalog::builders::ImportServerCatalogInputBuilder,
}
impl ImportServerCatalogFluentBuilder {
    /// Creates a new `ImportServerCatalog`.
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
            crate::operation::import_server_catalog::ImportServerCatalog,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::import_server_catalog::ImportServerCatalogError,
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
        crate::operation::import_server_catalog::ImportServerCatalogOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::import_server_catalog::ImportServerCatalogError,
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
}
