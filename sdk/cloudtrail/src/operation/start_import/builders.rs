// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_import::_start_import_output::StartImportOutputBuilder;

pub use crate::operation::start_import::_start_import_input::StartImportInputBuilder;

/// Fluent builder constructing a request to `StartImport`.
///
/// <p> Starts an import of logged trail events from a source S3 bucket to a destination event data store. By default, CloudTrail only imports events contained in the S3 bucket's <code>CloudTrail</code> prefix and the prefixes inside the <code>CloudTrail</code> prefix, and does not check prefixes for other Amazon Web Services services. If you want to import CloudTrail events contained in another prefix, you must include the prefix in the <code>S3LocationUri</code>. For more considerations about importing trail events, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-copy-trail-to-lake.html#cloudtrail-trail-copy-considerations">Considerations</a>. </p>
/// <p> When you start a new import, the <code>Destinations</code> and <code>ImportSource</code> parameters are required. Before starting a new import, disable any access control lists (ACLs) attached to the source S3 bucket. For more information about disabling ACLs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket</a>. </p>
/// <p> When you retry an import, the <code>ImportID</code> parameter is required. </p> <note>
/// <p> If the destination event data store is for an organization, you must use the management account to import trail events. You cannot use the delegated administrator account for the organization. </p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct StartImportFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_import::builders::StartImportInputBuilder,
}
impl StartImportFluentBuilder {
    /// Creates a new `StartImport`.
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
            crate::operation::start_import::StartImport,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
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
        crate::operation::start_import::StartImportOutput,
        aws_smithy_http::result::SdkError<crate::operation::start_import::StartImportError>,
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
    /// Appends an item to `Destinations`.
    ///
    /// To override the contents of this collection use [`set_destinations`](Self::set_destinations).
    ///
    /// <p> The ARN of the destination event data store. Use this parameter for a new import. </p>
    pub fn destinations(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.destinations(input.into());
        self
    }
    /// <p> The ARN of the destination event data store. Use this parameter for a new import. </p>
    pub fn set_destinations(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_destinations(input);
        self
    }
    /// <p> The source S3 bucket for the import. Use this parameter for a new import. </p>
    pub fn import_source(mut self, input: crate::types::ImportSource) -> Self {
        self.inner = self.inner.import_source(input);
        self
    }
    /// <p> The source S3 bucket for the import. Use this parameter for a new import. </p>
    pub fn set_import_source(
        mut self,
        input: std::option::Option<crate::types::ImportSource>,
    ) -> Self {
        self.inner = self.inner.set_import_source(input);
        self
    }
    /// <p> Use with <code>EndEventTime</code> to bound a <code>StartImport</code> request, and limit imported trail events to only those events logged within a specified time period. When you specify a time range, CloudTrail checks the prefix and log file names to verify the names contain a date between the specified <code>StartEventTime</code> and <code>EndEventTime</code> before attempting to import events. </p>
    pub fn start_event_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_event_time(input);
        self
    }
    /// <p> Use with <code>EndEventTime</code> to bound a <code>StartImport</code> request, and limit imported trail events to only those events logged within a specified time period. When you specify a time range, CloudTrail checks the prefix and log file names to verify the names contain a date between the specified <code>StartEventTime</code> and <code>EndEventTime</code> before attempting to import events. </p>
    pub fn set_start_event_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_start_event_time(input);
        self
    }
    /// <p> Use with <code>StartEventTime</code> to bound a <code>StartImport</code> request, and limit imported trail events to only those events logged within a specified time period. When you specify a time range, CloudTrail checks the prefix and log file names to verify the names contain a date between the specified <code>StartEventTime</code> and <code>EndEventTime</code> before attempting to import events. </p>
    pub fn end_event_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_event_time(input);
        self
    }
    /// <p> Use with <code>StartEventTime</code> to bound a <code>StartImport</code> request, and limit imported trail events to only those events logged within a specified time period. When you specify a time range, CloudTrail checks the prefix and log file names to verify the names contain a date between the specified <code>StartEventTime</code> and <code>EndEventTime</code> before attempting to import events. </p>
    pub fn set_end_event_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_end_event_time(input);
        self
    }
    /// <p> The ID of the import. Use this parameter when you are retrying an import. </p>
    pub fn import_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.import_id(input.into());
        self
    }
    /// <p> The ID of the import. Use this parameter when you are retrying an import. </p>
    pub fn set_import_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_import_id(input);
        self
    }
}
