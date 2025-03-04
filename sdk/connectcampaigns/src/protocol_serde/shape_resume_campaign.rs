// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_resume_campaign_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::resume_campaign::ResumeCampaignOutput,
    crate::operation::resume_campaign::ResumeCampaignError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::resume_campaign::ResumeCampaignError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::resume_campaign::ResumeCampaignError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_access_denied_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ConflictException" => {
            crate::operation::resume_campaign::ResumeCampaignError::ConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_conflict_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::resume_campaign::ResumeCampaignError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_internal_server_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidCampaignStateException" => {
            crate::operation::resume_campaign::ResumeCampaignError::InvalidCampaignStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidCampaignStateExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_campaign_state_exception::de_invalid_campaign_state_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_invalid_campaign_state_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
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
            crate::operation::resume_campaign::ResumeCampaignError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_resource_not_found_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::resume_campaign::ResumeCampaignError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_throttling_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::resume_campaign::ResumeCampaignError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::resume_campaign::ResumeCampaignError::unhandled)?;
                    output = output.set_x_amz_error_type(
                        crate::protocol_serde::shape_validation_exception::de_x_amz_error_type_header(response.headers())
                                                .map_err(|_|crate::operation::resume_campaign::ResumeCampaignError::unhandled("Failed to parse xAmzErrorType from header `x-amzn-ErrorType"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::resume_campaign::ResumeCampaignError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_resume_campaign_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::resume_campaign::ResumeCampaignOutput,
    crate::operation::resume_campaign::ResumeCampaignError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::resume_campaign::builders::ResumeCampaignOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
