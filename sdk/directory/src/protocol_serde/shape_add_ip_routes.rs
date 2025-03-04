// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_ip_routes_input(
    input: &crate::operation::add_ip_routes::AddIpRoutesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_add_ip_routes_input::ser_add_ip_routes_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_ip_routes_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::add_ip_routes::AddIpRoutesOutput,
    crate::operation::add_ip_routes::AddIpRoutesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::ClientException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ClientExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DirectoryUnavailableException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::DirectoryUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DirectoryUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_directory_unavailable_exception::de_directory_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EntityAlreadyExistsException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::EntityAlreadyExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EntityDoesNotExistException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::EntityDoesNotExistException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::EntityDoesNotExistExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_does_not_exist_exception::de_entity_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "IpRouteLimitExceededException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::IpRouteLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IpRouteLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_ip_route_limit_exceeded_exception::de_ip_route_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceException" => {
            crate::operation::add_ip_routes::AddIpRoutesError::ServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::add_ip_routes::AddIpRoutesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::add_ip_routes::AddIpRoutesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_ip_routes_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::add_ip_routes::AddIpRoutesOutput,
    crate::operation::add_ip_routes::AddIpRoutesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::add_ip_routes::builders::AddIpRoutesOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
