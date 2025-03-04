// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p></p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p></p>
    ConflictException(crate::types::error::ConflictException),
    /// <p></p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p></p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p></p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p></p>
    TooManyTagsException(crate::types::error::TooManyTagsException),
    /// <p></p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(aws_smithy_types::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::delete_app::DeleteAppError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::delete_app::DeleteAppError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::delete_app::DeleteAppError> for Error {
    fn from(err: crate::operation::delete_app::DeleteAppError) -> Self {
        match err {
            crate::operation::delete_app::DeleteAppError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::delete_app::DeleteAppError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::delete_app::DeleteAppError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::delete_app::DeleteAppError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::delete_app::DeleteAppError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::delete_app::DeleteAppError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::delete_simulation::DeleteSimulationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::delete_simulation::DeleteSimulationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::delete_simulation::DeleteSimulationError> for Error {
    fn from(err: crate::operation::delete_simulation::DeleteSimulationError) -> Self {
        match err {
            crate::operation::delete_simulation::DeleteSimulationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_simulation::DeleteSimulationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_simulation::DeleteSimulationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_simulation::DeleteSimulationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_simulation::DeleteSimulationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_simulation::DeleteSimulationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::describe_app::DescribeAppError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::describe_app::DescribeAppError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_app::DescribeAppError> for Error {
    fn from(err: crate::operation::describe_app::DescribeAppError) -> Self {
        match err {
            crate::operation::describe_app::DescribeAppError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::describe_app::DescribeAppError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::describe_app::DescribeAppError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::describe_app::DescribeAppError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::describe_app::DescribeAppError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::describe_simulation::DescribeSimulationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::describe_simulation::DescribeSimulationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::describe_simulation::DescribeSimulationError> for Error {
    fn from(err: crate::operation::describe_simulation::DescribeSimulationError) -> Self {
        match err {
            crate::operation::describe_simulation::DescribeSimulationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::describe_simulation::DescribeSimulationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::describe_simulation::DescribeSimulationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::describe_simulation::DescribeSimulationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::describe_simulation::DescribeSimulationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::list_apps::ListAppsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::list_apps::ListAppsError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_apps::ListAppsError> for Error {
    fn from(err: crate::operation::list_apps::ListAppsError) -> Self {
        match err {
            crate::operation::list_apps::ListAppsError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::list_apps::ListAppsError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_apps::ListAppsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_apps::ListAppsError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_apps::ListAppsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_simulations::ListSimulationsError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_simulations::ListSimulationsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_simulations::ListSimulationsError> for Error {
    fn from(err: crate::operation::list_simulations::ListSimulationsError) -> Self {
        match err {
            crate::operation::list_simulations::ListSimulationsError::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::operation::list_simulations::ListSimulationsError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::list_simulations::ListSimulationsError::ValidationException(
                inner,
            ) => Error::ValidationException(inner),
            crate::operation::list_simulations::ListSimulationsError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::list_tags_for_resource::ListTagsForResourceError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::start_app::StartAppError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::start_app::StartAppError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_app::StartAppError> for Error {
    fn from(err: crate::operation::start_app::StartAppError) -> Self {
        match err {
            crate::operation::start_app::StartAppError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::start_app::StartAppError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::start_app::StartAppError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::start_app::StartAppError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::start_app::StartAppError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::start_app::StartAppError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::start_clock::StartClockError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::start_clock::StartClockError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_clock::StartClockError> for Error {
    fn from(err: crate::operation::start_clock::StartClockError) -> Self {
        match err {
            crate::operation::start_clock::StartClockError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::start_clock::StartClockError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::start_clock::StartClockError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::start_clock::StartClockError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::start_clock::StartClockError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::start_clock::StartClockError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::start_simulation::StartSimulationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::start_simulation::StartSimulationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::start_simulation::StartSimulationError> for Error {
    fn from(err: crate::operation::start_simulation::StartSimulationError) -> Self {
        match err {
            crate::operation::start_simulation::StartSimulationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::start_simulation::StartSimulationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::start_simulation::StartSimulationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_simulation::StartSimulationError::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::operation::start_simulation::StartSimulationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::start_simulation::StartSimulationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::stop_app::StopAppError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::stop_app::StopAppError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::stop_app::StopAppError> for Error {
    fn from(err: crate::operation::stop_app::StopAppError) -> Self {
        match err {
            crate::operation::stop_app::StopAppError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::stop_app::StopAppError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::stop_app::StopAppError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::stop_app::StopAppError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::stop_app::StopAppError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::stop_app::StopAppError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::stop_clock::StopClockError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::stop_clock::StopClockError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::stop_clock::StopClockError> for Error {
    fn from(err: crate::operation::stop_clock::StopClockError) -> Self {
        match err {
            crate::operation::stop_clock::StopClockError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::stop_clock::StopClockError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::stop_clock::StopClockError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::stop_clock::StopClockError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::stop_clock::StopClockError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::stop_clock::StopClockError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<
        aws_smithy_http::result::SdkError<
            crate::operation::stop_simulation::StopSimulationError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::stop_simulation::StopSimulationError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::stop_simulation::StopSimulationError> for Error {
    fn from(err: crate::operation::stop_simulation::StopSimulationError) -> Self {
        match err {
            crate::operation::stop_simulation::StopSimulationError::AccessDeniedException(
                inner,
            ) => Error::AccessDeniedException(inner),
            crate::operation::stop_simulation::StopSimulationError::ConflictException(inner) => {
                Error::ConflictException(inner)
            }
            crate::operation::stop_simulation::StopSimulationError::InternalServerException(
                inner,
            ) => Error::InternalServerException(inner),
            crate::operation::stop_simulation::StopSimulationError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::stop_simulation::StopSimulationError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::stop_simulation::StopSimulationError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::operation::tag_resource::TagResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::operation::tag_resource::TagResourceError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::tag_resource::TagResourceError::TooManyTagsException(inner) => {
                Error::TooManyTagsException(inner)
            }
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::operation::untag_resource::UntagResourceError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(
                aws_smithy_types::error::Unhandled::builder()
                    .meta(
                        aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                    )
                    .source(err)
                    .build(),
            ),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(
                inner,
            ) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => {
                Error::Unhandled(inner)
            }
        }
    }
}
impl std::error::Error for Error {}
impl aws_http::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::TooManyTagsException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.request_id(),
        }
    }
}
