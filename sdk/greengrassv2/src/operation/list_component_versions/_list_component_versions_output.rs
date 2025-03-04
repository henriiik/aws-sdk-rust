// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListComponentVersionsOutput {
    /// <p>A list of versions that exist for the component.</p>
    #[doc(hidden)]
    pub component_versions:
        std::option::Option<std::vec::Vec<crate::types::ComponentVersionListItem>>,
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListComponentVersionsOutput {
    /// <p>A list of versions that exist for the component.</p>
    pub fn component_versions(
        &self,
    ) -> std::option::Option<&[crate::types::ComponentVersionListItem]> {
        self.component_versions.as_deref()
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListComponentVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListComponentVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListComponentVersionsOutput`](crate::operation::list_component_versions::ListComponentVersionsOutput).
    pub fn builder(
    ) -> crate::operation::list_component_versions::builders::ListComponentVersionsOutputBuilder
    {
        crate::operation::list_component_versions::builders::ListComponentVersionsOutputBuilder::default()
    }
}

/// A builder for [`ListComponentVersionsOutput`](crate::operation::list_component_versions::ListComponentVersionsOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListComponentVersionsOutputBuilder {
    pub(crate) component_versions:
        std::option::Option<std::vec::Vec<crate::types::ComponentVersionListItem>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListComponentVersionsOutputBuilder {
    /// Appends an item to `component_versions`.
    ///
    /// To override the contents of this collection use [`set_component_versions`](Self::set_component_versions).
    ///
    /// <p>A list of versions that exist for the component.</p>
    pub fn component_versions(mut self, input: crate::types::ComponentVersionListItem) -> Self {
        let mut v = self.component_versions.unwrap_or_default();
        v.push(input);
        self.component_versions = Some(v);
        self
    }
    /// <p>A list of versions that exist for the component.</p>
    pub fn set_component_versions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ComponentVersionListItem>>,
    ) -> Self {
        self.component_versions = input;
        self
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token for the next set of results, or null if there are no additional results.</p>
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
    /// Consumes the builder and constructs a [`ListComponentVersionsOutput`](crate::operation::list_component_versions::ListComponentVersionsOutput).
    pub fn build(self) -> crate::operation::list_component_versions::ListComponentVersionsOutput {
        crate::operation::list_component_versions::ListComponentVersionsOutput {
            component_versions: self.component_versions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
