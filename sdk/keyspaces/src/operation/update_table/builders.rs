// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_table::_update_table_output::UpdateTableOutputBuilder;

pub use crate::operation::update_table::_update_table_input::UpdateTableInputBuilder;

/// Fluent builder constructing a request to `UpdateTable`.
///
/// <p>Adds new columns to the table or updates one of the table's settings, for example capacity mode, encryption, point-in-time recovery, or ttl settings. Note that you can only update one specific table setting per update operation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTableFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_table::builders::UpdateTableInputBuilder,
}
impl UpdateTableFluentBuilder {
    /// Creates a new `UpdateTable`.
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
            crate::operation::update_table::UpdateTable,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
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
        crate::operation::update_table::UpdateTableOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_table::UpdateTableError>,
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
    /// <p>The name of the keyspace the specified table is stored in.</p>
    pub fn keyspace_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.keyspace_name(input.into());
        self
    }
    /// <p>The name of the keyspace the specified table is stored in.</p>
    pub fn set_keyspace_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_keyspace_name(input);
        self
    }
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// Appends an item to `addColumns`.
    ///
    /// To override the contents of this collection use [`set_add_columns`](Self::set_add_columns).
    ///
    /// <p>For each column to be added to the specified table:</p>
    /// <ul>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn add_columns(mut self, input: crate::types::ColumnDefinition) -> Self {
        self.inner = self.inner.add_columns(input);
        self
    }
    /// <p>For each column to be added to the specified table:</p>
    /// <ul>
    /// <li> <p> <code>name</code> - The name of the column.</p> </li>
    /// <li> <p> <code>type</code> - An Amazon Keyspaces data type. For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/cql.elements.html#cql.data-types">Data types</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p> </li>
    /// </ul>
    pub fn set_add_columns(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ColumnDefinition>>,
    ) -> Self {
        self.inner = self.inner.set_add_columns(input);
        self
    }
    /// <p>Modifies the read/write throughput capacity mode for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> and </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn capacity_specification(mut self, input: crate::types::CapacitySpecification) -> Self {
        self.inner = self.inner.capacity_specification(input);
        self
    }
    /// <p>Modifies the read/write throughput capacity mode for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>throughputMode:PAY_PER_REQUEST</code> and </p> </li>
    /// <li> <p> <code>throughputMode:PROVISIONED</code> - Provisioned capacity mode requires <code>readCapacityUnits</code> and <code>writeCapacityUnits</code> as input.</p> </li>
    /// </ul>
    /// <p>The default is <code>throughput_mode:PAY_PER_REQUEST</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/ReadWriteCapacityMode.html">Read/write capacity modes</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_capacity_specification(
        mut self,
        input: std::option::Option<crate::types::CapacitySpecification>,
    ) -> Self {
        self.inner = self.inner.set_capacity_specification(input);
        self
    }
    /// <p>Modifies the encryption settings of the table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input. </p> </li>
    /// </ul>
    /// <p>The default is <code>AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn encryption_specification(
        mut self,
        input: crate::types::EncryptionSpecification,
    ) -> Self {
        self.inner = self.inner.encryption_specification(input);
        self
    }
    /// <p>Modifies the encryption settings of the table. You can choose one of the following KMS key (KMS key):</p>
    /// <ul>
    /// <li> <p> <code>type:AWS_OWNED_KMS_KEY</code> - This key is owned by Amazon Keyspaces. </p> </li>
    /// <li> <p> <code>type:CUSTOMER_MANAGED_KMS_KEY</code> - This key is stored in your account and is created, owned, and managed by you. This option requires the <code>kms_key_identifier</code> of the KMS key in Amazon Resource Name (ARN) format as input. </p> </li>
    /// </ul>
    /// <p>The default is <code>AWS_OWNED_KMS_KEY</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/EncryptionAtRest.html">Encryption at rest</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_encryption_specification(
        mut self,
        input: std::option::Option<crate::types::EncryptionSpecification>,
    ) -> Self {
        self.inner = self.inner.set_encryption_specification(input);
        self
    }
    /// <p>Modifies the <code>pointInTimeRecovery</code> settings of the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn point_in_time_recovery(mut self, input: crate::types::PointInTimeRecovery) -> Self {
        self.inner = self.inner.point_in_time_recovery(input);
        self
    }
    /// <p>Modifies the <code>pointInTimeRecovery</code> settings of the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status=ENABLED</code> </p> </li>
    /// <li> <p> <code>status=DISABLED</code> </p> </li>
    /// </ul>
    /// <p>If it's not specified, the default is <code>status=DISABLED</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/PointInTimeRecovery.html">Point-in-time recovery</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_point_in_time_recovery(
        mut self,
        input: std::option::Option<crate::types::PointInTimeRecovery>,
    ) -> Self {
        self.inner = self.inner.set_point_in_time_recovery(input);
        self
    }
    /// <p>Modifies Time to Live custom settings for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status:enabled</code> </p> </li>
    /// <li> <p> <code>status:disabled</code> </p> </li>
    /// </ul>
    /// <p>The default is <code>status:disabled</code>. After <code>ttl</code> is enabled, you can't disable it for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html">Expiring data by using Amazon Keyspaces Time to Live (TTL)</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn ttl(mut self, input: crate::types::TimeToLive) -> Self {
        self.inner = self.inner.ttl(input);
        self
    }
    /// <p>Modifies Time to Live custom settings for the table. The options are:</p>
    /// <ul>
    /// <li> <p> <code>status:enabled</code> </p> </li>
    /// <li> <p> <code>status:disabled</code> </p> </li>
    /// </ul>
    /// <p>The default is <code>status:disabled</code>. After <code>ttl</code> is enabled, you can't disable it for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL.html">Expiring data by using Amazon Keyspaces Time to Live (TTL)</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_ttl(mut self, input: std::option::Option<crate::types::TimeToLive>) -> Self {
        self.inner = self.inner.set_ttl(input);
        self
    }
    /// <p>The default Time to Live setting in seconds for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl">Setting the default TTL value for a table</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn default_time_to_live(mut self, input: i32) -> Self {
        self.inner = self.inner.default_time_to_live(input);
        self
    }
    /// <p>The default Time to Live setting in seconds for the table.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/keyspaces/latest/devguide/TTL-how-it-works.html#ttl-howitworks_default_ttl">Setting the default TTL value for a table</a> in the <i>Amazon Keyspaces Developer Guide</i>.</p>
    pub fn set_default_time_to_live(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_default_time_to_live(input);
        self
    }
    /// <p>Enables client-side timestamps for the table. By default, the setting is disabled. You can enable client-side timestamps with the following option:</p>
    /// <ul>
    /// <li> <p> <code>status: "enabled"</code> </p> </li>
    /// </ul>
    /// <p>Once client-side timestamps are enabled for a table, this setting cannot be disabled.</p>
    pub fn client_side_timestamps(mut self, input: crate::types::ClientSideTimestamps) -> Self {
        self.inner = self.inner.client_side_timestamps(input);
        self
    }
    /// <p>Enables client-side timestamps for the table. By default, the setting is disabled. You can enable client-side timestamps with the following option:</p>
    /// <ul>
    /// <li> <p> <code>status: "enabled"</code> </p> </li>
    /// </ul>
    /// <p>Once client-side timestamps are enabled for a table, this setting cannot be disabled.</p>
    pub fn set_client_side_timestamps(
        mut self,
        input: std::option::Option<crate::types::ClientSideTimestamps>,
    ) -> Self {
        self.inner = self.inner.set_client_side_timestamps(input);
        self
    }
}
