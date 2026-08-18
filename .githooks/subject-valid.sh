#!/bin/sh
set -eu

subject="$1"
max=72
types='feat|fix|test|refactor|perf|docs|chore|ci|build|revert'
root=$(git rev-parse --show-toplevel)

scopes=workspace
for crate in "$root"/crates/*/; do
  [ -d "$crate" ] || continue
  crate=${crate%/}
  scopes="$scopes|${crate##*/}"
done

echo "$subject" | grep -Eq "^($types)\(($scopes)\)!?: ." || {
  echo "subject does not follow the commit convention" >&2
  echo "     got  $subject" >&2
  echo "    want  type(scope): summary" >&2
  echo "   types  $(echo "$types" | sed 's/|/, /g')" >&2
  echo "  scopes  $(echo "$scopes" | sed 's/|/, /g')" >&2
  exit 1
}

length=${#subject}
[ "$length" -le "$max" ] || {
  echo "subject is $length characters, $((length - max)) over the limit of $max" >&2
  echo "  $subject" >&2
  exit 1
}
