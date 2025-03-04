// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_tags_for_domain_input(
    input: &crate::operation::delete_tags_for_domain::DeleteTagsForDomainInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_tags_for_domain_input::ser_delete_tags_for_domain_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_tags_for_domain_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_tags_for_domain::DeleteTagsForDomainOutput,
    crate::operation::delete_tags_for_domain::DeleteTagsForDomainError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationLimitExceeded" => crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::OperationLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationLimitExceededBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_limit_exceeded::de_operation_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedTLD" => crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::UnsupportedTld({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedTldBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_tld::de_unsupported_tld_json_err(response.body().as_ref(), output).map_err(crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::delete_tags_for_domain::DeleteTagsForDomainError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_tags_for_domain_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_tags_for_domain::DeleteTagsForDomainOutput,
    crate::operation::delete_tags_for_domain::DeleteTagsForDomainError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_tags_for_domain::builders::DeleteTagsForDomainOutputBuilder::default();
        let _ = response;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}
