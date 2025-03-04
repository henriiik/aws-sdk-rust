// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_db_snapshot::_modify_db_snapshot_output::ModifyDbSnapshotOutputBuilder;

pub use crate::operation::modify_db_snapshot::_modify_db_snapshot_input::ModifyDbSnapshotInputBuilder;

/// Fluent builder constructing a request to `ModifyDBSnapshot`.
///
/// <p>Updates a manual DB snapshot with a new engine version. The snapshot can be encrypted or unencrypted, but not shared or public. </p>
/// <p>Amazon RDS supports upgrading DB snapshots for MySQL, PostgreSQL, and Oracle. This command doesn't apply to RDS Custom.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyDBSnapshotFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_db_snapshot::builders::ModifyDbSnapshotInputBuilder,
}
impl ModifyDBSnapshotFluentBuilder {
    /// Creates a new `ModifyDBSnapshot`.
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
            crate::operation::modify_db_snapshot::ModifyDBSnapshot,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_db_snapshot::ModifyDBSnapshotError,
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
        crate::operation::modify_db_snapshot::ModifyDbSnapshotOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_db_snapshot::ModifyDBSnapshotError,
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
    /// <p>The identifier of the DB snapshot to modify.</p>
    pub fn db_snapshot_identifier(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.db_snapshot_identifier(input.into());
        self
    }
    /// <p>The identifier of the DB snapshot to modify.</p>
    pub fn set_db_snapshot_identifier(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_snapshot_identifier(input);
        self
    }
    /// <p>The engine version to upgrade the DB snapshot to.</p>
    /// <p>The following are the database engines and engine versions that are available when you upgrade a DB snapshot.</p>
    /// <p> <b>MySQL</b> </p>
    /// <ul>
    /// <li> <p> <code>5.5.46</code> (supported for 5.1 DB snapshots)</p> </li>
    /// </ul>
    /// <p> <b>Oracle</b> </p>
    /// <ul>
    /// <li> <p> <code>12.1.0.2.v8</code> (supported for 12.1.0.1 DB snapshots)</p> </li>
    /// <li> <p> <code>11.2.0.4.v12</code> (supported for 11.2.0.2 DB snapshots)</p> </li>
    /// <li> <p> <code>11.2.0.4.v11</code> (supported for 11.2.0.3 DB snapshots)</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>For the list of engine versions that are available for upgrading a DB snapshot, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.PostgreSQL.html#USER_UpgradeDBInstance.PostgreSQL.MajorVersion"> Upgrading the PostgreSQL DB Engine for Amazon RDS</a>.</p>
    pub fn engine_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The engine version to upgrade the DB snapshot to.</p>
    /// <p>The following are the database engines and engine versions that are available when you upgrade a DB snapshot.</p>
    /// <p> <b>MySQL</b> </p>
    /// <ul>
    /// <li> <p> <code>5.5.46</code> (supported for 5.1 DB snapshots)</p> </li>
    /// </ul>
    /// <p> <b>Oracle</b> </p>
    /// <ul>
    /// <li> <p> <code>12.1.0.2.v8</code> (supported for 12.1.0.1 DB snapshots)</p> </li>
    /// <li> <p> <code>11.2.0.4.v12</code> (supported for 11.2.0.2 DB snapshots)</p> </li>
    /// <li> <p> <code>11.2.0.4.v11</code> (supported for 11.2.0.3 DB snapshots)</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>For the list of engine versions that are available for upgrading a DB snapshot, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.PostgreSQL.html#USER_UpgradeDBInstance.PostgreSQL.MajorVersion"> Upgrading the PostgreSQL DB Engine for Amazon RDS</a>.</p>
    pub fn set_engine_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The option group to identify with the upgraded DB snapshot.</p>
    /// <p>You can specify this parameter when you upgrade an Oracle DB snapshot. The same option group considerations apply when upgrading a DB snapshot as when upgrading a DB instance. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Oracle.html#USER_UpgradeDBInstance.Oracle.OGPG.OG">Option group considerations</a> in the <i>Amazon RDS User Guide.</i> </p>
    pub fn option_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.option_group_name(input.into());
        self
    }
    /// <p>The option group to identify with the upgraded DB snapshot.</p>
    /// <p>You can specify this parameter when you upgrade an Oracle DB snapshot. The same option group considerations apply when upgrading a DB snapshot as when upgrading a DB instance. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Oracle.html#USER_UpgradeDBInstance.Oracle.OGPG.OG">Option group considerations</a> in the <i>Amazon RDS User Guide.</i> </p>
    pub fn set_option_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_option_group_name(input);
        self
    }
}
