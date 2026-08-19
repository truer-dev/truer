#!/bin/sh
set -eu

max=400
body=$(cat)
status=0

section() {
  printf '%s\n' "$body" | awk -v want="$1" '
    $0 ~ "^## *" want "[[:space:]]*$" { inside = 1; found = 1; next }
    /^## / { inside = 0 }
    inside && NF && $0 !~ /^[[:space:]]*<!--/ { size += length($0) + 1 }
    END {
      if (!found) print "missing"
      else if (!size) print "empty"
      else print size
    }
  '
}

for heading in Why Verified; do
  size=$(section "$heading")
  case "$size" in
  missing)
    echo "::error::the description needs a '## $heading' section"
    status=1
    ;;
  empty)
    echo "::error::'## $heading' has no content, only the template comment"
    status=1
    ;;
  *)
    [ "$size" -le "$max" ] || {
      echo "::error::'## $heading' is $size characters, $((size - max)) over the limit of $max"
      status=1
    }
    ;;
  esac
done

exit "$status"
