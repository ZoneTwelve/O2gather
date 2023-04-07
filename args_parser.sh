#!/bin/bash
# Current progress: failed
# Parameter parser
target=$1
ARGS=()
PRE_NAME=
for a in "$@"
do
  if [[ "$a" == "--"* ]]; then
    echo "$a is argement"
    PRE_NAME=$a
  else
    ARGS+=(($PRE_NAME $a))
    echo "$PRE_NAME = $a"
  fi
done

echo "$ARGS"
