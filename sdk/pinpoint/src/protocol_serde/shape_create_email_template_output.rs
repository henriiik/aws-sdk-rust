// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_create_template_message_body_payload(
    body: &[u8],
) -> std::result::Result<
    std::option::Option<crate::types::CreateTemplateMessageBody>,
    crate::operation::create_email_template::CreateEmailTemplateError,
> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_create_template_message_body::de_create_template_message_body_payload(body).map_err(crate::operation::create_email_template::CreateEmailTemplateError::unhandled)
    }).transpose()
}
