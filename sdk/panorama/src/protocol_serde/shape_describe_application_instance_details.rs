// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_application_instance_details_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsOutput, crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConflictException" => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled("Failed to parse RetryAfterSeconds from header `Retry-After"))?
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
        "ResourceNotFoundException" => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_application_instance_details_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsOutput, crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_application_instance_details::builders::DescribeApplicationInstanceDetailsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_application_instance_details::de_describe_application_instance_details(response.body().as_ref(), output).map_err(crate::operation::describe_application_instance_details::DescribeApplicationInstanceDetailsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_application_instance_details(value: &[u8], mut builder: crate::operation::describe_application_instance_details::builders::DescribeApplicationInstanceDetailsOutputBuilder) -> Result<crate::operation::describe_application_instance_details::builders::DescribeApplicationInstanceDetailsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "ApplicationInstanceId" => {
                        builder = builder.set_application_instance_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "ApplicationInstanceIdToReplace" => {
                        builder = builder.set_application_instance_id_to_replace(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "CreatedTime" => {
                        builder = builder.set_created_time(
                            aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                aws_smithy_types::date_time::Format::EpochSeconds,
                            )?,
                        );
                    }
                    "DefaultRuntimeContextDevice" => {
                        builder = builder.set_default_runtime_context_device(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "ManifestOverridesPayload" => {
                        builder = builder.set_manifest_overrides_payload(
                            crate::protocol_serde::shape_manifest_overrides_payload::de_manifest_overrides_payload(tokens)?
                        );
                    }
                    "ManifestPayload" => {
                        builder = builder.set_manifest_payload(
                            crate::protocol_serde::shape_manifest_payload::de_manifest_payload(
                                tokens,
                            )?,
                        );
                    }
                    "Name" => {
                        builder = builder.set_name(
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
