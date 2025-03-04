// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourceSet`](crate::operation::get_resource_set::builders::GetResourceSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::get_resource_set::builders::GetResourceSetFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::get_resource_set::builders::GetResourceSetFluentBuilder::set_identifier): <p>A unique identifier for the resource set, used in a TODO to refer to the resource set.</p>
    /// - On success, responds with [`GetResourceSetOutput`](crate::operation::get_resource_set::GetResourceSetOutput) with field(s):
    ///   - [`resource_set(Option<ResourceSet>)`](crate::operation::get_resource_set::GetResourceSetOutput::resource_set): <p>Information about the specified resource set.</p>
    ///   - [`resource_set_arn(Option<String>)`](crate::operation::get_resource_set::GetResourceSetOutput::resource_set_arn): <p>The Amazon Resource Name (ARN) of the resource set.</p>
    /// - On failure, responds with [`SdkError<GetResourceSetError>`](crate::operation::get_resource_set::GetResourceSetError)
    pub fn get_resource_set(
        &self,
    ) -> crate::operation::get_resource_set::builders::GetResourceSetFluentBuilder {
        crate::operation::get_resource_set::builders::GetResourceSetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
