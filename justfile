#!/usr/bin/env -S just --working-directory . --justfile
# SPDX-FileCopyrightText: 2024 David Runge <dave@sleepmap.de>
# SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
# SPDX-License-Identifier: CC0-1.0
#
# Load project-specific properties from the `.env` file

set dotenv-load := true

# Runs all checks for currently checked files

# Since this is a first recipe it's being run by default.
check-files: spelling formatting lints dependencies licenses tests

# Faster checks need to be executed first for better UX.  For example
# codespell is very fast. cargo fmt does not need to download crates etc.

# Installs all tools required for development
install: install-packages install-tools

# Install development packages using pacman
install-packages:
    # Packages that are needed by this justfile are listed directly
    # Any extra packages are set in the `.env` file
    pacman -Syu --needed --noconfirm rustup codespell reuse cargo-deny tangler $PACMAN_PACKAGES

# Installs any user tools required to run development tooling
install-tools:
    rustup default stable
    rustup component add clippy
    rustup toolchain install nightly
    rustup component add --toolchain nightly rustfmt

# Runs all checks for new commits
check-commits: dco

# Runs all tasks intended for the CI environment
ci: check-files e2e

# Checks common spelling mistakes
spelling:
    codespell

# Checks source code formatting
formatting:
    just --unstable --fmt --check
    # We're using nightly to properly group imports, see rustfmt.toml
    cargo +nightly fmt -- --check

# Lints the source code
lints:
    cargo clippy --all -- -D warnings

# Checks for issues with dependencies
dependencies:
    cargo deny check -D warnings -A duplicate -A accepted

# Checks licensing status
licenses:
    reuse lint

# Runs all unit tests
tests:
    cargo test --all

# Runs all end-to-end tests
e2e:
    #!/usr/bin/env bash
    for dir in *; do
        if [ -d "$dir" -a -f "$dir/Cargo.toml" -a -f "$dir/README.md" ]; then
            just test-readme "$dir"
        fi
    done

# Runs per project end-to-end tests found in a project README
test-readme project $PATH=`printf "%s" "$HOME/.cargo/bin:$PATH"`:
    printf "%s\n" "$PATH"
    cargo install --path {{ project }}
    cd {{ project }} && tangler sh < README.md | bash -euxo pipefail -

# Adds git hooks (pre-commit, pre-push)
add-hooks:
    #!/usr/bin/env bash
    echo just check-files > .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit

    echo just check-commits > .git/hooks/pre-push
    chmod +x .git/hooks/pre-push

# Checks for commit sign-offs
dco:
    #!/usr/bin/env bash
    function check_dco {
      for commit in $(git rev-list "$1"); do
        if ! git show -s --format=%B "$commit" | grep -q "Signed-off-by: "; then
          echo "$commit is bad: lacks Signed-off-by line"
          echo "  Please use:"
          echo "    git rebase --signoff main && git push --force-with-lease"
          echo "  See https://developercertificate.org/ for more details."
          exit 1;
        else
          echo "$commit is good."
        fi
      done
    }
    if [ -z "${CI_REPO_DEFAULT_BRANCH-}" ]; then
      check_dco "main.."
    else
      check_dco "origin/$CI_REPO_DEFAULT_BRANCH.."
    fi

# Fixes common issues. Files need to be git add'ed
fix:
    #!/usr/bin/env bash
    if ! git diff-files --quiet ; then
        echo "Working tree has changes. Please stage them: git add ."
        exit 1
    fi

    codespell --write-changes
    just --unstable --fmt
    cargo clippy --fix --allow-staged

    # fmt must be last as clippy's changes may break formatting
    cargo +nightly fmt
