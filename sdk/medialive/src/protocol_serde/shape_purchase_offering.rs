// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_purchase_offering_input(
    input: &crate::operation::purchase_offering::PurchaseOfferingInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_purchase_offering_input::ser_purchase_offering_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_purchase_offering_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::purchase_offering::PurchaseOfferingOutput,
    crate::operation::purchase_offering::PurchaseOfferingError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::purchase_offering::PurchaseOfferingError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadGatewayException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::BadGatewayException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::BadGatewayExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_gateway_exception::de_bad_gateway_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "BadRequestException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::BadRequestExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
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
            crate::operation::purchase_offering::PurchaseOfferingError::ConflictException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ForbiddenException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::ForbiddenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ForbiddenExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "GatewayTimeoutException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::GatewayTimeoutException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::GatewayTimeoutExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_gateway_timeout_exception::de_gateway_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerErrorException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::InternalServerErrorException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "NotFoundException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::NotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::purchase_offering::PurchaseOfferingError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::purchase_offering::PurchaseOfferingError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_purchase_offering_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::purchase_offering::PurchaseOfferingOutput,
    crate::operation::purchase_offering::PurchaseOfferingError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::purchase_offering::builders::PurchaseOfferingOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_purchase_offering::de_purchase_offering(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::purchase_offering::PurchaseOfferingError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_purchase_offering(
    value: &[u8],
    mut builder: crate::operation::purchase_offering::builders::PurchaseOfferingOutputBuilder,
) -> Result<
    crate::operation::purchase_offering::builders::PurchaseOfferingOutputBuilder,
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
                    "reservation" => {
                        builder = builder.set_reservation(
                            crate::protocol_serde::shape_reservation::de_reservation(tokens)?,
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
