// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl DeleteIntentInput {
    /// Consumes the builder and constructs an Operation<[`DeleteIntent`](crate::operation::delete_intent::DeleteIntent)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::delete_intent::DeleteIntent,
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
                _input: &crate::operation::delete_intent::DeleteIntentInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.bot_id;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "bot_id",
                        "cannot be empty or unset",
                    )
                })?;
                let bot_id = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if bot_id.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "bot_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                let input_2 = &_input.bot_version;
                let input_2 = input_2.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "bot_version",
                        "cannot be empty or unset",
                    )
                })?;
                let bot_version = aws_smithy_http::label::fmt_string(
                    input_2,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if bot_version.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "bot_version",
                            "cannot be empty or unset",
                        ),
                    );
                }
                let input_3 = &_input.locale_id;
                let input_3 = input_3.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "locale_id",
                        "cannot be empty or unset",
                    )
                })?;
                let locale_id = aws_smithy_http::label::fmt_string(
                    input_3,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if locale_id.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "locale_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                let input_4 = &_input.intent_id;
                let input_4 = input_4.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "intent_id",
                        "cannot be empty or unset",
                    )
                })?;
                let intent_id = aws_smithy_http::label::fmt_string(
                    input_4,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if intent_id.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "intent_id",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(output, "/bots/{botId}/botversions/{botVersion}/botlocales/{localeId}/intents/{intentId}", botId = bot_id, botVersion = bot_version, localeId = locale_id, intentId = intent_id).expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_intent::DeleteIntentInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("DELETE").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
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
            crate::operation::delete_intent::DeleteIntent::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DeleteIntent",
            "lexmodelsv2",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DeleteIntent`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DeleteIntent;
impl DeleteIntent {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteIntent {
    type Output = std::result::Result<
        crate::operation::delete_intent::DeleteIntentOutput,
        crate::operation::delete_intent::DeleteIntentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::protocol_serde::shape_delete_intent::de_delete_intent_http_error(response)
        } else {
            crate::protocol_serde::shape_delete_intent::de_delete_intent_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DeleteIntentErrorKind = DeleteIntentError;
/// Error type for the `DeleteIntentError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteIntentError {
    /// <p>The action that you tried to perform couldn't be completed because the resource is in a conflicting state. For example, deleting a bot that is in the CREATING state. Try your request again. </p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>The service encountered an unexpected condition. Try your request again.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>Your request couldn't be completed because one or more request fields aren't valid. Check the fields in your request and try again.</p>
    PreconditionFailedException(crate::types::error::PreconditionFailedException),
    /// <p>You have reached a quota for your bot. </p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>Your request rate is too high. Reduce the frequency of requests.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>One of the input parameters in your request isn't valid. Check the parameters and try your request again.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for DeleteIntentError {
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
impl std::fmt::Display for DeleteIntentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConflictException(_inner) => _inner.fmt(f),
            Self::InternalServerException(_inner) => _inner.fmt(f),
            Self::PreconditionFailedException(_inner) => _inner.fmt(f),
            Self::ServiceQuotaExceededException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::ValidationException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteIntentError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::ConflictException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServerException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::PreconditionFailedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ServiceQuotaExceededException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ThrottlingException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ValidationException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId for crate::operation::delete_intent::DeleteIntentError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteIntentError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteIntentError {
    /// Creates the `DeleteIntentError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `DeleteIntentError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::ConflictException(e) => e.meta(),
            Self::InternalServerException(e) => e.meta(),
            Self::PreconditionFailedException(e) => e.meta(),
            Self::ServiceQuotaExceededException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DeleteIntentError::ConflictException`.
    pub fn is_conflict_exception(&self) -> bool {
        matches!(self, Self::ConflictException(_))
    }
    /// Returns `true` if the error kind is `DeleteIntentError::InternalServerException`.
    pub fn is_internal_server_exception(&self) -> bool {
        matches!(self, Self::InternalServerException(_))
    }
    /// Returns `true` if the error kind is `DeleteIntentError::PreconditionFailedException`.
    pub fn is_precondition_failed_exception(&self) -> bool {
        matches!(self, Self::PreconditionFailedException(_))
    }
    /// Returns `true` if the error kind is `DeleteIntentError::ServiceQuotaExceededException`.
    pub fn is_service_quota_exceeded_exception(&self) -> bool {
        matches!(self, Self::ServiceQuotaExceededException(_))
    }
    /// Returns `true` if the error kind is `DeleteIntentError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
    /// Returns `true` if the error kind is `DeleteIntentError::ValidationException`.
    pub fn is_validation_exception(&self) -> bool {
        matches!(self, Self::ValidationException(_))
    }
}
impl std::error::Error for DeleteIntentError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConflictException(_inner) => Some(_inner),
            Self::InternalServerException(_inner) => Some(_inner),
            Self::PreconditionFailedException(_inner) => Some(_inner),
            Self::ServiceQuotaExceededException(_inner) => Some(_inner),
            Self::ThrottlingException(_inner) => Some(_inner),
            Self::ValidationException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::delete_intent::_delete_intent_output::DeleteIntentOutput;

pub use crate::operation::delete_intent::_delete_intent_input::DeleteIntentInput;

mod _delete_intent_input;

mod _delete_intent_output;

/// Builders
pub mod builders;
