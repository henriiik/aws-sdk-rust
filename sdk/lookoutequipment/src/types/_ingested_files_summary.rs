// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Gives statistics about how many files have been ingested, and which files have not been ingested, for a particular ingestion job.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct IngestedFilesSummary {
    /// <p>Indicates the total number of files that were submitted for ingestion.</p>
    #[doc(hidden)]
    pub total_number_of_files: std::option::Option<i32>,
    /// <p>Indicates the number of files that were successfully ingested.</p>
    #[doc(hidden)]
    pub ingested_number_of_files: std::option::Option<i32>,
    /// <p>Indicates the number of files that were discarded. A file could be discarded because its format is invalid (for example, a jpg or pdf) or not readable.</p>
    #[doc(hidden)]
    pub discarded_files: std::option::Option<std::vec::Vec<crate::types::S3Object>>,
}
impl IngestedFilesSummary {
    /// <p>Indicates the total number of files that were submitted for ingestion.</p>
    pub fn total_number_of_files(&self) -> std::option::Option<i32> {
        self.total_number_of_files
    }
    /// <p>Indicates the number of files that were successfully ingested.</p>
    pub fn ingested_number_of_files(&self) -> std::option::Option<i32> {
        self.ingested_number_of_files
    }
    /// <p>Indicates the number of files that were discarded. A file could be discarded because its format is invalid (for example, a jpg or pdf) or not readable.</p>
    pub fn discarded_files(&self) -> std::option::Option<&[crate::types::S3Object]> {
        self.discarded_files.as_deref()
    }
}
impl IngestedFilesSummary {
    /// Creates a new builder-style object to manufacture [`IngestedFilesSummary`](crate::types::IngestedFilesSummary).
    pub fn builder() -> crate::types::builders::IngestedFilesSummaryBuilder {
        crate::types::builders::IngestedFilesSummaryBuilder::default()
    }
}

/// A builder for [`IngestedFilesSummary`](crate::types::IngestedFilesSummary).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct IngestedFilesSummaryBuilder {
    pub(crate) total_number_of_files: std::option::Option<i32>,
    pub(crate) ingested_number_of_files: std::option::Option<i32>,
    pub(crate) discarded_files: std::option::Option<std::vec::Vec<crate::types::S3Object>>,
}
impl IngestedFilesSummaryBuilder {
    /// <p>Indicates the total number of files that were submitted for ingestion.</p>
    pub fn total_number_of_files(mut self, input: i32) -> Self {
        self.total_number_of_files = Some(input);
        self
    }
    /// <p>Indicates the total number of files that were submitted for ingestion.</p>
    pub fn set_total_number_of_files(mut self, input: std::option::Option<i32>) -> Self {
        self.total_number_of_files = input;
        self
    }
    /// <p>Indicates the number of files that were successfully ingested.</p>
    pub fn ingested_number_of_files(mut self, input: i32) -> Self {
        self.ingested_number_of_files = Some(input);
        self
    }
    /// <p>Indicates the number of files that were successfully ingested.</p>
    pub fn set_ingested_number_of_files(mut self, input: std::option::Option<i32>) -> Self {
        self.ingested_number_of_files = input;
        self
    }
    /// Appends an item to `discarded_files`.
    ///
    /// To override the contents of this collection use [`set_discarded_files`](Self::set_discarded_files).
    ///
    /// <p>Indicates the number of files that were discarded. A file could be discarded because its format is invalid (for example, a jpg or pdf) or not readable.</p>
    pub fn discarded_files(mut self, input: crate::types::S3Object) -> Self {
        let mut v = self.discarded_files.unwrap_or_default();
        v.push(input);
        self.discarded_files = Some(v);
        self
    }
    /// <p>Indicates the number of files that were discarded. A file could be discarded because its format is invalid (for example, a jpg or pdf) or not readable.</p>
    pub fn set_discarded_files(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::S3Object>>,
    ) -> Self {
        self.discarded_files = input;
        self
    }
    /// Consumes the builder and constructs a [`IngestedFilesSummary`](crate::types::IngestedFilesSummary).
    pub fn build(self) -> crate::types::IngestedFilesSummary {
        crate::types::IngestedFilesSummary {
            total_number_of_files: self.total_number_of_files,
            ingested_number_of_files: self.ingested_number_of_files,
            discarded_files: self.discarded_files,
        }
    }
}
