// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_datalake_auto_enable_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableOutput,
    crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccountNotFoundException" => crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::AccountNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccountNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_not_found_exception::de_account_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
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
        "ValidationException" => crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_datalake_auto_enable_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableOutput,
    crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_datalake_auto_enable::builders::GetDatalakeAutoEnableOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_get_datalake_auto_enable::de_get_datalake_auto_enable(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::get_datalake_auto_enable::GetDatalakeAutoEnableError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_datalake_auto_enable(
    value: &[u8],
    mut builder: crate::operation::get_datalake_auto_enable::builders::GetDatalakeAutoEnableOutputBuilder,
) -> Result<
    crate::operation::get_datalake_auto_enable::builders::GetDatalakeAutoEnableOutputBuilder,
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
                    "autoEnableNewAccounts" => {
                        builder = builder.set_auto_enable_new_accounts(
                            crate::protocol_serde::shape_auto_enable_new_region_configuration_list::de_auto_enable_new_region_configuration_list(tokens)?
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
