// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

impl DeleteEventIntegrationInput {
    /// Consumes the builder and constructs an Operation<[`DeleteEventIntegration`](crate::operation::delete_event_integration::DeleteEventIntegration)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::delete_event_integration::DeleteEventIntegration,
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
                _input: &crate::operation::delete_event_integration::DeleteEventIntegrationInput,
                output: &mut String,
            ) -> std::result::Result<(), aws_smithy_http::operation::error::BuildError>
            {
                let input_1 = &_input.name;
                let input_1 = input_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "name",
                        "cannot be empty or unset",
                    )
                })?;
                let name = aws_smithy_http::label::fmt_string(
                    input_1,
                    aws_smithy_http::label::EncodingStrategy::Default,
                );
                if name.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "name",
                            "cannot be empty or unset",
                        ),
                    );
                }
                write!(output, "/eventIntegrations/{Name}", Name = name)
                    .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_event_integration::DeleteEventIntegrationInput,
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
            crate::operation::delete_event_integration::DeleteEventIntegration::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "DeleteEventIntegration",
            "appintegrations",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
}
/// `ParseStrictResponse` impl for `DeleteEventIntegration`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DeleteEventIntegration;
impl DeleteEventIntegration {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEventIntegration {
    type Output = std::result::Result<
        crate::operation::delete_event_integration::DeleteEventIntegrationOutput,
        crate::operation::delete_event_integration::DeleteEventIntegrationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::protocol_serde::shape_delete_event_integration::de_delete_event_integration_http_error(response)
        } else {
            crate::protocol_serde::shape_delete_event_integration::de_delete_event_integration_http_response(response)
        }
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DeleteEventIntegrationErrorKind = DeleteEventIntegrationError;
/// Error type for the `DeleteEventIntegrationError` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteEventIntegrationError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>Request processing failed due to an error or failure with the service.</p>
    InternalServiceError(crate::types::error::InternalServiceError),
    /// <p>The request is not valid. </p>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>The specified resource was not found.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The throttling limit has been exceeded.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl aws_smithy_http::result::CreateUnhandledError for DeleteEventIntegrationError {
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
impl std::fmt::Display for DeleteEventIntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessDeniedException(_inner) => _inner.fmt(f),
            Self::InternalServiceError(_inner) => _inner.fmt(f),
            Self::InvalidRequestException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::ThrottlingException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteEventIntegrationError {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::AccessDeniedException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InternalServiceError(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::InvalidRequestException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ResourceNotFoundException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::ThrottlingException(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
            Self::Unhandled(_inner) => {
                aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner)
            }
        }
    }
}
impl aws_http::request_id::RequestId
    for crate::operation::delete_event_integration::DeleteEventIntegrationError
{
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteEventIntegrationError {
    fn code(&self) -> std::option::Option<&str> {
        aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> std::option::Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteEventIntegrationError {
    /// Creates the `DeleteEventIntegrationError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self::Unhandled(
            aws_smithy_types::error::Unhandled::builder()
                .source(err)
                .build(),
        )
    }

    /// Creates the `DeleteEventIntegrationError::Unhandled` variant from a `aws_smithy_types::error::ErrorMetadata`.
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
            Self::AccessDeniedException(e) => e.meta(),
            Self::InternalServiceError(e) => e.meta(),
            Self::InvalidRequestException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ThrottlingException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DeleteEventIntegrationError::AccessDeniedException`.
    pub fn is_access_denied_exception(&self) -> bool {
        matches!(self, Self::AccessDeniedException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventIntegrationError::InternalServiceError`.
    pub fn is_internal_service_error(&self) -> bool {
        matches!(self, Self::InternalServiceError(_))
    }
    /// Returns `true` if the error kind is `DeleteEventIntegrationError::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventIntegrationError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `DeleteEventIntegrationError::ThrottlingException`.
    pub fn is_throttling_exception(&self) -> bool {
        matches!(self, Self::ThrottlingException(_))
    }
}
impl std::error::Error for DeleteEventIntegrationError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AccessDeniedException(_inner) => Some(_inner),
            Self::InternalServiceError(_inner) => Some(_inner),
            Self::InvalidRequestException(_inner) => Some(_inner),
            Self::ResourceNotFoundException(_inner) => Some(_inner),
            Self::ThrottlingException(_inner) => Some(_inner),
            Self::Unhandled(_inner) => Some(_inner),
        }
    }
}

pub use crate::operation::delete_event_integration::_delete_event_integration_output::DeleteEventIntegrationOutput;

pub use crate::operation::delete_event_integration::_delete_event_integration_input::DeleteEventIntegrationInput;

mod _delete_event_integration_input;

mod _delete_event_integration_output;

/// Builders
pub mod builders;
