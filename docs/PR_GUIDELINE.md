# Pull Request Guidelines

## Purpose

This file defines the pull request (PR) policy for the Reinhardt project. These rules ensure clear communication, proper review process, and consistent PR formatting across the development lifecycle.

---

## Language Requirements

### LR-1 (MUST): English-Only Policy

- **ALL** PR titles MUST be written in English
- **ALL** PR descriptions MUST be written in English
- **ALL** PR comments and discussions MUST be written in English
- This ensures accessibility for international contributors and maintainers

**Rationale:**
- GitHub is an international platform
- English is the lingua franca of software development
- Enables broader collaboration and code review
- Facilitates automated tooling and CI/CD integration

---

## PR Creation Policy

### PC-1 (MUST): Use GitHub CLI

- **MUST** use GitHub CLI (`gh`) for creating pull requests
- **NEVER** use web browser UI for PR creation when CLI is available
- CLI ensures consistency and can be automated

**Example:**
```bash
gh pr create --title "feat(auth): add JWT token validation" \
  --body "$(cat <<'EOF'
## Summary

- Implement JWT token validation with RS256 algorithm
- Add token expiration checking
- Include unit tests for edge cases

## Test plan

- [x] `cargo test --package reinhardt-auth` passes
- [x] All existing tests pass
- [x] Manual testing with expired tokens

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

### PC-2 (MUST): Branch Naming

- Branch names SHOULD follow the pattern: `<type>/<scope>/<short-description>`
- Types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, etc.
- Scope: Module or component name
- Short description: Kebab-case brief summary

**Examples:**
```
feat/auth/jwt-validation
fix/orm/connection-pool-race-condition
refactor/http/middleware-pipeline
docs/api/openapi-spec
test/database/integration-tests
chore/ci/github-actions-update
```

### PC-3 (SHOULD): Draft PRs for Work in Progress

- Use draft PRs for incomplete work
- Convert to ready for review when all tests pass
- Draft PRs allow early feedback without formal review requests

**Example:**
```bash
gh pr create --draft --title "feat(auth): add JWT validation (WIP)"
```

---

## PR Title Format

### TF-1 (MUST): Follow Conventional Commits

PR titles MUST follow the same format as commit messages:

```
<type>[optional scope][optional !]: <description>

Examples:
feat(auth): add JWT token validation with RS256 algorithm
fix(orm): resolve race condition in connection pool
feat(api)!: change response format to JSON:API specification
```

**Requirements:**
- **Type**: One of the defined types (feat, fix, refactor, docs, etc.)
- **Scope**: Module or component name (OPTIONAL but RECOMMENDED)
- **Breaking Change Indicator**: Append `!` for breaking changes
- **Description**: Concise summary in English
  - **MUST** start with lowercase letter
  - **MUST** be specific and descriptive
  - **MUST NOT** end with a period
  - Keep under 72 characters for readability

**See**: @docs/COMMIT_GUIDELINE.md for detailed commit type definitions

---

## PR Description Format

### DF-1 (MUST): Standard Structure

PR descriptions MUST follow this structure:

```markdown
## Summary

- Bullet point 1: Brief description of change
- Bullet point 2: Another change
- Bullet point 3: Additional context

## Test plan

- [ ] Test case 1
- [ ] Test case 2
- [ ] Test case 3
- [ ] Manual testing notes

## Breaking Changes (if applicable)

- Breaking change 1: Migration path
- Breaking change 2: Impact and solution

## Related Issues (if applicable)

Fixes #123
Closes #456
Refs #789

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

**Requirements:**

1. **Summary Section** (REQUIRED)
   - Use bullet points for clarity
   - List key changes in logical order
   - Be specific about what changed and why
   - Mention new features, bug fixes, or improvements

2. **Test Plan Section** (REQUIRED)
   - List all verification steps
   - Include automated test commands
   - Note manual testing performed
   - Use checkboxes (`- [ ]` or `- [x]`) for tracking

3. **Breaking Changes Section** (REQUIRED for breaking changes)
   - List all API changes that break compatibility
   - Provide migration path for each change
   - Explain impact on existing code

4. **Related Issues Section** (OPTIONAL)
   - Link related issues using GitHub keywords
   - Use `Fixes #123` to auto-close issues
   - Use `Refs #123` for related but not closed issues

5. **Footer** (REQUIRED)
   - Include Claude Code attribution for AI-assisted PRs

### DF-2 (SHOULD): Additional Context

Include additional sections when relevant:

- **Migration Guide**: For breaking changes with complex migration
- **Performance Impact**: For performance-related changes
- **Security Considerations**: For security-related changes
- **Documentation**: Links to updated documentation
- **Screenshots**: For UI changes (use relative paths or URLs)

---

## PR Review Process

### RP-1 (MUST): Pre-Review Checklist

Before requesting review, ensure:

- [ ] All CI checks pass
- [ ] All tests pass locally
- [ ] Code follows project style guidelines
- [ ] Documentation is updated
- [ ] Commit history is clean and logical
- [ ] PR description is complete and accurate

**Commands to run:**
```bash
cargo check --workspace --all --all-features
cargo test --workspace --all --all-features
cargo make fmt-check
cargo make clippy-check
```

### RP-2 (SHOULD): Self-Review

- Review your own PR before requesting review from others
- Check for:
  - Unnecessary debug code or comments
  - Proper error handling
  - Test coverage
  - Documentation completeness
  - Code clarity and readability

### RP-3 (MUST): Address Review Comments

- Respond to all review comments
- Mark conversations as resolved when addressed
- Request re-review after making changes
- Be respectful and constructive in discussions

### RP-4 (SHOULD): Keep PRs Small

- Aim for PRs under 400 lines of changes
- Split large features into multiple PRs
- Each PR should have a single, clear purpose
- Smaller PRs are easier to review and less risky to merge

---

## PR Merge Policy

### MP-1 (MUST): Merge Requirements

A PR can only be merged when:

- All CI checks pass
- All conversations are resolved
- At least one approval from a maintainer (if required by repo settings)
- No merge conflicts with base branch
- All commits follow commit guidelines (@docs/COMMIT_GUIDELINE.md)

### MP-2 (MUST): Merge Strategy

**Squash and Merge** (Default):
- Combine all PR commits into a single commit
- Use PR title as commit message
- Include PR description in commit body
- Use for feature branches with multiple interim commits

**Rebase and Merge**:
- Preserve individual commits
- Use when commits are already well-structured
- Each commit MUST follow commit guidelines
- Prefer for PRs with clean, logical commit history

**Merge Commit** (Avoid):
- Creates additional merge commit
- Only use for merging long-lived branches
- Generally avoid for feature branches

### MP-3 (SHOULD): Delete Branch After Merge

- Delete feature branches after successful merge
- Keeps repository clean
- Use GitHub's automatic branch deletion feature

---

## Special Cases

### Release PRs

For release preparation PRs (version bumps):

**Title Format:**
```
chore(release): bump [crate-name] to v[version]

Example:
chore(release): bump reinhardt-core to v0.2.0
```

**Description Format:**
```markdown
## Summary

Prepare for crate publication to crates.io.

Version Changes:
- crates/[crate-name]/Cargo.toml: version [old-version] -> [new-version]
- crates/[crate-name]/CHANGELOG.md: Add release notes for v[new-version]

## Breaking Changes (if MAJOR version bump)

- List breaking changes here
- API changes that affect backward compatibility

## New Features (if MINOR version bump)

- List new features here
- Enhancements and additions

## Bug Fixes (if PATCH version bump)

- List bug fixes here
- Resolved issues and corrections

## Test plan

- [x] `cargo check -p [crate-name] --all-features`
- [x] `cargo test -p [crate-name] --all-features`
- [x] `cargo publish --dry-run -p [crate-name]`
- [ ] Ready for publish after PR merge

🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

**See**: @docs/RELEASE_PROCESS.md for detailed release procedures

### Documentation-Only PRs

For documentation changes:

**Title Format:**
```
docs(<scope>): <description>

Example:
docs(api): update OpenAPI specification for v0.2.0
docs(readme): add installation instructions
```

**Description:**
- List all documentation files changed
- Note what information was added/updated/removed
- Include links to rendered documentation if available

---

## Quick Reference

### ✅ MUST DO
- Write all PR content in English
- Use `gh pr create` for creating PRs
- Follow Conventional Commits format for titles
- Include Summary and Test plan sections
- Run all checks before requesting review
- Address all review comments
- Ensure all CI checks pass before merge

### ❌ NEVER DO
- Write PR titles or descriptions in non-English languages
- Create PRs without proper description
- Skip test plan section
- Merge with failing CI checks
- Leave unresolved review comments
- Force push after review has started (unless explicitly requested)

---

## Related Documentation

- **Main Quick Reference**: @CLAUDE.md (see Quick Reference section)
- **Commit Guidelines**: @docs/COMMIT_GUIDELINE.md
- **Release Process**: @docs/RELEASE_PROCESS.md
- **GitHub CLI Documentation**: https://cli.github.com/manual/
