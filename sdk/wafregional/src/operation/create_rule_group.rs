// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl CreateRuleGroupInput {
    /// Consumes the builder and constructs an Operation<[`CreateRuleGroup`](crate::operation::create_rule_group::CreateRuleGroup)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::create_rule_group::CreateRuleGroup,
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
                _input: &crate::operation::create_rule_group::CreateRuleGroupInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::create_rule_group::CreateRuleGroupInput,
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
                "application/x-amz-json-1.1",
            );
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::HeaderName::from_static("x-amz-target"),
                "AWSWAF_Regional_20161128.CreateRuleGroup",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_create_rule_group::ser_create_rule_group_input(&self)?,
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
            crate::operation::create_rule_group::CreateRuleGroup::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CreateRuleGroup",
            "wafregional",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `CreateRuleGroup`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct CreateRuleGroup;
impl CreateRuleGroup {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRuleGroup {
    type Output = std::result::Result<
        crate::operation::create_rule_group::CreateRuleGroupOutput,
        crate::operation::create_rule_group::CreateRuleGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_create_rule_group::de_create_rule_group_http_error(
                response,
            )
        } else {
            crate::protocol_serde::shape_create_rule_group::de_create_rule_group_http_response(
                response,
            )
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type CreateRuleGroupErrorKind = CreateRuleGroupError;
/// Error type for the `CreateRuleGroupError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum CreateRuleGroupError {
    /// <p></p>
    WafBadRequestException(crate::types::error::WafBadRequestException),
    /// <p>The name specified is invalid.</p>
    WafDisallowedNameException(crate::types::error::WafDisallowedNameException),
    /// <p>The operation failed because of a system problem, even though the request was valid. Retry your request.</p>
    WafInternalErrorException(crate::types::error::WafInternalErrorException),
    /// <p>The operation exceeds a resource limit, for example, the maximum number of <code>WebACL</code> objects that you can create for an AWS account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">Limits</a> in the <i>AWS WAF Developer Guide</i>.</p>
    WafLimitsExceededException(crate::types::error::WafLimitsExceededException),
    /// <p>The operation failed because you tried to create, update, or delete an object by using a change token that has already been used.</p>
    WafStaleDataException(crate::types::error::WafStaleDataException),
    /// <p></p>
    WafTagOperationException(crate::types::error::WafTagOperationException),
    /// <p></p>
    WafTagOperationInternalErrorException(
        crate::types::error::WafTagOperationInternalErrorException,
    ),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for CreateRuleGroupError {
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
impl std::fmt::Display for CreateRuleGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WafBadRequestException(_inner) => _inner.fmt(f),
            Self::WafDisallowedNameException(_inner) => _inner.fmt(f),
            Self::WafInternalErrorException(_inner) => _inner.fmt(f),
            Self::WafLimitsExceededException(_inner) => _inner.fmt(f),
            Self::WafStaleDataException(_inner) => _inner.fmt(f),
            Self::WafTagOperationException(_inner) => _inner.fmt(f),
            Self::WafTagOperationInternalErrorException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for CreateRuleGroupError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::WafBadRequestException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafDisallowedNameException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInternalErrorException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafLimitsExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafStaleDataException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafTagOperationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafTagOperationInternalErrorException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::create_rule_group::CreateRuleGroupError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateRuleGroupError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateRuleGroupError {
    /// Creates the `CreateRuleGroupError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `CreateRuleGroupError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::WafBadRequestException(e) => e.meta(),
            Self::WafDisallowedNameException(e) => e.meta(),
            Self::WafInternalErrorException(e) => e.meta(),
            Self::WafLimitsExceededException(e) => e.meta(),
            Self::WafStaleDataException(e) => e.meta(),
            Self::WafTagOperationException(e) => e.meta(),
            Self::WafTagOperationInternalErrorException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafBadRequestException`.
    pub fn is_waf_bad_request_exception(&self) -> bool {
        matches!(self, Self::WafBadRequestException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafDisallowedNameException`.
    pub fn is_waf_disallowed_name_exception(&self) -> bool {
        matches!(self, Self::WafDisallowedNameException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafInternalErrorException`.
    pub fn is_waf_internal_error_exception(&self) -> bool {
        matches!(self, Self::WafInternalErrorException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafLimitsExceededException`.
    pub fn is_waf_limits_exceeded_exception(&self) -> bool {
        matches!(self, Self::WafLimitsExceededException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafStaleDataException`.
    pub fn is_waf_stale_data_exception(&self) -> bool {
        matches!(self, Self::WafStaleDataException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafTagOperationException`.
    pub fn is_waf_tag_operation_exception(&self) -> bool {
        matches!(self, Self::WafTagOperationException(_))
    }
    /// Returns `true` if the error kind is `CreateRuleGroupError::WafTagOperationInternalErrorException`.
    pub fn is_waf_tag_operation_internal_error_exception(&self) -> bool {
        matches!(self, Self::WafTagOperationInternalErrorException(_))
    }
}
impl std::error::Error for CreateRuleGroupError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WafBadRequestException(_inner) => Some(_inner),
            Self::WafDisallowedNameException(_inner) => Some(_inner),
            Self::WafInternalErrorException(_inner) => Some(_inner),
            Self::WafLimitsExceededException(_inner) => Some(_inner),
            Self::WafStaleDataException(_inner) => Some(_inner),
            Self::WafTagOperationException(_inner) => Some(_inner),
            Self::WafTagOperationInternalErrorException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::create_rule_group::_create_rule_group_output::CreateRuleGroupOutput;

pub use crate::operation::create_rule_group::_create_rule_group_input::CreateRuleGroupInput;

mod _create_rule_group_input;

mod _create_rule_group_output;

/// Builders
pub mod builders;
