# Maintaining the official project

slaterain controls the official main branch and releases. External contributors
use forks and pull requests; no collaborator invitations are needed.

## Accepting changes

Main requires a pull request, code-owner approval, resolved conversations and
the two CI jobs: `check (core)` and `check (viewer)`. New pushes dismiss old
approvals. Force pushes and deletion are blocked. CODEOWNERS includes workflows.
The owner has a PR-only ruleset bypass for their own changes (GitHub cannot
accept a self-approval); use it deliberately, after checks pass. No other
contributors receive bypass rights. Changing repository settings remains an
owner responsibility.

Keep forks' CI unprivileged: use pull_request, read-only token permissions,
GitHub-hosted runners and no secrets. Never execute contributed code in a
privileged pull_request_target workflow. Inspect workflow changes carefully.

## Releasing a playtest

1. Merge a checked source commit and compare the relevant route against the
   original game. Record outstanding limitations, not just screenshots.
2. From a clean Windows checkout with stable Rust, run:
   `./scripts/package-playtest.ps1 -Version experimental-YYYY-MM-DD.N`.
   It builds the viewer with the lockfile and packages only approved files.
3. Test the extracted ZIP with Play.ps1 -ValidateOnly and a live launch. Test
   paths containing spaces and an invalid Data folder. Verify SHA256 and BUILD.txt.
4. Create a release tag at that exact source commit. Create a draft pre-release,
   attach the ZIP and its .sha256 file, and describe verified behavior, known
   issues, controls, required owned game data and reporting instructions.
5. Publish after reviewing the draft. Official builds are maintainer-published;
   ordinary pushes and contributor PRs never publish a release automatically.

Dependency updates require refreshing distribution/licenses and
THIRD-PARTY-NOTICES.txt before packaging. Keep their upstream provenance and
copyright notices. Packaging a license list alone is insufficient.

## Work queue and AI handoff

Keep a small set of milestone issues. Contributors comment to claim a bounded
task before starting; assignment is not automatic acceptance of a design.
Each handoff identifies branch/commit, changed files, verified evidence, test
results, unresolved questions and the next action. Do not paste entire chats.

The initial private upload history was retained separately because it included
build outputs. The clean public history starts with the reconciled source;
do not publish private research or old build directories to restore that history.
