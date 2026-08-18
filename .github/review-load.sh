#!/bin/sh
set -eu

base="$1"
kind="$2"
max_files=15

fail() {
  echo "::error::$*"
  exit 1
}

changed_files() {
  git diff --name-only "$base...HEAD" -- . ':(exclude)Cargo.lock'
}

changed_lines() {
  git diff --numstat "$base...HEAD" -- . ':(exclude)Cargo.lock' |
    awk '{ total += $1 + $2 } END { print total + 0 }'
}

files=$(changed_files | grep -c . || true)
lines=$(changed_lines)

echo "$kind: $files files, $lines lines"

if [ "$kind" = docs ]; then
  stray=$(changed_files | grep -v '\.md$' || true)
  [ -z "$stray" ] ||
    fail "a docs pull request may only change markdown, found $(echo "$stray" | tr '\n' ' ')"
else
  case "$kind" in
  fix) cap=100 ;;
  feat | perf | test) cap=300 ;;
  refactor) cap=400 ;;
  *) cap=150 ;;
  esac

  [ "$lines" -le "$cap" ] ||
    fail "a $kind pull request caps at $cap lines, this one is $lines"
fi

[ "$files" -le "$max_files" ] ||
  echo "::warning::$files files is a lot to hold in your head at once"
