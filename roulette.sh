#!/usr/bin/env bash

COMMIT_MSG_FILE=$1
GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || echo ".")
JOKES_FILE="$GIT_ROOT/src/jokes.txt"

if [ $((RANDOM % 2)) -eq 0 ]; then
    API_MSG=$(curl -s --max-time 2 https://whatthecommit.com/index.txt | tr -d '\r\n')
    if [ ! -z "$API_MSG" ]; then
        echo "$API_MSG" > "$COMMIT_MSG_FILE"
        exit 0
    fi
fi

if [ -f "$JOKES_FILE" ]; then
    LINES_COUNT=$(grep -c '[^[:space:]]' "$JOKES_FILE")

    if [ "$LINES_COUNT" -gt 0 ]; then
        RANDOM_LINE=$(( (RANDOM % LINES_COUNT) + 1 ))
        CHOSEN_JOKE=$(grep -v '^[[:space:]]*$' "$JOKES_FILE" | sed -n "${RANDOM_LINE}p")
        echo "$CHOSEN_JOKE" > "$COMMIT_MSG_FILE"
        exit 0
    fi
fi

echo "I have no idea what I'm doing." > "$COMMIT_MSG_FILE"
