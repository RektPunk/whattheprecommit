#!/usr/bin/env bash

COMMIT_MSG_FILE=$1
JOKES_FILE="$(dirname "$0")/src/jokes.txt"

if [ -f "$JOKES_FILE" ]; then
    JOKES=()

    while IFS= read -r line; do
        JOKES+=("$line")
    done < <(grep -v '^[[:space:]]*$' "$JOKES_FILE")

    if [ "${#JOKES[@]}" -gt 0 ]; then
        RANDOM_INDEX=$((RANDOM % ${#JOKES[@]}))
        echo "${JOKES[$RANDOM_INDEX]}" > "$COMMIT_MSG_FILE"
        exit 0
    fi
fi

echo "I have no idea what I'm doing." > "$COMMIT_MSG_FILE"
