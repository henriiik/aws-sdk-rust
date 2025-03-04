// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::archive_findings::_archive_findings_output::ArchiveFindingsOutputBuilder;

pub use crate::operation::archive_findings::_archive_findings_input::ArchiveFindingsInputBuilder;

/// Fluent builder constructing a request to `ArchiveFindings`.
///
/// <p>Archives GuardDuty findings that are specified by the list of finding IDs.</p> <note>
/// <p>Only the administrator account can archive findings. Member accounts don't have permission to archive findings from their accounts.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ArchiveFindingsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::archive_findings::builders::ArchiveFindingsInputBuilder,
}
impl ArchiveFindingsFluentBuilder {
    /// Creates a new `ArchiveFindings`.
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
            crate::operation::archive_findings::ArchiveFindings,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::archive_findings::ArchiveFindingsError>,
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
        crate::operation::archive_findings::ArchiveFindingsOutput,
        aws_smithy_http::result::SdkError<crate::operation::archive_findings::ArchiveFindingsError>,
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
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to archive.</p>
    pub fn detector_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The ID of the detector that specifies the GuardDuty service whose findings you want to archive.</p>
    pub fn set_detector_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// Appends an item to `FindingIds`.
    ///
    /// To override the contents of this collection use [`set_finding_ids`](Self::set_finding_ids).
    ///
    /// <p>The IDs of the findings that you want to archive.</p>
    pub fn finding_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.finding_ids(input.into());
        self
    }
    /// <p>The IDs of the findings that you want to archive.</p>
    pub fn set_finding_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_finding_ids(input);
        self
    }
}
