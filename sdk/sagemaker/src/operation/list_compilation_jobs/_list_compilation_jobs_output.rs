// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListCompilationJobsOutput {
    /// <p>An array of <code>CompilationJobSummary</code> objects, each describing a model compilation job. </p>
    #[doc(hidden)]
    pub compilation_job_summaries:
        std::option::Option<std::vec::Vec<crate::types::CompilationJobSummary>>,
    /// <p>If the response is truncated, Amazon SageMaker returns this <code>NextToken</code>. To retrieve the next set of model compilation jobs, use this token in the next request.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListCompilationJobsOutput {
    /// <p>An array of <code>CompilationJobSummary</code> objects, each describing a model compilation job. </p>
    pub fn compilation_job_summaries(
        &self,
    ) -> std::option::Option<&[crate::types::CompilationJobSummary]> {
        self.compilation_job_summaries.as_deref()
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this <code>NextToken</code>. To retrieve the next set of model compilation jobs, use this token in the next request.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListCompilationJobsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListCompilationJobsOutput {
    /// Creates a new builder-style object to manufacture [`ListCompilationJobsOutput`](crate::operation::list_compilation_jobs::ListCompilationJobsOutput).
    pub fn builder(
    ) -> crate::operation::list_compilation_jobs::builders::ListCompilationJobsOutputBuilder {
        crate::operation::list_compilation_jobs::builders::ListCompilationJobsOutputBuilder::default(
        )
    }
}

/// A builder for [`ListCompilationJobsOutput`](crate::operation::list_compilation_jobs::ListCompilationJobsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListCompilationJobsOutputBuilder {
    pub(crate) compilation_job_summaries:
        std::option::Option<std::vec::Vec<crate::types::CompilationJobSummary>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListCompilationJobsOutputBuilder {
    /// Appends an item to `compilation_job_summaries`.
    ///
    /// To override the contents of this collection use [`set_compilation_job_summaries`](Self::set_compilation_job_summaries).
    ///
    /// <p>An array of <code>CompilationJobSummary</code> objects, each describing a model compilation job. </p>
    pub fn compilation_job_summaries(mut self, input: crate::types::CompilationJobSummary) -> Self {
        let mut v = self.compilation_job_summaries.unwrap_or_default();
        v.push(input);
        self.compilation_job_summaries = Some(v);
        self
    }
    /// <p>An array of <code>CompilationJobSummary</code> objects, each describing a model compilation job. </p>
    pub fn set_compilation_job_summaries(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CompilationJobSummary>>,
    ) -> Self {
        self.compilation_job_summaries = input;
        self
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this <code>NextToken</code>. To retrieve the next set of model compilation jobs, use this token in the next request.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If the response is truncated, Amazon SageMaker returns this <code>NextToken</code>. To retrieve the next set of model compilation jobs, use this token in the next request.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListCompilationJobsOutput`](crate::operation::list_compilation_jobs::ListCompilationJobsOutput).
    pub fn build(self) -> crate::operation::list_compilation_jobs::ListCompilationJobsOutput {
        crate::operation::list_compilation_jobs::ListCompilationJobsOutput {
            compilation_job_summaries: self.compilation_job_summaries,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
