// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl RestoreDbClusterFromS3Input {
    /// Consumes the builder and constructs an Operation<[`RestoreDBClusterFromS3`](crate::operation::restore_db_cluster_from_s3::RestoreDBClusterFromS3)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::restore_db_cluster_from_s3::RestoreDBClusterFromS3,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(_config.use_dual_stack)
            .set_use_fips(_config.use_fips)
            .set_endpoint(_config.endpoint_url.clone())
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::operation::restore_db_cluster_from_s3::RestoreDbClusterFromS3Input,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::restore_db_cluster_from_s3::RestoreDbClusterFromS3Input,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_restore_db_cluster_from_s3_input::ser_restore_db_cluster_from_s3_input_input(&self)?
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::meta::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_credentials_cache(
            &mut request.properties_mut(),
            _config.credentials_cache.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::restore_db_cluster_from_s3::RestoreDBClusterFromS3::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "RestoreDBClusterFromS3",
            "rds",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `RestoreDBClusterFromS3`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct RestoreDBClusterFromS3;
impl RestoreDBClusterFromS3 {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreDBClusterFromS3 {
    type Output = std::result::Result<
        crate::operation::restore_db_cluster_from_s3::RestoreDbClusterFromS3Output,
        crate::operation::restore_db_cluster_from_s3::RestoreDBClusterFromS3Error,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_restore_db_cluster_from_s3::de_restore_db_cluster_from_s3_http_error(response)
        } else {
            crate::protocol_serde::shape_restore_db_cluster_from_s3::de_restore_db_cluster_from_s3_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type RestoreDBClusterFromS3ErrorKind = RestoreDBClusterFromS3Error;
/// Error type for the `RestoreDBClusterFromS3Error` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum RestoreDBClusterFromS3Error {
    /// <p>The user already has a DB cluster with the given identifier.</p>
    DbClusterAlreadyExistsFault(crate::types::error::DbClusterAlreadyExistsFault),
    /// <p> <code>DBClusterIdentifier</code> doesn't refer to an existing DB cluster.</p>
    DbClusterNotFoundFault(crate::types::error::DbClusterNotFoundFault),
    /// <p> <code>DBClusterParameterGroupName</code> doesn't refer to an existing DB cluster parameter group.</p>
    DbClusterParameterGroupNotFoundFault(crate::types::error::DbClusterParameterGroupNotFoundFault),
    /// <p>The user attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota.</p>
    DbClusterQuotaExceededFault(crate::types::error::DbClusterQuotaExceededFault),
    /// <p> <code>DBSubnetGroupName</code> doesn't refer to an existing DB subnet group.</p>
    DbSubnetGroupNotFoundFault(crate::types::error::DbSubnetGroupNotFoundFault),
    /// <p> <code>Domain</code> doesn't refer to an existing Active Directory domain.</p>
    DomainNotFoundFault(crate::types::error::DomainNotFoundFault),
    /// <p>There is insufficient storage available for the current action. You might be able to resolve this error by updating your subnet group to use different Availability Zones that have more storage available.</p>
    InsufficientStorageClusterCapacityFault(
        crate::types::error::InsufficientStorageClusterCapacityFault,
    ),
    /// <p>The requested operation can't be performed while the cluster is in this state.</p>
    InvalidDbClusterStateFault(crate::types::error::InvalidDbClusterStateFault),
    /// <p>The DB subnet group cannot be deleted because it's in use.</p>
    InvalidDbSubnetGroupStateFault(crate::types::error::InvalidDbSubnetGroupStateFault),
    /// <p>The specified Amazon S3 bucket name can't be found or Amazon RDS isn't authorized to access the specified Amazon S3 bucket. Verify the <b>SourceS3BucketName</b> and <b>S3IngestionRoleArn</b> values and try again.</p>
    InvalidS3BucketFault(crate::types::error::InvalidS3BucketFault),
    /// <p>The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC.</p>
    InvalidSubnet(crate::types::error::InvalidSubnet),
    /// <p>The DB subnet group doesn't cover all Availability Zones after it's created because of users' change.</p>
    InvalidVpcNetworkStateFault(crate::types::error::InvalidVpcNetworkStateFault),
    /// <p>An error occurred accessing an Amazon Web Services KMS key.</p>
    KmsKeyNotAccessibleFault(crate::types::error::KmsKeyNotAccessibleFault),
    /// <p>The request would result in the user exceeding the allowed amount of storage available across all DB instances.</p>
    StorageQuotaExceededFault(crate::types::error::StorageQuotaExceededFault),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for RestoreDBClusterFromS3Error {
    fn create_unhandled_error(
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl std::fmt::Display for RestoreDBClusterFromS3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DbClusterAlreadyExistsFault(_inner) => _inner.fmt(f),
            Self::DbClusterNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbClusterParameterGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DbClusterQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::DbSubnetGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::DomainNotFoundFault(_inner) => _inner.fmt(f),
            Self::InsufficientStorageClusterCapacityFault(_inner) => _inner.fmt(f),
            Self::InvalidDbClusterStateFault(_inner) => _inner.fmt(f),
            Self::InvalidDbSubnetGroupStateFault(_inner) => _inner.fmt(f),
            Self::InvalidS3BucketFault(_inner) => _inner.fmt(f),
            Self::InvalidSubnet(_inner) => _inner.fmt(f),
            Self::InvalidVpcNetworkStateFault(_inner) => _inner.fmt(f),
            Self::KmsKeyNotAccessibleFault(_inner) => _inner.fmt(f),
            Self::StorageQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for RestoreDBClusterFromS3Error {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::DbClusterAlreadyExistsFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbClusterNotFoundFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbClusterParameterGroupNotFoundFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbClusterQuotaExceededFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DbSubnetGroupNotFoundFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DomainNotFoundFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InsufficientStorageClusterCapacityFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidDbClusterStateFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidDbSubnetGroupStateFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidS3BucketFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSubnet(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidVpcNetworkStateFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::KmsKeyNotAccessibleFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::StorageQuotaExceededFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::restore_db_cluster_from_s3::RestoreDBClusterFromS3Error
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for RestoreDBClusterFromS3Error {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl RestoreDBClusterFromS3Error {
    /// Creates the `RestoreDBClusterFromS3Error::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `RestoreDBClusterFromS3Error::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err.clone())
                .meta(err)
                .build(),
        )
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::DbClusterAlreadyExistsFault(e) => e.meta(),
            Self::DbClusterNotFoundFault(e) => e.meta(),
            Self::DbClusterParameterGroupNotFoundFault(e) => e.meta(),
            Self::DbClusterQuotaExceededFault(e) => e.meta(),
            Self::DbSubnetGroupNotFoundFault(e) => e.meta(),
            Self::DomainNotFoundFault(e) => e.meta(),
            Self::InsufficientStorageClusterCapacityFault(e) => e.meta(),
            Self::InvalidDbClusterStateFault(e) => e.meta(),
            Self::InvalidDbSubnetGroupStateFault(e) => e.meta(),
            Self::InvalidS3BucketFault(e) => e.meta(),
            Self::InvalidSubnet(e) => e.meta(),
            Self::InvalidVpcNetworkStateFault(e) => e.meta(),
            Self::KmsKeyNotAccessibleFault(e) => e.meta(),
            Self::StorageQuotaExceededFault(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DbClusterAlreadyExistsFault`.
    pub fn is_db_cluster_already_exists_fault(&self) -> bool {
        matches!(self, Self::DbClusterAlreadyExistsFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DbClusterNotFoundFault`.
    pub fn is_db_cluster_not_found_fault(&self) -> bool {
        matches!(self, Self::DbClusterNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DbClusterParameterGroupNotFoundFault`.
    pub fn is_db_cluster_parameter_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbClusterParameterGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DbClusterQuotaExceededFault`.
    pub fn is_db_cluster_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::DbClusterQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DbSubnetGroupNotFoundFault`.
    pub fn is_db_subnet_group_not_found_fault(&self) -> bool {
        matches!(self, Self::DbSubnetGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::DomainNotFoundFault`.
    pub fn is_domain_not_found_fault(&self) -> bool {
        matches!(self, Self::DomainNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InsufficientStorageClusterCapacityFault`.
    pub fn is_insufficient_storage_cluster_capacity_fault(&self) -> bool {
        matches!(self, Self::InsufficientStorageClusterCapacityFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InvalidDbClusterStateFault`.
    pub fn is_invalid_db_cluster_state_fault(&self) -> bool {
        matches!(self, Self::InvalidDbClusterStateFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InvalidDbSubnetGroupStateFault`.
    pub fn is_invalid_db_subnet_group_state_fault(&self) -> bool {
        matches!(self, Self::InvalidDbSubnetGroupStateFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InvalidS3BucketFault`.
    pub fn is_invalid_s3_bucket_fault(&self) -> bool {
        matches!(self, Self::InvalidS3BucketFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InvalidSubnet`.
    pub fn is_invalid_subnet(&self) -> bool {
        matches!(self, Self::InvalidSubnet(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::InvalidVpcNetworkStateFault`.
    pub fn is_invalid_vpc_network_state_fault(&self) -> bool {
        matches!(self, Self::InvalidVpcNetworkStateFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::KmsKeyNotAccessibleFault`.
    pub fn is_kms_key_not_accessible_fault(&self) -> bool {
        matches!(self, Self::KmsKeyNotAccessibleFault(_))
    }
    /// Returns `true` if the error kind is `RestoreDBClusterFromS3Error::StorageQuotaExceededFault`.
    pub fn is_storage_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::StorageQuotaExceededFault(_))
    }
}
impl std::error::Error for RestoreDBClusterFromS3Error {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DbClusterAlreadyExistsFault(_inner) => Some(_inner),
            Self::DbClusterNotFoundFault(_inner) => Some(_inner),
            Self::DbClusterParameterGroupNotFoundFault(_inner) => Some(_inner),
            Self::DbClusterQuotaExceededFault(_inner) => Some(_inner),
            Self::DbSubnetGroupNotFoundFault(_inner) => Some(_inner),
            Self::DomainNotFoundFault(_inner) => Some(_inner),
            Self::InsufficientStorageClusterCapacityFault(_inner) => Some(_inner),
            Self::InvalidDbClusterStateFault(_inner) => Some(_inner),
            Self::InvalidDbSubnetGroupStateFault(_inner) => Some(_inner),
            Self::InvalidS3BucketFault(_inner) => Some(_inner),
            Self::InvalidSubnet(_inner) => Some(_inner),
            Self::InvalidVpcNetworkStateFault(_inner) => Some(_inner),
            Self::KmsKeyNotAccessibleFault(_inner) => Some(_inner),
            Self::StorageQuotaExceededFault(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::restore_db_cluster_from_s3::_restore_db_cluster_from_s3_output::RestoreDbClusterFromS3Output;

pub use crate::operation::restore_db_cluster_from_s3::_restore_db_cluster_from_s3_input::RestoreDbClusterFromS3Input;

mod _restore_db_cluster_from_s3_input;

mod _restore_db_cluster_from_s3_output;

/// Builders
pub mod builders;
