// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_pipeline_input(
    input: &crate::operation::delete_pipeline::DeletePipelineInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_pipeline_input::ser_delete_pipeline_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_pipeline_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_pipeline::DeletePipelineOutput,
    crate::operation::delete_pipeline::DeletePipelineError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_pipeline::DeletePipelineError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_pipeline::DeletePipelineError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceError" => {
            crate::operation::delete_pipeline::DeletePipelineError::InternalServiceError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServiceErrorBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error::de_internal_service_error_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_pipeline::DeletePipelineError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::delete_pipeline::DeletePipelineError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_pipeline::DeletePipelineError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "PipelineNotFoundException" => {
            crate::operation::delete_pipeline::DeletePipelineError::PipelineNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::PipelineNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pipeline_not_found_exception::de_pipeline_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_pipeline::DeletePipelineError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::delete_pipeline::DeletePipelineError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_pipeline_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_pipeline::DeletePipelineOutput,
    crate::operation::delete_pipeline::DeletePipelineError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::delete_pipeline::builders::DeletePipelineOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
