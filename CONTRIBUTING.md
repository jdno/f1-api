# Contributing to F1 API

Hello! 👋

We're happy that you're here, and appreciate your interest in making F1 API a
better tool. This document is supposed to give you a helping hand in getting
started.

- [How to report a bug](#how-to-report-a-bug)
- [How to request a feature](#how-to-request-a-feature)
- [How to submit changes](#how-to-submit-changes)
- [How to release a new version](#how-to-release-a-new-version)

## How to report a bug

In the unfortunate (but not unlikely) event that you found a bug in F1 API, it
is best to file a _bug report_ in our [issue tracker][issues]. Please answer all
questions in the form, and be as detailed as possible. This helps us investigate
and fix the issue.

## How to request a feature

If there is something missing in F1 API that you think should be there, we'd
love to here from you. Before investing any time writing us, though, have a look
at the issues in our [issue tracker][issues]. Someone else might have already
requested the same or a similar feature, and in that case it's best to join this
discussion instead of starting a new one.

If you don't find anything, make sure to also check the [closed issues][issues-closed].
The feature may have already been implemented, but not yet released.

Please be aware that while we appreciate your feature requests, we might not be
able to implement it in F1 API. It is our responsibility to make sure F1 API is
well maintained, and as a small team this is only possible if the library stays
small.

## How to submit changes

When working on a bug or a feature, make sure to follow the style guidelines,
write good documentation, and test your code thoroughly. The style guidelines
are automatically checked and enforced whenever you commit code. And as a rule
of thumb, every function should have a unit test.

Also make sure to write proper Git commit messages. We're big fans the blog post
[How to write a Git commit message](https://chris.beams.io/posts/git-commit/),
and the rules it introduces.

When your code is ready, open a [pull request][pr]. You can also open a _draft
pull request_ before, and ask for early feedback. We will review the pull
request, and suggest or require changes. Since we are responsible for the code
once it is merged, we take this very seriously. We want to be able to support it
in the best way possible, and that requires us to fully understand it.

## How to release a new version

Only maintainers of Nord SDK can cut a new release. The process is largely
automated to avoid errors and mistakes. These are the steps necessary to create
a new release and publish it to [crates.io]:

1. Create a new branch for the release.

   ```shell script
   git checkout master
   git checkout -b cut-release
   ```

1. Run [cargo-release](https://github.com/sunng87/cargo-release) to prepare the
   release. This sets the version, updates the changelog, and tags the commit.

   ```shell script
   cargo release [patch|minor|major]
   ```

1. Push the branch to GitHub, create a pull request, and wait for the checks to
   pass.

1. Go to the [releases](https://github.com/nordsdk/f1-api/releases) and create
   a release for the new version. Copy the relevant section from the [changelog]
   into the release message, and name the release after the version. When saving
   the release, a GitHub action is started that publishes the version to
   [crates.io].

[changelog]: ./CHANGELOG.md
[crates.io]: https://crates.io
[issues]: https://github.com/jdno/f1-api/issues
[issues-closed]: https://github.com/jdno/f1-api/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aclosed
[pr]: https://github.com/jdno/f1-api/pulls
