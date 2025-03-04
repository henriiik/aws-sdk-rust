// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_function_event_invoke_config_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput,
    crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceConflictException" => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_function_event_invoke_config_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigOutput,
    crate::operation::delete_function_event_invoke_config::DeleteFunctionEventInvokeConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_function_event_invoke_config::builders::DeleteFunctionEventInvokeConfigOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
