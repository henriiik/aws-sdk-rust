// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_data_cells_filter::_delete_data_cells_filter_output::DeleteDataCellsFilterOutputBuilder;

pub use crate::operation::delete_data_cells_filter::_delete_data_cells_filter_input::DeleteDataCellsFilterInputBuilder;

/// Fluent builder constructing a request to `DeleteDataCellsFilter`.
///
/// <p>Deletes a data cell filter.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDataCellsFilterFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_data_cells_filter::builders::DeleteDataCellsFilterInputBuilder,
}
impl DeleteDataCellsFilterFluentBuilder {
    /// Creates a new `DeleteDataCellsFilter`.
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
            crate::operation::delete_data_cells_filter::DeleteDataCellsFilter,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_data_cells_filter::DeleteDataCellsFilterError,
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
        crate::operation::delete_data_cells_filter::DeleteDataCellsFilterOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_data_cells_filter::DeleteDataCellsFilterError,
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
    /// <p>The ID of the catalog to which the table belongs.</p>
    pub fn table_catalog_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_catalog_id(input.into());
        self
    }
    /// <p>The ID of the catalog to which the table belongs.</p>
    pub fn set_table_catalog_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_catalog_id(input);
        self
    }
    /// <p>A database in the Glue Data Catalog.</p>
    pub fn database_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>A database in the Glue Data Catalog.</p>
    pub fn set_database_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>A table in the database.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>A table in the database.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name given by the user to the data filter cell.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name given by the user to the data filter cell.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
}
