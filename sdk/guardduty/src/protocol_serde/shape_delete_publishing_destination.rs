// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_publishing_destination_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_publishing_destination::DeletePublishingDestinationOutput,
    crate::operation::delete_publishing_destination::DeletePublishingDestinationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_publishing_destination::DeletePublishingDestinationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::delete_publishing_destination::DeletePublishingDestinationError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::delete_publishing_destination::DeletePublishingDestinationError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_publishing_destination::DeletePublishingDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::operation::delete_publishing_destination::DeletePublishingDestinationError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_publishing_destination::DeletePublishingDestinationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_publishing_destination::DeletePublishingDestinationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_publishing_destination_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_publishing_destination::DeletePublishingDestinationOutput,
    crate::operation::delete_publishing_destination::DeletePublishingDestinationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_publishing_destination::builders::DeletePublishingDestinationOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
