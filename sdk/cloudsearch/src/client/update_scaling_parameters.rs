// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateScalingParameters`](crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder::set_domain_name): <p>A string that represents the name of a domain. Domain names are unique across the domains owned by an account within an AWS region. Domain names start with a letter or number and can contain the following characters: a-z (lowercase), 0-9, and - (hyphen).</p>
    ///   - [`scaling_parameters(ScalingParameters)`](crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder::scaling_parameters) / [`set_scaling_parameters(Option<ScalingParameters>)`](crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder::set_scaling_parameters): <p>The desired instance type and desired number of replicas of each index partition.</p>
    /// - On success, responds with [`UpdateScalingParametersOutput`](crate::operation::update_scaling_parameters::UpdateScalingParametersOutput) with field(s):
    ///   - [`scaling_parameters(Option<ScalingParametersStatus>)`](crate::operation::update_scaling_parameters::UpdateScalingParametersOutput::scaling_parameters): <p>The status and configuration of a search domain's scaling parameters. </p>
    /// - On failure, responds with [`SdkError<UpdateScalingParametersError>`](crate::operation::update_scaling_parameters::UpdateScalingParametersError)
    pub fn update_scaling_parameters(
        &self,
    ) -> crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder
    {
        crate::operation::update_scaling_parameters::builders::UpdateScalingParametersFluentBuilder::new(self.handle.clone())
    }
}
