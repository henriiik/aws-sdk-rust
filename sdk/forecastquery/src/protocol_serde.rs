// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) mod shape_query_forecast;

pub fn parse_http_error_metadata(
    response: &http::Response<bytes::Bytes>,
) -> Result<
    aws_smithy_types::error::metadata::Builder,
    aws_smithy_json::deserialize::error::DeserializeError,
> {
    crate::json_errors::parse_error_metadata(response.body(), response.headers())
}

pub(crate) mod shape_query_what_if_forecast;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_invalid_input_exception;

pub(crate) mod shape_invalid_next_token_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_query_forecast_input;

pub(crate) mod shape_query_what_if_forecast_input;

pub(crate) mod shape_resource_in_use_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_forecast;

pub(crate) mod shape_predictions;

pub(crate) mod shape_time_series;

pub(crate) mod shape_data_point;
