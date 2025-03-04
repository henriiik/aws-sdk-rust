// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_load_balancer_metric_data_input(
    input: &crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_load_balancer_metric_data_input::ser_get_load_balancer_metric_data_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_load_balancer_metric_data_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataOutput,
    crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccountSetupInProgressException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::AccountSetupInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccountSetupInProgressExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_setup_in_progress_exception::de_account_setup_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationFailureException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::OperationFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_failure_exception::de_operation_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthenticatedException" => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::UnauthenticatedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthenticatedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthenticated_exception::de_unauthenticated_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_load_balancer_metric_data_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataOutput,
    crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_load_balancer_metric_data::builders::GetLoadBalancerMetricDataOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_load_balancer_metric_data::de_get_load_balancer_metric_data(response.body().as_ref(), output).map_err(crate::operation::get_load_balancer_metric_data::GetLoadBalancerMetricDataError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_load_balancer_metric_data(value: &[u8], mut builder: crate::operation::get_load_balancer_metric_data::builders::GetLoadBalancerMetricDataOutputBuilder) -> Result<crate::operation::get_load_balancer_metric_data::builders::GetLoadBalancerMetricDataOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "metricName" => {
                        builder = builder.set_metric_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::LoadBalancerMetricName::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "metricData" => {
                        builder = builder.set_metric_data(
                            crate::protocol_serde::shape_metric_datapoint_list::de_metric_datapoint_list(tokens)?
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
