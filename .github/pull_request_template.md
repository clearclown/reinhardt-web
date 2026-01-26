<!--
Thank you for contributing to Reinhardt!

Please fill out the template below. PRs that don't include enough
information may be closed or requested for changes.

For the PR title, please follow Conventional Commits format:
  <type>[optional scope]: <description>
  Example: feat(auth): add JWT token validation
-->

## Summary

<!-- What does this PR do? Explain the motivation and context, not just what changed. -->
<!-- For bug fixes: What was the bug and what caused it? -->
<!-- For features: What problem does this solve? -->

-

## Type of Change

<!-- Mark the relevant option with an "x" -->

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)
- [ ] Test update

## How Was This Tested?

<!-- Describe the tests you ran to verify your changes. -->
<!-- Provide commands and their output, or screenshots if applicable. -->

**Test commands:**
```bash
# Example:
cargo nextest run --package <package-name>
```

**Test results:**
<!-- Paste relevant test output here -->

## Checklist

<!-- Mark completed items with an "x" -->

- [ ] I have searched for existing PRs/issues related to this change
- [ ] My code follows the project's style guidelines
- [ ] I have run `cargo make fmt-check` and fixed any issues
- [ ] I have run `cargo make clippy-check` and fixed any warnings
- [ ] I have added/updated tests that prove my fix is effective or my feature works
- [ ] All tests pass locally (`cargo test --workspace --all --all-features`)
- [ ] I have updated the documentation accordingly (if applicable)

## Breaking Changes

<!-- If this PR introduces breaking changes, list them here with migration paths -->
<!-- Delete this section if not applicable -->

- **Change**: Description of the breaking change
- **Migration**: How users should update their code

## Related Issues

<!-- Link related issues using keywords: Fixes #123, Closes #456, Refs #789 -->
<!-- Delete this section if not applicable -->

---
Generated with [Claude Code](https://claude.com/claude-code)
