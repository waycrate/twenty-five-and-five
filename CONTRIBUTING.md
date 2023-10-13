# Contributing

We expect all contributors to follow a set of rules and also follow [CODE OF CONDUCT](./CODE_OF_CONDUCT.md).

## How to contribute

- Create a separate branch where you prefix a date with format `%4Y%m%d` e.g. `20231010-branch-name` where branch name follows the formats below:
  - If it's a fix, name the branch `fix/yourfixname`.
  - If it's a feature, name the branch `feature/yourfeature`.
  - If it cannot be determined, name the branch `misc/yourplansandwishes`.
- Once you are done with everything, please format using the defaults of `cargo fmt` or `rustfmt`.
- Check for screaming lints from `clippy` by running `cargo clippy`.
- Open a PR. Wait for someone to review.
  - Wait for the review to tell you what to resolve.
  - Resolve any issues.
  - Get your PR approved
  - Win.
- END. Now go have a coffee.