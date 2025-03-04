// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_admin_list_devices_input(
    input: &crate::operation::admin_list_devices::AdminListDevicesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_admin_list_devices_input::ser_admin_list_devices_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_admin_list_devices_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::admin_list_devices::AdminListDevicesOutput,
    crate::operation::admin_list_devices::AdminListDevicesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::admin_list_devices::AdminListDevicesError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalErrorException" => crate::operation::admin_list_devices::AdminListDevicesError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::operation::admin_list_devices::AdminListDevicesError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidUserPoolConfigurationException" => crate::operation::admin_list_devices::AdminListDevicesError::InvalidUserPoolConfigurationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidUserPoolConfigurationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_user_pool_configuration_exception::de_invalid_user_pool_configuration_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotAuthorizedException" => crate::operation::admin_list_devices::AdminListDevicesError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotAuthorizedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::admin_list_devices::AdminListDevicesError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::admin_list_devices::AdminListDevicesError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::admin_list_devices::AdminListDevicesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_admin_list_devices_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::admin_list_devices::AdminListDevicesOutput,
    crate::operation::admin_list_devices::AdminListDevicesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::admin_list_devices::builders::AdminListDevicesOutputBuilder::default(
            );
        let _ = response;
        output = crate::protocol_serde::shape_admin_list_devices::de_admin_list_devices(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::admin_list_devices::AdminListDevicesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_admin_list_devices(
    value: &[u8],
    mut builder: crate::operation::admin_list_devices::builders::AdminListDevicesOutputBuilder,
) -> Result<
    crate::operation::admin_list_devices::builders::AdminListDevicesOutputBuilder,
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
                    "Devices" => {
                        builder = builder.set_devices(
                            crate::protocol_serde::shape_device_list_type::de_device_list_type(
                                tokens,
                            )?,
                        );
                    }
                    "PaginationToken" => {
                        builder = builder.set_pagination_token(
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
