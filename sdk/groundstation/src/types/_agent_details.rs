// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Detailed information about the agent.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct AgentDetails {
    /// <p>Current agent version.</p>
    #[doc(hidden)]
    pub agent_version: std::option::Option<std::string::String>,
    /// <p>ID of EC2 instance agent is running on.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>Type of EC2 instance agent is running on.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
    /// <p>Number of Cpu cores reserved for agent.</p>
    #[doc(hidden)]
    pub reserved_cpu_cores: std::option::Option<std::vec::Vec<i32>>,
    /// <p>List of versions being used by agent components.</p>
    #[doc(hidden)]
    pub component_versions: std::option::Option<std::vec::Vec<crate::types::ComponentVersion>>,
}
impl AgentDetails {
    /// <p>Current agent version.</p>
    pub fn agent_version(&self) -> std::option::Option<&str> {
        self.agent_version.as_deref()
    }
    /// <p>ID of EC2 instance agent is running on.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>Type of EC2 instance agent is running on.</p>
    pub fn instance_type(&self) -> std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>Number of Cpu cores reserved for agent.</p>
    pub fn reserved_cpu_cores(&self) -> std::option::Option<&[i32]> {
        self.reserved_cpu_cores.as_deref()
    }
    /// <p>List of versions being used by agent components.</p>
    pub fn component_versions(&self) -> std::option::Option<&[crate::types::ComponentVersion]> {
        self.component_versions.as_deref()
    }
}
impl AgentDetails {
    /// Creates a new builder-style object to manufacture [`AgentDetails`](crate::types::AgentDetails).
    pub fn builder() -> crate::types::builders::AgentDetailsBuilder {
        crate::types::builders::AgentDetailsBuilder::default()
    }
}

/// A builder for [`AgentDetails`](crate::types::AgentDetails).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct AgentDetailsBuilder {
    pub(crate) agent_version: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) instance_type: std::option::Option<std::string::String>,
    pub(crate) reserved_cpu_cores: std::option::Option<std::vec::Vec<i32>>,
    pub(crate) component_versions:
        std::option::Option<std::vec::Vec<crate::types::ComponentVersion>>,
}
impl AgentDetailsBuilder {
    /// <p>Current agent version.</p>
    pub fn agent_version(mut self, input: impl Into<std::string::String>) -> Self {
        self.agent_version = Some(input.into());
        self
    }
    /// <p>Current agent version.</p>
    pub fn set_agent_version(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.agent_version = input;
        self
    }
    /// <p>ID of EC2 instance agent is running on.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>ID of EC2 instance agent is running on.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>Type of EC2 instance agent is running on.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_type = Some(input.into());
        self
    }
    /// <p>Type of EC2 instance agent is running on.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// Appends an item to `reserved_cpu_cores`.
    ///
    /// To override the contents of this collection use [`set_reserved_cpu_cores`](Self::set_reserved_cpu_cores).
    ///
    /// <p>Number of Cpu cores reserved for agent.</p>
    pub fn reserved_cpu_cores(mut self, input: i32) -> Self {
        let mut v = self.reserved_cpu_cores.unwrap_or_default();
        v.push(input);
        self.reserved_cpu_cores = Some(v);
        self
    }
    /// <p>Number of Cpu cores reserved for agent.</p>
    pub fn set_reserved_cpu_cores(
        mut self,
        input: std::option::Option<std::vec::Vec<i32>>,
    ) -> Self {
        self.reserved_cpu_cores = input;
        self
    }
    /// Appends an item to `component_versions`.
    ///
    /// To override the contents of this collection use [`set_component_versions`](Self::set_component_versions).
    ///
    /// <p>List of versions being used by agent components.</p>
    pub fn component_versions(mut self, input: crate::types::ComponentVersion) -> Self {
        let mut v = self.component_versions.unwrap_or_default();
        v.push(input);
        self.component_versions = Some(v);
        self
    }
    /// <p>List of versions being used by agent components.</p>
    pub fn set_component_versions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ComponentVersion>>,
    ) -> Self {
        self.component_versions = input;
        self
    }
    /// Consumes the builder and constructs a [`AgentDetails`](crate::types::AgentDetails).
    pub fn build(self) -> crate::types::AgentDetails {
        crate::types::AgentDetails {
            agent_version: self.agent_version,
            instance_id: self.instance_id,
            instance_type: self.instance_type,
            reserved_cpu_cores: self.reserved_cpu_cores,
            component_versions: self.component_versions,
        }
    }
}
