// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateExperiment`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`experiment_name(impl Into<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::experiment_name) / [`set_experiment_name(Option<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::set_experiment_name): <p>The name of the experiment. The name must be unique in your Amazon Web Services account and is not case-sensitive.</p>
    ///   - [`display_name(impl Into<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::set_display_name): <p>The name of the experiment as displayed. The name doesn't need to be unique. If you don't specify <code>DisplayName</code>, the value in <code>ExperimentName</code> is displayed.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::set_description): <p>The description of the experiment.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::set_tags): <p>A list of tags to associate with the experiment. You can use <code>Search</code> API to search on the tags.</p>
    /// - On success, responds with [`CreateExperimentOutput`](crate::operation::create_experiment::CreateExperimentOutput) with field(s):
    ///   - [`experiment_arn(Option<String>)`](crate::operation::create_experiment::CreateExperimentOutput::experiment_arn): <p>The Amazon Resource Name (ARN) of the experiment.</p>
    /// - On failure, responds with [`SdkError<CreateExperimentError>`](crate::operation::create_experiment::CreateExperimentError)
    pub fn create_experiment(
        &self,
    ) -> crate::operation::create_experiment::builders::CreateExperimentFluentBuilder {
        crate::operation::create_experiment::builders::CreateExperimentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
