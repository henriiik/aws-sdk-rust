// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_estimation_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_cost_estimation::GetCostEstimationOutput,
    crate::operation::get_cost_estimation::GetCostEstimationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::get_cost_estimation::GetCostEstimationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::get_cost_estimation::GetCostEstimationError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
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
            crate::operation::get_cost_estimation::GetCostEstimationError::InternalServerException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::types::error::builders::InternalServerExceptionBuilder::default(
                            );
                        let _ = response;
                        output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
                        output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::get_cost_estimation::GetCostEstimationError::unhandled("Failed to parse RetryAfterSeconds from header `Retry-After"))?
                    );
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
        "ResourceNotFoundException" => {
            crate::operation::get_cost_estimation::GetCostEstimationError::ResourceNotFoundException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
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
        "ThrottlingException" => {
            crate::operation::get_cost_estimation::GetCostEstimationError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::get_cost_estimation::GetCostEstimationError::unhandled("Failed to parse RetryAfterSeconds from header `Retry-After"))?
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
            crate::operation::get_cost_estimation::GetCostEstimationError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_cost_estimation::GetCostEstimationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_estimation_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_cost_estimation::GetCostEstimationOutput,
    crate::operation::get_cost_estimation::GetCostEstimationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_cost_estimation::builders::GetCostEstimationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_cost_estimation::de_get_cost_estimation(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_cost_estimation::GetCostEstimationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_cost_estimation(
    value: &[u8],
    mut builder: crate::operation::get_cost_estimation::builders::GetCostEstimationOutputBuilder,
) -> Result<
    crate::operation::get_cost_estimation::builders::GetCostEstimationOutputBuilder,
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
                    "Costs" => {
                        builder = builder.set_costs(
                            crate::protocol_serde::shape_service_resource_costs::de_service_resource_costs(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "ResourceCollection" => {
                        builder = builder.set_resource_collection(
                            crate::protocol_serde::shape_cost_estimation_resource_collection_filter::de_cost_estimation_resource_collection_filter(tokens)?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::CostEstimationStatus::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "TimeRange" => {
                        builder = builder.set_time_range(
                            crate::protocol_serde::shape_cost_estimation_time_range::de_cost_estimation_time_range(tokens)?
                        );
                    }
                    "TotalCost" => {
                        builder = builder.set_total_cost(
                            aws_smithy_json::deserialize::token::expect_number_or_null(
                                tokens.next(),
                            )?
                            .map(|v| v.to_f64_lossy()),
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
