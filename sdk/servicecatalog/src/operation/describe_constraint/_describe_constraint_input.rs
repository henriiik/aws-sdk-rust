// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeConstraintInput {
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub accept_language: std::option::Option<std::string::String>,
    /// <p>The identifier of the constraint.</p>
    #[doc(hidden)]
    pub id: std::option::Option<std::string::String>,
}
impl DescribeConstraintInput {
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(&self) -> std::option::Option<&str> {
        self.accept_language.as_deref()
    }
    /// <p>The identifier of the constraint.</p>
    pub fn id(&self) -> std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl DescribeConstraintInput {
    /// Creates a new builder-style object to manufacture [`DescribeConstraintInput`](crate::operation::describe_constraint::DescribeConstraintInput).
    pub fn builder(
    ) -> crate::operation::describe_constraint::builders::DescribeConstraintInputBuilder {
        crate::operation::describe_constraint::builders::DescribeConstraintInputBuilder::default()
    }
}

/// A builder for [`DescribeConstraintInput`](crate::operation::describe_constraint::DescribeConstraintInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct DescribeConstraintInputBuilder {
    pub(crate) accept_language: std::option::Option<std::string::String>,
    pub(crate) id: std::option::Option<std::string::String>,
}
impl DescribeConstraintInputBuilder {
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(mut self, input: impl Into<std::string::String>) -> Self {
        self.accept_language = Some(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.accept_language = input;
        self
    }
    /// <p>The identifier of the constraint.</p>
    pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
        self.id = Some(input.into());
        self
    }
    /// <p>The identifier of the constraint.</p>
    pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeConstraintInput`](crate::operation::describe_constraint::DescribeConstraintInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_constraint::DescribeConstraintInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_constraint::DescribeConstraintInput {
                accept_language: self.accept_language,
                id: self.id,
            },
        )
    }
}
