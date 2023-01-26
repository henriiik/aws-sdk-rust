// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(
    clippy::collapsible_if,
    clippy::bool_comparison,
    clippy::nonminimal_bool,
    clippy::comparison_to_empty,
    clippy::redundant_pattern_matching
)]
pub(super) fn resolve_endpoint(
    _params: &crate::endpoint::Params,
    _diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
    partition_resolver: &crate::endpoint_lib::partition::PartitionResolver,
) -> aws_smithy_http::endpoint::Result {
    #[allow(unused_variables)]
    let region = &_params.region;
    #[allow(unused_variables)]
    let use_dual_stack = &_params.use_dual_stack;
    #[allow(unused_variables)]
    let use_fips = &_params.use_fips;
    #[allow(unused_variables)]
    let endpoint = &_params.endpoint;
    #[allow(unused_variables)]
    if let Some(partition_result) =
        partition_resolver.resolve_partition(region, _diagnostic_collector)
    {
        #[allow(unused_variables)]
        if let Some(endpoint) = endpoint {
            if (*use_fips) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: FIPS and custom endpoint are not supported".to_string(),
                ));
            }
            if (*use_dual_stack) == (true) {
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                    "Invalid Configuration: Dualstack and custom endpoint are not supported"
                        .to_string(),
                ));
            }
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url(endpoint.to_owned())
                .build());
        }
        if (*use_fips) == (true) {
            if (*use_dual_stack) == (true) {
                if (true) == (partition_result.supports_fips()) {
                    if (true) == (partition_result.supports_dual_stack()) {
                        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                            .url({
                                let mut out = String::new();
                                out.push_str("https://streams.dynamodb-fips.");
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&region);
                                out.push('.');
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&partition_result.dual_stack_dns_suffix());
                                out
                            })
                            .build());
                    }
                }
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
.to_string()));
            }
        }
        if (*use_fips) == (true) {
            if (true) == (partition_result.supports_fips()) {
                if ("aws-us-gov") == (partition_result.name()) {
                    return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                        .url({
                            let mut out = String::new();
                            out.push_str("https://streams.dynamodb.");
                            #[allow(clippy::needless_borrow)]
                            out.push_str(&region);
                            out.push_str(".amazonaws.com");
                            out
                        })
                        .build());
                }
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://streams.dynamodb-fips.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "FIPS is enabled but this partition does not support FIPS".to_string(),
            ));
        }
        if (*use_dual_stack) == (true) {
            if (true) == (partition_result.supports_dual_stack()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://streams.dynamodb.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dual_stack_dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "DualStack is enabled but this partition does not support DualStack".to_string(),
            ));
        }
        if (region) == ("local") {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url("http://localhost:8000".to_string())
                .property(
                    "authSchemes",
                    vec![aws_smithy_types::Document::from({
                        let mut out =
                            std::collections::HashMap::<String, aws_smithy_types::Document>::new();
                        out.insert("name".to_string(), "sigv4".to_string().into());
                        out.insert("signingName".to_string(), "dynamodb".to_string().into());
                        out.insert("signingRegion".to_string(), "us-east-1".to_string().into());
                        out
                    })],
                )
                .build());
        }
        if ("aws") == (partition_result.name()) {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url({
                    let mut out = String::new();
                    out.push_str("https://streams.dynamodb.");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&region);
                    out.push_str(".amazonaws.com");
                    out
                })
                .build());
        }
        if ("aws-cn") == (partition_result.name()) {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url({
                    let mut out = String::new();
                    out.push_str("https://streams.dynamodb.");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&region);
                    out.push_str(".amazonaws.com.cn");
                    out
                })
                .build());
        }
        if ("aws-us-gov") == (partition_result.name()) {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url({
                    let mut out = String::new();
                    out.push_str("https://streams.dynamodb.");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&region);
                    out.push_str(".amazonaws.com");
                    out
                })
                .build());
        }
        if ("aws-iso") == (partition_result.name()) {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url({
                    let mut out = String::new();
                    out.push_str("https://streams.dynamodb.");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&region);
                    out.push_str(".c2s.ic.gov");
                    out
                })
                .build());
        }
        if ("aws-iso-b") == (partition_result.name()) {
            return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                .url({
                    let mut out = String::new();
                    out.push_str("https://streams.dynamodb.");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&region);
                    out.push_str(".sc2s.sgov.gov");
                    out
                })
                .build());
        }
        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
            .url({
                let mut out = String::new();
                out.push_str("https://streams.dynamodb.");
                #[allow(clippy::needless_borrow)]
                out.push_str(&region);
                out.push('.');
                #[allow(clippy::needless_borrow)]
                out.push_str(&partition_result.dns_suffix());
                out
            })
            .build());
    }
    #[allow(unreachable_code)]
    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
        format!(
            "No rules matched these parameters. This is a bug. {:?}",
            _params
        ),
    ));
}
