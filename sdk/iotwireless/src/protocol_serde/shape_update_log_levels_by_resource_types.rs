// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_log_levels_by_resource_types_input(
    input: &crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_log_levels_by_resource_types_input::ser_update_log_levels_by_resource_types_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_log_levels_by_resource_types_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesOutput,
    crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConflictException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_log_levels_by_resource_types_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesOutput,
    crate::operation::update_log_levels_by_resource_types::UpdateLogLevelsByResourceTypesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_log_levels_by_resource_types::builders::UpdateLogLevelsByResourceTypesOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
