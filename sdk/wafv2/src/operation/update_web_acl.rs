// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl UpdateWebAclInput {
    /// Consumes the builder and constructs an Operation<[`UpdateWebACL`](crate::operation::update_web_acl::UpdateWebACL)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::update_web_acl::UpdateWebACL,
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
                _input: &crate::operation::update_web_acl::UpdateWebAclInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::update_web_acl::UpdateWebAclInput,
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
                "AWSWAF_20190729.UpdateWebACL",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_update_web_acl::ser_update_web_acl_input(&self)?,
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
            crate::operation::update_web_acl::UpdateWebACL::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "UpdateWebACL",
            "wafv2",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `UpdateWebACL`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct UpdateWebACL;
impl UpdateWebACL {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateWebACL {
    type Output = std::result::Result<
        crate::operation::update_web_acl::UpdateWebAclOutput,
        crate::operation::update_web_acl::UpdateWebACLError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_update_web_acl::de_update_web_acl_http_error(response)
        } else {
            crate::protocol_serde::shape_update_web_acl::de_update_web_acl_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type UpdateWebACLErrorKind = UpdateWebACLError;
/// Error type for the `UpdateWebACLError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum UpdateWebACLError {
    /// <p>The operation failed because you are inspecting the web request body, headers, or cookies without specifying how to handle oversize components. Rules that inspect the body must either provide an <code>OversizeHandling</code> configuration or they must be preceded by a <code>SizeConstraintStatement</code> that blocks the body content from being too large. Rules that inspect the headers or cookies must provide an <code>OversizeHandling</code> configuration. </p>
    /// <p>Provide the handling configuration and retry your operation.</p>
    /// <p>Alternately, you can suppress this warning by adding the following tag to the resource that you provide to this operation: <code>Tag</code> (key:<code>WAF:OversizeFieldsHandlingConstraintOptOut</code>, value:<code>true</code>).</p>
    WafConfigurationWarningException(crate::types::error::WafConfigurationWarningException),
    /// <p>WAF couldn’t perform the operation because the resource that you tried to save is a duplicate of an existing one.</p>
    WafDuplicateItemException(crate::types::error::WafDuplicateItemException),
    /// <p>The operation failed because the specified version for the managed rule group has expired. You can retrieve the available versions for the managed rule group by calling <code>ListAvailableManagedRuleGroupVersions</code>.</p>
    WafExpiredManagedRuleGroupVersionException(
        crate::types::error::WafExpiredManagedRuleGroupVersionException,
    ),
    /// <p>Your request is valid, but WAF couldn’t perform the operation because of a system problem. Retry your request. </p>
    WafInternalErrorException(crate::types::error::WafInternalErrorException),
    /// <p>The operation isn't valid. </p>
    WafInvalidOperationException(crate::types::error::WafInvalidOperationException),
    /// <p>The operation failed because WAF didn't recognize a parameter in the request. For example: </p>
    /// <ul>
    /// <li> <p>You specified a parameter name or value that isn't valid.</p> </li>
    /// <li> <p>Your nested statement isn't valid. You might have tried to nest a statement that can’t be nested. </p> </li>
    /// <li> <p>You tried to update a <code>WebACL</code> with a <code>DefaultAction</code> that isn't among the types available at <code>DefaultAction</code>.</p> </li>
    /// <li> <p>Your request references an ARN that is malformed, or corresponds to a resource with which a web ACL can't be associated.</p> </li>
    /// </ul>
    WafInvalidParameterException(crate::types::error::WafInvalidParameterException),
    /// <p>WAF couldn’t perform the operation because the resource that you requested isn’t valid. Check the resource, and try again.</p>
    WafInvalidResourceException(crate::types::error::WafInvalidResourceException),
    /// <p>WAF couldn’t perform the operation because you exceeded your resource limit. For example, the maximum number of <code>WebACL</code> objects that you can create for an Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> in the <i>WAF Developer Guide</i>.</p>
    WafLimitsExceededException(crate::types::error::WafLimitsExceededException),
    /// <p>WAF couldn’t perform the operation because your resource doesn't exist. If you've just created a resource that you're using in this operation, you might just need to wait a few minutes. It can take from a few seconds to a number of minutes for changes to propagate. </p>
    WafNonexistentItemException(crate::types::error::WafNonexistentItemException),
    /// <p>WAF couldn’t save your changes because you tried to update or delete a resource that has changed since you last retrieved it. Get the resource again, make any changes you need to make to the new copy, and retry your operation. </p>
    WafOptimisticLockException(crate::types::error::WafOptimisticLockException),
    /// <p>You tried to use a managed rule group that's available by subscription, but you aren't subscribed to it yet. </p>
    WafSubscriptionNotFoundException(crate::types::error::WafSubscriptionNotFoundException),
    /// <p>WAF couldn’t retrieve a resource that you specified for this operation. If you've just created a resource that you're using in this operation, you might just need to wait a few minutes. It can take from a few seconds to a number of minutes for changes to propagate. Verify the resources that you are specifying in your request parameters and then retry the operation.</p>
    WafUnavailableEntityException(crate::types::error::WafUnavailableEntityException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for UpdateWebACLError {
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
impl std::fmt::Display for UpdateWebACLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WafConfigurationWarningException(_inner) => _inner.fmt(f),
            Self::WafDuplicateItemException(_inner) => _inner.fmt(f),
            Self::WafExpiredManagedRuleGroupVersionException(_inner) => _inner.fmt(f),
            Self::WafInternalErrorException(_inner) => _inner.fmt(f),
            Self::WafInvalidOperationException(_inner) => _inner.fmt(f),
            Self::WafInvalidParameterException(_inner) => _inner.fmt(f),
            Self::WafInvalidResourceException(_inner) => _inner.fmt(f),
            Self::WafLimitsExceededException(_inner) => _inner.fmt(f),
            Self::WafNonexistentItemException(_inner) => _inner.fmt(f),
            Self::WafOptimisticLockException(_inner) => _inner.fmt(f),
            Self::WafSubscriptionNotFoundException(_inner) => _inner.fmt(f),
            Self::WafUnavailableEntityException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for UpdateWebACLError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::WafConfigurationWarningException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafDuplicateItemException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafExpiredManagedRuleGroupVersionException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInternalErrorException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInvalidOperationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInvalidParameterException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafInvalidResourceException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafLimitsExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafNonexistentItemException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafOptimisticLockException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafSubscriptionNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::WafUnavailableEntityException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::update_web_acl::UpdateWebACLError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for UpdateWebACLError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl UpdateWebACLError {
    /// Creates the `UpdateWebACLError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `UpdateWebACLError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::WafConfigurationWarningException(e) => e.meta(),
            Self::WafDuplicateItemException(e) => e.meta(),
            Self::WafExpiredManagedRuleGroupVersionException(e) => e.meta(),
            Self::WafInternalErrorException(e) => e.meta(),
            Self::WafInvalidOperationException(e) => e.meta(),
            Self::WafInvalidParameterException(e) => e.meta(),
            Self::WafInvalidResourceException(e) => e.meta(),
            Self::WafLimitsExceededException(e) => e.meta(),
            Self::WafNonexistentItemException(e) => e.meta(),
            Self::WafOptimisticLockException(e) => e.meta(),
            Self::WafSubscriptionNotFoundException(e) => e.meta(),
            Self::WafUnavailableEntityException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafConfigurationWarningException`.
    pub fn is_waf_configuration_warning_exception(&self) -> bool {
        matches!(self, Self::WafConfigurationWarningException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafDuplicateItemException`.
    pub fn is_waf_duplicate_item_exception(&self) -> bool {
        matches!(self, Self::WafDuplicateItemException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafExpiredManagedRuleGroupVersionException`.
    pub fn is_waf_expired_managed_rule_group_version_exception(&self) -> bool {
        matches!(self, Self::WafExpiredManagedRuleGroupVersionException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafInternalErrorException`.
    pub fn is_waf_internal_error_exception(&self) -> bool {
        matches!(self, Self::WafInternalErrorException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafInvalidOperationException`.
    pub fn is_waf_invalid_operation_exception(&self) -> bool {
        matches!(self, Self::WafInvalidOperationException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafInvalidParameterException`.
    pub fn is_waf_invalid_parameter_exception(&self) -> bool {
        matches!(self, Self::WafInvalidParameterException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafInvalidResourceException`.
    pub fn is_waf_invalid_resource_exception(&self) -> bool {
        matches!(self, Self::WafInvalidResourceException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafLimitsExceededException`.
    pub fn is_waf_limits_exceeded_exception(&self) -> bool {
        matches!(self, Self::WafLimitsExceededException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafNonexistentItemException`.
    pub fn is_waf_nonexistent_item_exception(&self) -> bool {
        matches!(self, Self::WafNonexistentItemException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafOptimisticLockException`.
    pub fn is_waf_optimistic_lock_exception(&self) -> bool {
        matches!(self, Self::WafOptimisticLockException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafSubscriptionNotFoundException`.
    pub fn is_waf_subscription_not_found_exception(&self) -> bool {
        matches!(self, Self::WafSubscriptionNotFoundException(_))
    }
    /// Returns `true` if the error kind is `UpdateWebACLError::WafUnavailableEntityException`.
    pub fn is_waf_unavailable_entity_exception(&self) -> bool {
        matches!(self, Self::WafUnavailableEntityException(_))
    }
}
impl std::error::Error for UpdateWebACLError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WafConfigurationWarningException(_inner) => Some(_inner),
            Self::WafDuplicateItemException(_inner) => Some(_inner),
            Self::WafExpiredManagedRuleGroupVersionException(_inner) => Some(_inner),
            Self::WafInternalErrorException(_inner) => Some(_inner),
            Self::WafInvalidOperationException(_inner) => Some(_inner),
            Self::WafInvalidParameterException(_inner) => Some(_inner),
            Self::WafInvalidResourceException(_inner) => Some(_inner),
            Self::WafLimitsExceededException(_inner) => Some(_inner),
            Self::WafNonexistentItemException(_inner) => Some(_inner),
            Self::WafOptimisticLockException(_inner) => Some(_inner),
            Self::WafSubscriptionNotFoundException(_inner) => Some(_inner),
            Self::WafUnavailableEntityException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::update_web_acl::_update_web_acl_output::UpdateWebAclOutput;

pub use crate::operation::update_web_acl::_update_web_acl_input::UpdateWebAclInput;

mod _update_web_acl_input;

mod _update_web_acl_output;

/// Builders
pub mod builders;
