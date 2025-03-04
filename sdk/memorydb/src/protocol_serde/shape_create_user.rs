// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_input(
    input: &crate::operation::create_user::CreateUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_user_input::ser_create_user_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_user_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_user::CreateUserOutput,
    crate::operation::create_user::CreateUserError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::create_user::CreateUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_user::CreateUserError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DuplicateUserNameFault" => {
            crate::operation::create_user::CreateUserError::DuplicateUserNameFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::DuplicateUserNameFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_user_name_fault::de_duplicate_user_name_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterCombinationException" => {
            crate::operation::create_user::CreateUserError::InvalidParameterCombinationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValueException" => {
            crate::operation::create_user::CreateUserError::InvalidParameterValueException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TagQuotaPerResourceExceeded" => {
            crate::operation::create_user::CreateUserError::TagQuotaPerResourceExceeded({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TagQuotaPerResourceExceededBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_quota_per_resource_exceeded::de_tag_quota_per_resource_exceeded_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UserAlreadyExistsFault" => {
            crate::operation::create_user::CreateUserError::UserAlreadyExistsFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UserAlreadyExistsFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_already_exists_fault::de_user_already_exists_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UserQuotaExceededFault" => {
            crate::operation::create_user::CreateUserError::UserQuotaExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UserQuotaExceededFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_quota_exceeded_fault::de_user_quota_exceeded_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::create_user::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_user::CreateUserError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_user_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::create_user::CreateUserOutput,
    crate::operation::create_user::CreateUserError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::create_user::builders::CreateUserOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_user::de_create_user(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::create_user::CreateUserError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_create_user(
    value: &[u8],
    mut builder: crate::operation::create_user::builders::CreateUserOutputBuilder,
) -> Result<
    crate::operation::create_user::builders::CreateUserOutputBuilder,
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
                    "User" => {
                        builder =
                            builder.set_user(crate::protocol_serde::shape_user::de_user(tokens)?);
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
