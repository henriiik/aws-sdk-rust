// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_resource::UntagResourceOutput,
    crate::operation::untag_resource::UntagResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::untag_resource::UntagResourceError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => {
            crate::operation::untag_resource::UntagResourceError::ConcurrentModificationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServiceError" => {
            crate::operation::untag_resource::UntagResourceError::InternalServiceFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServiceFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_fault::de_internal_service_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValue" => {
            crate::operation::untag_resource::UntagResourceError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::untag_resource::UntagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_resource::UntagResourceOutput,
    crate::operation::untag_resource::UntagResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::untag_resource::builders::UntagResourceOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
