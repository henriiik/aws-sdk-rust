// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_folder_resolved_permissions_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsOutput,
    crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedUserEditionException" => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::UnsupportedUserEditionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedUserEditionExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_user_edition_exception::de_unsupported_user_edition_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_folder_resolved_permissions_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsOutput,
    crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_folder_resolved_permissions::builders::DescribeFolderResolvedPermissionsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_folder_resolved_permissions::de_describe_folder_resolved_permissions(response.body().as_ref(), output).map_err(crate::operation::describe_folder_resolved_permissions::DescribeFolderResolvedPermissionsError::unhandled)?;
        output = output.set_status(Some(response.status().as_u16() as _));
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_describe_folder_resolved_permissions(value: &[u8], mut builder: crate::operation::describe_folder_resolved_permissions::builders::DescribeFolderResolvedPermissionsOutputBuilder) -> Result<crate::operation::describe_folder_resolved_permissions::builders::DescribeFolderResolvedPermissionsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "Arn" => {
                        builder = builder.set_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "FolderId" => {
                        builder = builder.set_folder_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "Permissions" => {
                        builder = builder.set_permissions(
                            crate::protocol_serde::shape_resource_permission_list::de_resource_permission_list(tokens)?
                        );
                    }
                    "RequestId" => {
                        builder = builder.set_request_id(
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
