# aws-sdk-codecommit

**Please Note: The SDK is currently in Developer Preview and is intended strictly for
feedback purposes only. Do not use this SDK for production workloads.**

This is the _AWS CodeCommit API Reference_. This reference provides descriptions of the operations and data types for AWS CodeCommit API along with usage examples.

You can use the AWS CodeCommit API to work with the following objects:

Repositories, by calling the following:
  - BatchGetRepositories, which returns information about one or more repositories associated with your AWS account.
  - CreateRepository, which creates an AWS CodeCommit repository.
  - DeleteRepository, which deletes an AWS CodeCommit repository.
  - GetRepository, which returns information about a specified repository.
  - ListRepositories, which lists all AWS CodeCommit repositories associated with your AWS account.
  - UpdateRepositoryDescription, which sets or updates the description of the repository.
  - UpdateRepositoryName, which changes the name of the repository. If you change the name of a repository, no other users of that repository can access it until you send them the new HTTPS or SSH URL to use.

Branches, by calling the following:
  - CreateBranch, which creates a branch in a specified repository.
  - DeleteBranch, which deletes the specified branch in a repository unless it is the default branch.
  - GetBranch, which returns information about a specified branch.
  - ListBranches, which lists all branches for a specified repository.
  - UpdateDefaultBranch, which changes the default branch for a repository.

Files, by calling the following:
  - DeleteFile, which deletes the content of a specified file from a specified branch.
  - GetBlob, which returns the base-64 encoded content of an individual Git blob object in a repository.
  - GetFile, which returns the base-64 encoded content of a specified file.
  - GetFolder, which returns the contents of a specified folder or directory.
  - PutFile, which adds or modifies a single file in a specified repository and branch.

Commits, by calling the following:
  - BatchGetCommits, which returns information about one or more commits in a repository.
  - CreateCommit, which creates a commit for changes to a repository.
  - GetCommit, which returns information about a commit, including commit messages and author and committer information.
  - GetDifferences, which returns information about the differences in a valid commit specifier (such as a branch, tag, HEAD, commit ID, or other fully qualified reference).

Merges, by calling the following:
  - BatchDescribeMergeConflicts, which returns information about conflicts in a merge between commits in a repository.
  - CreateUnreferencedMergeCommit, which creates an unreferenced commit between two branches or commits for the purpose of comparing them and identifying any potential conflicts.
  - DescribeMergeConflicts, which returns information about merge conflicts between the base, source, and destination versions of a file in a potential merge.
  - GetMergeCommit, which returns information about the merge between a source and destination commit.
  - GetMergeConflicts, which returns information about merge conflicts between the source and destination branch in a pull request.
  - GetMergeOptions, which returns information about the available merge options between two branches or commit specifiers.
  - MergeBranchesByFastForward, which merges two branches using the fast-forward merge option.
  - MergeBranchesBySquash, which merges two branches using the squash merge option.
  - MergeBranchesByThreeWay, which merges two branches using the three-way merge option.

Pull requests, by calling the following:
  - CreatePullRequest, which creates a pull request in a specified repository.
  - CreatePullRequestApprovalRule, which creates an approval rule for a specified pull request.
  - DeletePullRequestApprovalRule, which deletes an approval rule for a specified pull request.
  - DescribePullRequestEvents, which returns information about one or more pull request events.
  - EvaluatePullRequestApprovalRules, which evaluates whether a pull request has met all the conditions specified in its associated approval rules.
  - GetCommentsForPullRequest, which returns information about comments on a specified pull request.
  - GetPullRequest, which returns information about a specified pull request.
  - GetPullRequestApprovalStates, which returns information about the approval states for a specified pull request.
  - GetPullRequestOverrideState, which returns information about whether approval rules have been set aside (overriden) for a pull request, and if so, the Amazon Resource Name (ARN) of the user or identity that overrode the rules and their requirements for the pull request.
  - ListPullRequests, which lists all pull requests for a repository.
  - MergePullRequestByFastForward, which merges the source destination branch of a pull request into the specified destination branch for that pull request using the fast-forward merge option.
  - MergePullRequestBySquash, which merges the source destination branch of a pull request into the specified destination branch for that pull request using the squash merge option.
  - MergePullRequestByThreeWay. which merges the source destination branch of a pull request into the specified destination branch for that pull request using the three-way merge option.
  - OverridePullRequestApprovalRules, which sets aside all approval rule requirements for a pull request.
  - PostCommentForPullRequest, which posts a comment to a pull request at the specified line, file, or request.
  - UpdatePullRequestApprovalRuleContent, which updates the structure of an approval rule for a pull request.
  - UpdatePullRequestApprovalState, which updates the state of an approval on a pull request.
  - UpdatePullRequestDescription, which updates the description of a pull request.
  - UpdatePullRequestStatus, which updates the status of a pull request.
  - UpdatePullRequestTitle, which updates the title of a pull request.

Approval rule templates, by calling the following:
  - AssociateApprovalRuleTemplateWithRepository, which associates a template with a specified repository. After the template is associated with a repository, AWS CodeCommit creates approval rules that match the template conditions on every pull request created in the specified repository.
  - BatchAssociateApprovalRuleTemplateWithRepositories, which associates a template with one or more specified repositories. After the template is associated with a repository, AWS CodeCommit creates approval rules that match the template conditions on every pull request created in the specified repositories.
  - BatchDisassociateApprovalRuleTemplateFromRepositories, which removes the association between a template and specified repositories so that approval rules based on the template are not automatically created when pull requests are created in those repositories.
  - CreateApprovalRuleTemplate, which creates a template for approval rules that can then be associated with one or more repositories in your AWS account.
  - DeleteApprovalRuleTemplate, which deletes the specified template. It does not remove approval rules on pull requests already created with the template.
  - DisassociateApprovalRuleTemplateFromRepository, which removes the association between a template and a repository so that approval rules based on the template are not automatically created when pull requests are created in the specified repository.
  - GetApprovalRuleTemplate, which returns information about an approval rule template.
  - ListApprovalRuleTemplates, which lists all approval rule templates in the AWS Region in your AWS account.
  - ListAssociatedApprovalRuleTemplatesForRepository, which lists all approval rule templates that are associated with a specified repository.
  - ListRepositoriesForApprovalRuleTemplate, which lists all repositories associated with the specified approval rule template.
  - UpdateApprovalRuleTemplateDescription, which updates the description of an approval rule template.
  - UpdateApprovalRuleTemplateName, which updates the name of an approval rule template.
  - UpdateApprovalRuleTemplateContent, which updates the content of an approval rule template.

Comments in a repository, by calling the following:
  - DeleteCommentContent, which deletes the content of a comment on a commit in a repository.
  - GetComment, which returns information about a comment on a commit.
  - GetCommentReactions, which returns information about emoji reactions to comments.
  - GetCommentsForComparedCommit, which returns information about comments on the comparison between two commit specifiers in a repository.
  - PostCommentForComparedCommit, which creates a comment on the comparison between two commit specifiers in a repository.
  - PostCommentReply, which creates a reply to a comment.
  - PutCommentReaction, which creates or updates an emoji reaction to a comment.
  - UpdateComment, which updates the content of a comment on a commit in a repository.

Tags used to tag resources in AWS CodeCommit (not Git tags), by calling the following:
  - ListTagsForResource, which gets information about AWS tags for a specified Amazon Resource Name (ARN) in AWS CodeCommit.
  - TagResource, which adds or updates tags for a resource in AWS CodeCommit.
  - UntagResource, which removes tags for a resource in AWS CodeCommit.

Triggers, by calling the following:
  - GetRepositoryTriggers, which returns information about triggers configured for a repository.
  - PutRepositoryTriggers, which replaces all triggers for a repository and can be used to create or delete triggers.
  - TestRepositoryTriggers, which tests the functionality of a repository trigger by sending data to the trigger target.

For information about how to use AWS CodeCommit, see the [AWS CodeCommit User Guide](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html).

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-codecommit` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = "0.55.1"
aws-sdk-codecommit = "0.26.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust,no_run
use aws_sdk_codecommit as codecommit;

#[tokio::main]
async fn main() -> Result<(), codecommit::Error> {
    let config = aws_config::load_from_env().await;
    let client = codecommit::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-codecommit/latest/aws_sdk_codecommit/client/struct.Client.html)
for information on what calls can be made, and the inputs and outputs for each of those calls.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.

