// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_connector_profile_input(
    input: &crate::operation::update_connector_profile::UpdateConnectorProfileInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_connector_profile_input::ser_update_connector_profile_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_connector_profile_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
    crate::operation::update_connector_profile::UpdateConnectorProfileError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code =
        match generic.code() {
            Some(code) => code,
            None => return Err(
                crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled(
                    generic,
                ),
            ),
        };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictException" => crate::operation::update_connector_profile::UpdateConnectorProfileError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConnectorAuthenticationException" => crate::operation::update_connector_profile::UpdateConnectorProfileError::ConnectorAuthenticationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConnectorAuthenticationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_connector_authentication_exception::de_connector_authentication_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::update_connector_profile::UpdateConnectorProfileError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::update_connector_profile::UpdateConnectorProfileError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::update_connector_profile::UpdateConnectorProfileError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_connector_profile::UpdateConnectorProfileError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_connector_profile_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::update_connector_profile::UpdateConnectorProfileOutput,
    crate::operation::update_connector_profile::UpdateConnectorProfileError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_connector_profile::builders::UpdateConnectorProfileOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_update_connector_profile::de_update_connector_profile(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::update_connector_profile::UpdateConnectorProfileError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_update_connector_profile(
    value: &[u8],
    mut builder: crate::operation::update_connector_profile::builders::UpdateConnectorProfileOutputBuilder,
) -> Result<
    crate::operation::update_connector_profile::builders::UpdateConnectorProfileOutputBuilder,
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
                    "connectorProfileArn" => {
                        builder = builder.set_connector_profile_arn(
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
