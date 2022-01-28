# Contributing

1. [File an issue](https://github.com/plabayo/news/issues/new/choose).
   The issue will be used to discuss the bug or feature and should be created before opening an MR.
   > Best to even wait on actually developing it as to make sure
   > that we're all aligned on what you're trying to contribute,
   > as to avoid having to reject your hard work and code.

In case you also want to help resolve it by contributing to the code base you would continue as follows:

2. Install Rust and configure correctly (https://www.rust-lang.org/tools/install).
3. Clone the repo: `git clone https://github.com/plabayo/news`
4. Change into the checked out source: `cd news`
5. Fork the repo.
6. Set your fork as a remote: `git remote add fork git@github.com:GITHUB_USERNAME/news.git`
7. Make changes, commit to your fork.
   Please add a short summary and a detailed commit message for each commit.
   > Feel free to make as many commits as you want in your branch,
   > prior to making your MR ready for review, please clean up the git history
   > from your branch by rebasing and squashing relevant commits together,
   > with the final commits being minimal in number and detailed in their description.
8. To minimize friction, consider setting Allow edits from maintainers on the PR,
   which will enable project committers and automation to update your PR.
9. A maintainer will review the pull request and make comments.

   Prefer adding additional commits over amending and force-pushing
   since it can be difficult to follow code reviews when the commit history changes.
   
   Commits will be squashed when they're merged.

## Testing

All tests can be run locally against the latest Rust version (or whichever supported Rust version you're using on your development machine):

```bash
cargo test -v
```

These tests will also be run in our CI system (using GitHub Actions)
on both the minimum supported Go version as well as the latest major Go version.

 To see which versions these are checkout our [README](README.md#rust-versions-supported).

## Contributor Code of Conduct

As contributors and maintainers of this project, and in the interest of fostering an open and welcoming community, we pledge to respect all people who contribute through reporting issues, posting feature requests, updating documentation, submitting pull requests or patches, and other activities.

We are committed to making participation in this project a harassment-free experience for everyone, regardless of level of experience, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

This code of conduct applies both within project spaces and in public spaces when an individual is representing the project or its community.

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by opening an issue or contacting one or more of the project maintainers.

This Code of Conduct is adapted from the Contributor Covenant, version 1.2.0, available at https://contributor-covenant.org/version/1/2/0/
