// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_rotation::_update_rotation_output::UpdateRotationOutputBuilder;

pub use crate::operation::update_rotation::_update_rotation_input::UpdateRotationInputBuilder;

/// Fluent builder constructing a request to `UpdateRotation`.
///
/// <p>Updates the information specified for an on-call rotation.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRotationFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_rotation::builders::UpdateRotationInputBuilder,
}
impl UpdateRotationFluentBuilder {
    /// Creates a new `UpdateRotation`.
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
            crate::operation::update_rotation::UpdateRotation,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::update_rotation::UpdateRotationError>,
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
        crate::operation::update_rotation::UpdateRotationOutput,
        aws_smithy_http::result::SdkError<crate::operation::update_rotation::UpdateRotationError>,
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
    /// <p>The Amazon Resource Name (ARN) of the rotation to update.</p>
    pub fn rotation_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.rotation_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rotation to update.</p>
    pub fn set_rotation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_rotation_id(input);
        self
    }
    /// Appends an item to `ContactIds`.
    ///
    /// To override the contents of this collection use [`set_contact_ids`](Self::set_contact_ids).
    ///
    /// <p>The Amazon Resource Names (ARNs) of the contacts to include in the updated rotation. </p>
    /// <p>The order in which you list the contacts is their shift order in the rotation schedule.</p>
    pub fn contact_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.contact_ids(input.into());
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the contacts to include in the updated rotation. </p>
    /// <p>The order in which you list the contacts is their shift order in the rotation schedule.</p>
    pub fn set_contact_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_contact_ids(input);
        self
    }
    /// <p>The date and time the rotation goes into effect.</p>
    pub fn start_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The date and time the rotation goes into effect.</p>
    pub fn set_start_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The time zone to base the updated rotation’s activity on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p> <note>
    /// <p>Designators for time zones that don’t support Daylight Savings Time Rules, such as Pacific Standard Time (PST) and Pacific Daylight Time (PDT), aren't supported.</p>
    /// </note>
    pub fn time_zone_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.time_zone_id(input.into());
        self
    }
    /// <p>The time zone to base the updated rotation’s activity on, in Internet Assigned Numbers Authority (IANA) format. For example: "America/Los_Angeles", "UTC", or "Asia/Seoul". For more information, see the <a href="https://www.iana.org/time-zones">Time Zone Database</a> on the IANA website.</p> <note>
    /// <p>Designators for time zones that don’t support Daylight Savings Time Rules, such as Pacific Standard Time (PST) and Pacific Daylight Time (PDT), aren't supported.</p>
    /// </note>
    pub fn set_time_zone_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_time_zone_id(input);
        self
    }
    /// <p>Information about how long the updated rotation lasts before restarting at the beginning of the shift order.</p>
    pub fn recurrence(mut self, input: crate::types::RecurrenceSettings) -> Self {
        self.inner = self.inner.recurrence(input);
        self
    }
    /// <p>Information about how long the updated rotation lasts before restarting at the beginning of the shift order.</p>
    pub fn set_recurrence(
        mut self,
        input: std::option::Option<crate::types::RecurrenceSettings>,
    ) -> Self {
        self.inner = self.inner.set_recurrence(input);
        self
    }
}
