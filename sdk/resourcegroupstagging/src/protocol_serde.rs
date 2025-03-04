// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_describe_report_creation;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_get_compliance_summary;

pub(crate) mod shape_get_resources;

pub(crate) mod shape_get_tag_keys;

pub(crate) mod shape_get_tag_values;

pub(crate) mod shape_start_report_creation;

pub(crate) mod shape_tag_resources;

pub(crate) mod shape_untag_resources;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_constraint_violation_exception;

pub(crate) mod shape_get_compliance_summary_input;

pub(crate) mod shape_get_resources_input;

pub(crate) mod shape_get_tag_keys_input;

pub(crate) mod shape_get_tag_values_input;

pub(crate) mod shape_internal_service_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_pagination_token_expired_exception;

pub(crate) mod shape_start_report_creation_input;

pub(crate) mod shape_tag_resources_input;

pub(crate) mod shape_throttled_exception;

pub(crate) mod shape_untag_resources_input;

pub(crate) mod shape_failed_resources_map;

pub(crate) mod shape_resource_tag_mapping_list;

pub(crate) mod shape_summary_list;

pub(crate) mod shape_tag_filter;

pub(crate) mod shape_tag_key_list;

pub(crate) mod shape_tag_values_output_list;

pub(crate) mod shape_failure_info;

pub(crate) mod shape_resource_tag_mapping;

pub(crate) mod shape_summary;

pub(crate) mod shape_compliance_details;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_tag;
