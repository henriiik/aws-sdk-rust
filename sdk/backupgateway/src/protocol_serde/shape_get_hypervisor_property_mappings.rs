// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_hypervisor_property_mappings_input(
    input: &crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_hypervisor_property_mappings_input::ser_get_hypervisor_property_mappings_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_hypervisor_property_mappings_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsOutput,
    crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_hypervisor_property_mappings_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsOutput,
    crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_hypervisor_property_mappings::builders::GetHypervisorPropertyMappingsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_hypervisor_property_mappings::de_get_hypervisor_property_mappings(response.body().as_ref(), output).map_err(crate::operation::get_hypervisor_property_mappings::GetHypervisorPropertyMappingsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_get_hypervisor_property_mappings(value: &[u8], mut builder: crate::operation::get_hypervisor_property_mappings::builders::GetHypervisorPropertyMappingsOutputBuilder) -> Result<crate::operation::get_hypervisor_property_mappings::builders::GetHypervisorPropertyMappingsOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "HypervisorArn" => {
                        builder = builder.set_hypervisor_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(
                                tokens.next(),
                            )?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                        );
                    }
                    "VmwareToAwsTagMappings" => {
                        builder = builder.set_vmware_to_aws_tag_mappings(
                            crate::protocol_serde::shape_vmware_to_aws_tag_mappings::de_vmware_to_aws_tag_mappings(tokens)?
                        );
                    }
                    "IamRoleArn" => {
                        builder = builder.set_iam_role_arn(
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
