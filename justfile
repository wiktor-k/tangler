# SPDX-FileCopyrightText: 2023 Wiktor Kwapisiewicz <wiktor@metacode.biz>
# SPDX-License-Identifier: CC0-1.0

# Faster checks need to be executed first for better UX.  For example
# codespell is very fast. cargo fmt does not need to download crates etc.

check-files: spelling formatting lints dependencies licenses tests
check-commits: dco

spelling:
  codespell

formatting:
  # We're using nightly to properly group imports, see rustfmt.toml
  cargo +nightly fmt -- --check

lints:
  cargo clippy --all -- -D warnings

dependencies:
  cargo deny check -D warnings -A duplicate -A accepted

licenses:
  reuse lint

tests:
  cargo test --all

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

fix:
  #!/usr/bin/env bash
  if ! git diff-files --quiet ; then
      echo "Working tree has changes. Please stage them: git add ."
      exit 1
  fi

  codespell --write-changes
  cargo clippy --fix --allow-staged

  # fmt must be last as clippy's changes may break formatting
  cargo +nightly fmt
