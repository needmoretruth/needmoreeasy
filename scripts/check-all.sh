#!/usr/bin/env bash
# Every documentation check, in one command that actually fails.
#
#   bash scripts/check-all.sh
#
# Piping a checker into `tail` was how a guide with a program that does not
# compile got committed: the pipeline's exit status is `tail`'s, which is
# always 0. This runs each one whole and stops at the first failure.
set -euo pipefail
cd "$(dirname "$0")/.."

for check in \
  check-guide-code \
  check-guide-output \
  check-guide-silent \
  check-guide-index \
  check-doc-parity \
  check-prose-blocks \
  check-syntax-reference \
  check-common-english-words \
  check-version \
  check-tier-parity
do
  printf '── %s\n' "$check"
  python3 "scripts/$check.py"
done

echo
echo "문서 검사 전부 통과했습니다"
