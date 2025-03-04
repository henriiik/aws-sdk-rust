// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_nfs_file_share::_update_nfs_file_share_output::UpdateNfsFileShareOutputBuilder;

pub use crate::operation::update_nfs_file_share::_update_nfs_file_share_input::UpdateNfsFileShareInputBuilder;

/// Fluent builder constructing a request to `UpdateNFSFileShare`.
///
/// <p>Updates a Network File System (NFS) file share. This operation is only supported in S3 File Gateways.</p> <note>
/// <p>To leave a file share field unchanged, set the corresponding input field to null.</p>
/// </note>
/// <p>Updates the following file share settings:</p>
/// <ul>
/// <li> <p>Default storage class for your S3 bucket</p> </li>
/// <li> <p>Metadata defaults for your S3 bucket</p> </li>
/// <li> <p>Allowed NFS clients for your file share</p> </li>
/// <li> <p>Squash settings</p> </li>
/// <li> <p>Write status of your file share</p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNFSFileShareFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_nfs_file_share::builders::UpdateNfsFileShareInputBuilder,
}
impl UpdateNFSFileShareFluentBuilder {
    /// Creates a new `UpdateNFSFileShare`.
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
            crate::operation::update_nfs_file_share::UpdateNFSFileShare,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::update_nfs_file_share::UpdateNFSFileShareError,
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
        crate::operation::update_nfs_file_share::UpdateNfsFileShareOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::update_nfs_file_share::UpdateNFSFileShareError,
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
    /// <p>The Amazon Resource Name (ARN) of the file share to be updated.</p>
    pub fn file_share_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.file_share_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the file share to be updated.</p>
    pub fn set_file_share_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_file_share_arn(input);
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn kms_encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.kms_encrypted(input);
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_kms_encrypted(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_kms_encrypted(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn kms_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.kms_key(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn set_kms_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key(input);
        self
    }
    /// <p>The default values for the file share. Optional.</p>
    pub fn nfs_file_share_defaults(mut self, input: crate::types::NfsFileShareDefaults) -> Self {
        self.inner = self.inner.nfs_file_share_defaults(input);
        self
    }
    /// <p>The default values for the file share. Optional.</p>
    pub fn set_nfs_file_share_defaults(
        mut self,
        input: std::option::Option<crate::types::NfsFileShareDefaults>,
    ) -> Self {
        self.inner = self.inner.set_nfs_file_share_defaults(input);
        self
    }
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    pub fn default_storage_class(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.default_storage_class(input.into());
        self
    }
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    pub fn set_default_storage_class(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_default_storage_class(input);
        self
    }
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    pub fn object_acl(mut self, input: crate::types::ObjectAcl) -> Self {
        self.inner = self.inner.object_acl(input);
        self
    }
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    pub fn set_object_acl(mut self, input: std::option::Option<crate::types::ObjectAcl>) -> Self {
        self.inner = self.inner.set_object_acl(input);
        self
    }
    /// Appends an item to `ClientList`.
    ///
    /// To override the contents of this collection use [`set_client_list`](Self::set_client_list).
    ///
    /// <p>The list of clients that are allowed to access the S3 File Gateway. The list must contain either valid IP addresses or valid CIDR blocks.</p>
    pub fn client_list(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_list(input.into());
        self
    }
    /// <p>The list of clients that are allowed to access the S3 File Gateway. The list must contain either valid IP addresses or valid CIDR blocks.</p>
    pub fn set_client_list(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_list(input);
        self
    }
    /// <p>The user mapped to anonymous user.</p>
    /// <p>Valid values are the following:</p>
    /// <ul>
    /// <li> <p> <code>RootSquash</code>: Only root is mapped to anonymous user.</p> </li>
    /// <li> <p> <code>NoSquash</code>: No one is mapped to anonymous user.</p> </li>
    /// <li> <p> <code>AllSquash</code>: Everyone is mapped to anonymous user.</p> </li>
    /// </ul>
    pub fn squash(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.squash(input.into());
        self
    }
    /// <p>The user mapped to anonymous user.</p>
    /// <p>Valid values are the following:</p>
    /// <ul>
    /// <li> <p> <code>RootSquash</code>: Only root is mapped to anonymous user.</p> </li>
    /// <li> <p> <code>NoSquash</code>: No one is mapped to anonymous user.</p> </li>
    /// <li> <p> <code>AllSquash</code>: Everyone is mapped to anonymous user.</p> </li>
    /// </ul>
    pub fn set_squash(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_squash(input);
        self
    }
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn read_only(mut self, input: bool) -> Self {
        self.inner = self.inner.read_only(input);
        self
    }
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set the write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_read_only(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_read_only(input);
        self
    }
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn guess_mime_type_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.guess_mime_type_enabled(input);
        self
    }
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_guess_mime_type_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_guess_mime_type_enabled(input);
        self
    }
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn requester_pays(mut self, input: bool) -> Self {
        self.inner = self.inner.requester_pays(input);
        self
    }
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_requester_pays(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_requester_pays(input);
        self
    }
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    pub fn file_share_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.file_share_name(input.into());
        self
    }
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    pub fn set_file_share_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_file_share_name(input);
        self
    }
    /// <p>Specifies refresh cache information for the file share.</p>
    pub fn cache_attributes(mut self, input: crate::types::CacheAttributes) -> Self {
        self.inner = self.inner.cache_attributes(input);
        self
    }
    /// <p>Specifies refresh cache information for the file share.</p>
    pub fn set_cache_attributes(
        mut self,
        input: std::option::Option<crate::types::CacheAttributes>,
    ) -> Self {
        self.inner = self.inner.set_cache_attributes(input);
        self
    }
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    pub fn notification_policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.notification_policy(input.into());
        self
    }
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    pub fn set_notification_policy(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_notification_policy(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    pub fn audit_destination_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.audit_destination_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    pub fn set_audit_destination_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_audit_destination_arn(input);
        self
    }
}
