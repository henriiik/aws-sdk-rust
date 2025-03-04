// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_insight_rules_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_insight_rules::DescribeInsightRulesOutput,
    crate::operation::describe_insight_rules::DescribeInsightRulesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::describe_insight_rules::DescribeInsightRulesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::describe_insight_rules::DescribeInsightRulesError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidNextToken" => {
            crate::operation::describe_insight_rules::DescribeInsightRulesError::InvalidNextToken({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::InvalidNextTokenBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token::de_invalid_next_token_xml_err(response.body().as_ref(), output).map_err(crate::operation::describe_insight_rules::DescribeInsightRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::describe_insight_rules::DescribeInsightRulesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_insight_rules_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_insight_rules::DescribeInsightRulesOutput,
    crate::operation::describe_insight_rules::DescribeInsightRulesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_insight_rules::builders::DescribeInsightRulesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_insight_rules::de_describe_insight_rules(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::describe_insight_rules::DescribeInsightRulesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_insight_rules(
    inp: &[u8],
    mut builder: crate::operation::describe_insight_rules::builders::DescribeInsightRulesOutputBuilder,
) -> Result<
    crate::operation::describe_insight_rules::builders::DescribeInsightRulesOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeInsightRulesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeInsightRulesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeInsightRulesResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeInsightRulesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudwatch.synthetic#DescribeInsightRulesOutput$NextToken */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_1);
            }
            ,
            s if s.matches("InsightRules") /* InsightRules com.amazonaws.cloudwatch.synthetic#DescribeInsightRulesOutput$InsightRules */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_insight_rules::de_insight_rules(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_insight_rules(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeInsightRulesResult tag",
        ));
    };
    Ok(builder)
}
