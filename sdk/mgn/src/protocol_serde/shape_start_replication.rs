// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_replication_input(
    input: &crate::operation::start_replication::StartReplicationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_replication_input::ser_start_replication_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_replication_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_replication::StartReplicationOutput,
    crate::operation::start_replication::StartReplicationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::start_replication::StartReplicationError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictException" => crate::operation::start_replication::StartReplicationError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::start_replication::StartReplicationError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::start_replication::StartReplicationError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UninitializedAccountException" => crate::operation::start_replication::StartReplicationError::UninitializedAccountException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UninitializedAccountExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_uninitialized_account_exception::de_uninitialized_account_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::start_replication::StartReplicationError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::start_replication::StartReplicationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_replication_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::start_replication::StartReplicationOutput,
    crate::operation::start_replication::StartReplicationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::start_replication::builders::StartReplicationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_start_replication::de_start_replication(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::start_replication::StartReplicationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_start_replication(
    value: &[u8],
    mut builder: crate::operation::start_replication::builders::StartReplicationOutputBuilder,
) -> Result<
    crate::operation::start_replication::builders::StartReplicationOutputBuilder,
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
                    "applicationID" => {
                        builder = builder.set_application_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "arn" => {
                        builder = builder.set_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "dataReplicationInfo" => {
                        builder = builder.set_data_replication_info(
                            crate::protocol_serde::shape_data_replication_info::de_data_replication_info(tokens)?
                        );
                    }
                    "fqdnForActionFramework" => {
                        builder = builder.set_fqdn_for_action_framework(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "isArchived" => {
                        builder = builder.set_is_archived(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(
                                tokens.next(),
                            )?,
                        );
                    }
                    "launchedInstance" => {
                        builder = builder.set_launched_instance(
                            crate::protocol_serde::shape_launched_instance::de_launched_instance(
                                tokens,
                            )?,
                        );
                    }
                    "lifeCycle" => {
                        builder = builder.set_life_cycle(
                            crate::protocol_serde::shape_life_cycle::de_life_cycle(tokens)?,
                        );
                    }
                    "replicationType" => {
                        builder = builder.set_replication_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::ReplicationType::from(u.as_ref()))
                            })
                            .transpose()?,
                        );
                    }
                    "sourceProperties" => {
                        builder = builder.set_source_properties(
                            crate::protocol_serde::shape_source_properties::de_source_properties(
                                tokens,
                            )?,
                        );
                    }
                    "sourceServerID" => {
                        builder = builder.set_source_server_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "tags" => {
                        builder = builder
                            .set_tags(crate::protocol_serde::shape_tags_map::de_tags_map(tokens)?);
                    }
                    "userProvidedID" => {
                        builder = builder.set_user_provided_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "vcenterClientID" => {
                        builder = builder.set_vcenter_client_id(
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
