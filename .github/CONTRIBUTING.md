# Contributing to Sherlock

Thanks for considering contributing to our project! Here are some guidelines for submitting issues and contributions.

## Reporting Bugs and Issues

If you encounter a bug, please make sure to:

1. **Search the Issues** to check if the bug has already been reported.
2. **Follow the Bug Report Template** by using the "Bug Report" issue template This will help us gather all the information we need to fix the issue efficiently.

## Suggesting Features

We are always open to suggestiongs for new features. To suggest a feature:

1. **Search the Issues** to see if someone has already suggested the same feature.
2. **Use the Feature Request Template** when creating a new feature request.

Please include:

- A detailed description of the feature
- Possible use caess for the feature
- Any relevant context

## Pull Requests (PRs)

We appreciate your contributions! When submitting a PR, please follow these guidelines:

1. **Fork the repository** and create a new branch for your feature or bug fix.
   - Avoid working directly on the `main` branch

2. **Make sure your code follows Sherlock's coding conventions** and adds tests if necessary.
   - If you're fixing a bug, try to include test that reproduce the issue.
   - If you're adding a feature, try to provide tests that cover your new functionality.

3. **Keep your commits clear and concise.**
   - Write meaningful commit messages that  explain why the change was made.
   - Break large changes into smaller, more managagle commits.

4. **Update documentation**
   - If your PR introduces new features or changes existing functionality, make sure to update the relevant documentation.  Sherlock uses a custom  documentation renderer, provided by the `Documentation` trait. For more documentation, please see: [documentation generator](#)

5. - make sure your changes work properly and don't break existing functionality.
   - if applicable, run the project to verify that everything is functioning as expected.


### Steps for Submitting a PR:

1. **Fork** the repository and **clone** it into your local machine.
2. Create a **new branch** for your changes.
   - Example: `git checkout -b fix/bug-description`

3. **Make your changes** and commit them to your branch
   - Example: `git commit -m "fix(category): fixed issue with bug description in README"`

4. **Push** your changes to your forked repository.
   - Example: `git push origin fix/bug-description`

5. **Open a Pull Request** in the original repository from your fork. Usually, PRs should be targeting the `dev` branch and **not** the `main` branch. If you're createing a new feature where others should also be able to contribute to, feel free to mention @skxxtz to create a new development branch for you.

## General Guidelines

- Be respectful of others
- Provide enough information in your issue or PR description to help us understand the problemm or propsed feature
- If youre reporting a bug, please write steps to reproduce the issue.
- If your PR is ready for review, tag a maintainer or submit for review.

Thank you for your contributions!