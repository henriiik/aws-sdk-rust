// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_app_assessments::_list_app_assessments_output::ListAppAssessmentsOutputBuilder;

pub use crate::operation::list_app_assessments::_list_app_assessments_input::ListAppAssessmentsInputBuilder;

/// Fluent builder constructing a request to `ListAppAssessments`.
///
/// <p>Lists the assessments for an Resilience Hub application. You can use request parameters to refine the results for the response object.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ListAppAssessmentsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_app_assessments::builders::ListAppAssessmentsInputBuilder,
}
impl ListAppAssessmentsFluentBuilder {
    /// Creates a new `ListAppAssessments`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::list_app_assessments::ListAppAssessments,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::list_app_assessments::ListAppAssessmentsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::list_app_assessments::ListAppAssessmentsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::list_app_assessments::ListAppAssessmentsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_app_assessments::paginator::ListAppAssessmentsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::list_app_assessments::paginator::ListAppAssessmentsPaginator {
        crate::operation::list_app_assessments::paginator::ListAppAssessmentsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.app_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_app_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_app_arn(input);
        self
    }
    /// <p>The name for the assessment.</p>
    pub fn assessment_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.assessment_name(input.into());
        self
    }
    /// <p>The name for the assessment.</p>
    pub fn set_assessment_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_assessment_name(input);
        self
    }
    /// Appends an item to `assessmentStatus`.
    ///
    /// To override the contents of this collection use [`set_assessment_status`](Self::set_assessment_status).
    ///
    /// <p>The current status of the assessment for the resiliency policy.</p>
    pub fn assessment_status(mut self, input: crate::types::AssessmentStatus) -> Self {
        self.inner = self.inner.assessment_status(input);
        self
    }
    /// <p>The current status of the assessment for the resiliency policy.</p>
    pub fn set_assessment_status(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AssessmentStatus>>,
    ) -> Self {
        self.inner = self.inner.set_assessment_status(input);
        self
    }
    /// <p>The current status of compliance for the resiliency policy.</p>
    pub fn compliance_status(mut self, input: crate::types::ComplianceStatus) -> Self {
        self.inner = self.inner.compliance_status(input);
        self
    }
    /// <p>The current status of compliance for the resiliency policy.</p>
    pub fn set_compliance_status(
        mut self,
        input: std::option::Option<crate::types::ComplianceStatus>,
    ) -> Self {
        self.inner = self.inner.set_compliance_status(input);
        self
    }
    /// <p>Specifies the entity that invoked a specific assessment, either a <code>User</code> or the <code>System</code>.</p>
    pub fn invoker(mut self, input: crate::types::AssessmentInvoker) -> Self {
        self.inner = self.inner.invoker(input);
        self
    }
    /// <p>Specifies the entity that invoked a specific assessment, either a <code>User</code> or the <code>System</code>.</p>
    pub fn set_invoker(
        mut self,
        input: std::option::Option<crate::types::AssessmentInvoker>,
    ) -> Self {
        self.inner = self.inner.set_invoker(input);
        self
    }
    /// <p>The default is to sort by ascending <b>startTime</b>. To sort by descending <b>startTime</b>, set reverseOrder to <code>true</code>.</p>
    pub fn reverse_order(mut self, input: bool) -> Self {
        self.inner = self.inner.reverse_order(input);
        self
    }
    /// <p>The default is to sort by ascending <b>startTime</b>. To sort by descending <b>startTime</b>, set reverseOrder to <code>true</code>.</p>
    pub fn set_reverse_order(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_reverse_order(input);
        self
    }
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Null, or the token from a previous call to get the next set of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to include in the response. If more results exist than the specified <code>MaxResults</code> value, a token is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
}
