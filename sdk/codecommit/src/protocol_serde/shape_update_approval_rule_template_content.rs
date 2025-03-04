// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_approval_rule_template_content_input(
    input: &crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_approval_rule_template_content_input::ser_update_approval_rule_template_content_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_approval_rule_template_content_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentOutput, crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ApprovalRuleTemplateContentRequiredException" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateContentRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApprovalRuleTemplateContentRequiredExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_approval_rule_template_content_required_exception::de_approval_rule_template_content_required_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApprovalRuleTemplateDoesNotExistException" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApprovalRuleTemplateDoesNotExistExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_approval_rule_template_does_not_exist_exception::de_approval_rule_template_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApprovalRuleTemplateNameRequiredException" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::ApprovalRuleTemplateNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ApprovalRuleTemplateNameRequiredExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_approval_rule_template_name_required_exception::de_approval_rule_template_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidApprovalRuleTemplateContentException" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateContentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidApprovalRuleTemplateContentExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_approval_rule_template_content_exception::de_invalid_approval_rule_template_content_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidApprovalRuleTemplateNameException" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::InvalidApprovalRuleTemplateNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidApprovalRuleTemplateNameExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_approval_rule_template_name_exception::de_invalid_approval_rule_template_name_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRuleContentSha256Exception" => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::InvalidRuleContentSha256Exception({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRuleContentSha256ExceptionBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_rule_content_sha256_exception::de_invalid_rule_content_sha256_exception_json_err(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_approval_rule_template_content_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentOutput, crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_approval_rule_template_content::builders::UpdateApprovalRuleTemplateContentOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_approval_rule_template_content::de_update_approval_rule_template_content(response.body().as_ref(), output).map_err(crate::operation::update_approval_rule_template_content::UpdateApprovalRuleTemplateContentError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

pub(crate) fn de_update_approval_rule_template_content(value: &[u8], mut builder: crate::operation::update_approval_rule_template_content::builders::UpdateApprovalRuleTemplateContentOutputBuilder) -> Result<crate::operation::update_approval_rule_template_content::builders::UpdateApprovalRuleTemplateContentOutputBuilder, aws_smithy_json::deserialize::error::DeserializeError>{
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
                    "approvalRuleTemplate" => {
                        builder = builder.set_approval_rule_template(
                            crate::protocol_serde::shape_approval_rule_template::de_approval_rule_template(tokens)?
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
