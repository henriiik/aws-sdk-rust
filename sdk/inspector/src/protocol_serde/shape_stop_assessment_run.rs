// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_assessment_run_input(
    input: &crate::operation::stop_assessment_run::StopAssessmentRunInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_assessment_run_input::ser_stop_assessment_run_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_assessment_run_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_assessment_run::StopAssessmentRunOutput,
    crate::operation::stop_assessment_run::StopAssessmentRunError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::stop_assessment_run::StopAssessmentRunError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalException" => crate::operation::stop_assessment_run::StopAssessmentRunError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::stop_assessment_run::StopAssessmentRunError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntityException" => crate::operation::stop_assessment_run::StopAssessmentRunError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceTemporarilyUnavailableException" => crate::operation::stop_assessment_run::StopAssessmentRunError::ServiceTemporarilyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceTemporarilyUnavailableExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_temporarily_unavailable_exception::de_service_temporarily_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::stop_assessment_run::StopAssessmentRunError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::stop_assessment_run::StopAssessmentRunError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_assessment_run_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::stop_assessment_run::StopAssessmentRunOutput,
    crate::operation::stop_assessment_run::StopAssessmentRunError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::stop_assessment_run::builders::StopAssessmentRunOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
