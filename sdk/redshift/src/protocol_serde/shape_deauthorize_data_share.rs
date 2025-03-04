// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deauthorize_data_share_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deauthorize_data_share::DeauthorizeDataShareOutput,
    crate::operation::deauthorize_data_share::DeauthorizeDataShareError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::deauthorize_data_share::DeauthorizeDataShareError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::deauthorize_data_share::DeauthorizeDataShareError::unhandled(
                    generic,
                ),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidDataShareFault" => crate::operation::deauthorize_data_share::DeauthorizeDataShareError::InvalidDataShareFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidDataShareFaultBuilder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_data_share_fault::de_invalid_data_share_fault_xml_err(response.body().as_ref(), output).map_err(crate::operation::deauthorize_data_share::DeauthorizeDataShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::deauthorize_data_share::DeauthorizeDataShareError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deauthorize_data_share_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deauthorize_data_share::DeauthorizeDataShareOutput,
    crate::operation::deauthorize_data_share::DeauthorizeDataShareError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deauthorize_data_share::builders::DeauthorizeDataShareOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_deauthorize_data_share::de_deauthorize_data_share(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::deauthorize_data_share::DeauthorizeDataShareError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_deauthorize_data_share(
    inp: &[u8],
    mut builder: crate::operation::deauthorize_data_share::builders::DeauthorizeDataShareOutputBuilder,
) -> Result<
    crate::operation::deauthorize_data_share::builders::DeauthorizeDataShareOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeauthorizeDataShareResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeauthorizeDataShareResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DeauthorizeDataShareResult")) {
            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DeauthorizeDataShareResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DataShareArn") /* DataShareArn com.amazonaws.redshift.synthetic#DeauthorizeDataShareOutput$DataShareArn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_data_share_arn(var_1);
            }
            ,
            s if s.matches("ProducerArn") /* ProducerArn com.amazonaws.redshift.synthetic#DeauthorizeDataShareOutput$ProducerArn */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_producer_arn(var_2);
            }
            ,
            s if s.matches("AllowPubliclyAccessibleConsumers") /* AllowPubliclyAccessibleConsumers com.amazonaws.redshift.synthetic#DeauthorizeDataShareOutput$AllowPubliclyAccessibleConsumers */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.redshift#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_allow_publicly_accessible_consumers(var_3);
            }
            ,
            s if s.matches("DataShareAssociations") /* DataShareAssociations com.amazonaws.redshift.synthetic#DeauthorizeDataShareOutput$DataShareAssociations */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_data_share_association_list::de_data_share_association_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_data_share_associations(var_4);
            }
            ,
            s if s.matches("ManagedBy") /* ManagedBy com.amazonaws.redshift.synthetic#DeauthorizeDataShareOutput$ManagedBy */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_managed_by(var_5);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DeauthorizeDataShareResult tag",
        ));
    };
    Ok(builder)
}
