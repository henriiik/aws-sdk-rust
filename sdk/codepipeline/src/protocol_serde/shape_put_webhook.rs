// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_webhook_input(
    input: &crate::operation::put_webhook::PutWebhookInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_webhook_input::ser_put_webhook_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_webhook_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_webhook::PutWebhookOutput,
    crate::operation::put_webhook::PutWebhookError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::put_webhook::PutWebhookError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModificationException" => crate::operation::put_webhook::PutWebhookError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTagsException" => crate::operation::put_webhook::PutWebhookError::InvalidTagsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidTagsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_tags_exception::de_invalid_tags_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidWebhookAuthenticationParametersException" => crate::operation::put_webhook::PutWebhookError::InvalidWebhookAuthenticationParametersException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidWebhookAuthenticationParametersExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_webhook_authentication_parameters_exception::de_invalid_webhook_authentication_parameters_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidWebhookFilterPatternException" => crate::operation::put_webhook::PutWebhookError::InvalidWebhookFilterPatternException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidWebhookFilterPatternExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_webhook_filter_pattern_exception::de_invalid_webhook_filter_pattern_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::operation::put_webhook::PutWebhookError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PipelineNotFoundException" => crate::operation::put_webhook::PutWebhookError::PipelineNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PipelineNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pipeline_not_found_exception::de_pipeline_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyTagsException" => crate::operation::put_webhook::PutWebhookError::TooManyTagsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyTagsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::put_webhook::PutWebhookError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::put_webhook::PutWebhookError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_webhook_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::put_webhook::PutWebhookOutput,
    crate::operation::put_webhook::PutWebhookError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::put_webhook::builders::PutWebhookOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_webhook::de_put_webhook(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::put_webhook::PutWebhookError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_put_webhook(
    value: &[u8],
    mut builder: crate::operation::put_webhook::builders::PutWebhookOutputBuilder,
) -> Result<
    crate::operation::put_webhook::builders::PutWebhookOutputBuilder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned =
        aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value))
            .peekable();
    let tokens = &mut tokens_owned;
    aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "webhook" => {
                        builder = builder.set_webhook(
                            crate::protocol_serde::shape_list_webhook_item::de_list_webhook_item(
                                tokens,
                            )?,
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                }
            }
            other => {
                return Err(
                    aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )),
                )
            }
        }
    }
    if tokens.next().is_some() {
        return Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "found more JSON tokens after completing parsing",
            ),
        );
    }
    Ok(builder)
}
