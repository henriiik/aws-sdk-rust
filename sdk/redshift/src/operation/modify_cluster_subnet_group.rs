// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl ModifyClusterSubnetGroupInput {
    /// Consumes the builder and constructs an Operation<[`ModifyClusterSubnetGroup`](crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroup)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroup,
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
                _input: &crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroupInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroupInput,
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
            crate::protocol_serde::shape_modify_cluster_subnet_group_input::ser_modify_cluster_subnet_group_input_input(&self)?
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
            crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroup::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "ModifyClusterSubnetGroup",
            "redshift",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `ModifyClusterSubnetGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ModifyClusterSubnetGroup;
impl ModifyClusterSubnetGroup {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ModifyClusterSubnetGroup {
    type Output = std::result::Result<
        crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroupOutput,
        crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_modify_cluster_subnet_group::de_modify_cluster_subnet_group_http_error(response)
        } else {
            crate::protocol_serde::shape_modify_cluster_subnet_group::de_modify_cluster_subnet_group_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type ModifyClusterSubnetGroupErrorKind = ModifyClusterSubnetGroupError;
/// Error type for the `ModifyClusterSubnetGroupError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ModifyClusterSubnetGroupError {
    /// <p>The cluster subnet group name does not refer to an existing cluster subnet group.</p>
    ClusterSubnetGroupNotFoundFault(crate::types::error::ClusterSubnetGroupNotFoundFault),
    /// <p>The request would result in user exceeding the allowed number of subnets in a cluster subnet groups. For information about increasing your quota, go to <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/amazon-redshift-limits.html">Limits in Amazon Redshift</a> in the <i>Amazon Redshift Cluster Management Guide</i>. </p>
    ClusterSubnetQuotaExceededFault(crate::types::error::ClusterSubnetQuotaExceededFault),
    /// <p>The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. Wait and retry the request.</p>
    DependentServiceRequestThrottlingFault(
        crate::types::error::DependentServiceRequestThrottlingFault,
    ),
    /// <p>The requested subnet is not valid, or not all of the subnets are in the same VPC.</p>
    InvalidSubnet(crate::types::error::InvalidSubnet),
    /// <p>A specified subnet is already in use by another cluster.</p>
    SubnetAlreadyInUse(crate::types::error::SubnetAlreadyInUse),
    /// <p>Your account is not authorized to perform the requested operation.</p>
    UnauthorizedOperation(crate::types::error::UnauthorizedOperation),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for ModifyClusterSubnetGroupError {
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
impl std::fmt::Display for ModifyClusterSubnetGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClusterSubnetGroupNotFoundFault(_inner) => _inner.fmt(f),
            Self::ClusterSubnetQuotaExceededFault(_inner) => _inner.fmt(f),
            Self::DependentServiceRequestThrottlingFault(_inner) => _inner.fmt(f),
            Self::InvalidSubnet(_inner) => _inner.fmt(f),
            Self::SubnetAlreadyInUse(_inner) => _inner.fmt(f),
            Self::UnauthorizedOperation(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ModifyClusterSubnetGroupError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ClusterSubnetGroupNotFoundFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ClusterSubnetQuotaExceededFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::DependentServiceRequestThrottlingFault(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidSubnet(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::SubnetAlreadyInUse(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::UnauthorizedOperation(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::modify_cluster_subnet_group::ModifyClusterSubnetGroupError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ModifyClusterSubnetGroupError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ModifyClusterSubnetGroupError {
    /// Creates the `ModifyClusterSubnetGroupError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `ModifyClusterSubnetGroupError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ClusterSubnetGroupNotFoundFault(e) => e.meta(),
            Self::ClusterSubnetQuotaExceededFault(e) => e.meta(),
            Self::DependentServiceRequestThrottlingFault(e) => e.meta(),
            Self::InvalidSubnet(e) => e.meta(),
            Self::SubnetAlreadyInUse(e) => e.meta(),
            Self::UnauthorizedOperation(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::ClusterSubnetGroupNotFoundFault`.
    pub fn is_cluster_subnet_group_not_found_fault(&self) -> bool {
        matches!(self, Self::ClusterSubnetGroupNotFoundFault(_))
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::ClusterSubnetQuotaExceededFault`.
    pub fn is_cluster_subnet_quota_exceeded_fault(&self) -> bool {
        matches!(self, Self::ClusterSubnetQuotaExceededFault(_))
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::DependentServiceRequestThrottlingFault`.
    pub fn is_dependent_service_request_throttling_fault(&self) -> bool {
        matches!(self, Self::DependentServiceRequestThrottlingFault(_))
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::InvalidSubnet`.
    pub fn is_invalid_subnet(&self) -> bool {
        matches!(self, Self::InvalidSubnet(_))
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::SubnetAlreadyInUse`.
    pub fn is_subnet_already_in_use(&self) -> bool {
        matches!(self, Self::SubnetAlreadyInUse(_))
    }
    /// Returns `true` if the error kind is `ModifyClusterSubnetGroupError::UnauthorizedOperation`.
    pub fn is_unauthorized_operation(&self) -> bool {
        matches!(self, Self::UnauthorizedOperation(_))
    }
}
impl std::error::Error for ModifyClusterSubnetGroupError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ClusterSubnetGroupNotFoundFault(_inner) => Some(_inner),
            Self::ClusterSubnetQuotaExceededFault(_inner) => Some(_inner),
            Self::DependentServiceRequestThrottlingFault(_inner) => Some(_inner),
            Self::InvalidSubnet(_inner) => Some(_inner),
            Self::SubnetAlreadyInUse(_inner) => Some(_inner),
            Self::UnauthorizedOperation(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::modify_cluster_subnet_group::_modify_cluster_subnet_group_output::ModifyClusterSubnetGroupOutput;

pub use crate::operation::modify_cluster_subnet_group::_modify_cluster_subnet_group_input::ModifyClusterSubnetGroupInput;

mod _modify_cluster_subnet_group_input;

mod _modify_cluster_subnet_group_output;

/// Builders
pub mod builders;
