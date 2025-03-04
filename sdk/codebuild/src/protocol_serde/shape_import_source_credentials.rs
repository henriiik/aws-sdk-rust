// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_source_credentials_input(
    input: &crate::operation::import_source_credentials::ImportSourceCredentialsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_import_source_credentials_input::ser_import_source_credentials_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_import_source_credentials_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::import_source_credentials::ImportSourceCredentialsOutput,
    crate::operation::import_source_credentials::ImportSourceCredentialsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(
            crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled(
                generic,
            ),
        ),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccountLimitExceededException" => crate::operation::import_source_credentials::ImportSourceCredentialsError::AccountLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccountLimitExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_limit_exceeded_exception::de_account_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::import_source_credentials::ImportSourceCredentialsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceAlreadyExistsException" => crate::operation::import_source_credentials::ImportSourceCredentialsError::ResourceAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceAlreadyExistsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_already_exists_exception::de_resource_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::import_source_credentials::ImportSourceCredentialsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_import_source_credentials_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::import_source_credentials::ImportSourceCredentialsOutput,
    crate::operation::import_source_credentials::ImportSourceCredentialsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::import_source_credentials::builders::ImportSourceCredentialsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_import_source_credentials::de_import_source_credentials(response.body().as_ref(), output).map_err(crate::operation::import_source_credentials::ImportSourceCredentialsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_import_source_credentials(
    value: &[u8],
    mut builder: crate::operation::import_source_credentials::builders::ImportSourceCredentialsOutputBuilder,
) -> Result<
    crate::operation::import_source_credentials::builders::ImportSourceCredentialsOutputBuilder,
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
                    "arn" => {
                        builder = builder.set_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
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
