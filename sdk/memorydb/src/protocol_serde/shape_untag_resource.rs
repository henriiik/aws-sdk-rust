// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_untag_resource_input(
    input: &crate::operation::untag_resource::UntagResourceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_untag_resource_input::ser_untag_resource_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_resource_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_resource::UntagResourceOutput,
    crate::operation::untag_resource::UntagResourceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::untag_resource::UntagResourceError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ACLNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::AclNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::AclNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_acl_not_found_fault::de_acl_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClusterNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::ClusterNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ClusterNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_not_found_fault::de_cluster_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidARNFault" => {
            crate::operation::untag_resource::UntagResourceError::InvalidArnFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidArnFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_fault::de_invalid_arn_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidClusterStateFault" => {
            crate::operation::untag_resource::UntagResourceError::InvalidClusterStateFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidClusterStateFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cluster_state_fault::de_invalid_cluster_state_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ParameterGroupNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::ParameterGroupNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::ParameterGroupNotFoundFaultBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_parameter_group_not_found_fault::de_parameter_group_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceLinkedRoleNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::ServiceLinkedRoleNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceLinkedRoleNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_linked_role_not_found_fault::de_service_linked_role_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SnapshotNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::SnapshotNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::SnapshotNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snapshot_not_found_fault::de_snapshot_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SubnetGroupNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::SubnetGroupNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::SubnetGroupNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_subnet_group_not_found_fault::de_subnet_group_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TagNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::TagNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::TagNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_not_found_fault::de_tag_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UserNotFoundFault" => {
            crate::operation::untag_resource::UntagResourceError::UserNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::UserNotFoundFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_not_found_fault::de_user_not_found_fault_json_err(response.body().as_ref(), output).map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::untag_resource::UntagResourceError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_untag_resource_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::untag_resource::UntagResourceOutput,
    crate::operation::untag_resource::UntagResourceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::untag_resource::builders::UntagResourceOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_untag_resource::de_untag_resource(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::untag_resource::UntagResourceError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_untag_resource(
    value: &[u8],
    mut builder: crate::operation::untag_resource::builders::UntagResourceOutputBuilder,
) -> Result<
    crate::operation::untag_resource::builders::UntagResourceOutputBuilder,
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
                    "TagList" => {
                        builder = builder.set_tag_list(
                            crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?,
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
