#!/usr/bin/env bash

set -e

TEST_FILE=$1
LOG_FILE="log.txt"

python3 "${TEST_FILE}" |& tee "${LOG_FILE}"

ERROR_PATTERN="assert sum_even_numbers(numbers) == 12"
if grep -q "$ERROR_PATTERN" "$LOG_FILE"; then
    rm "$LOG_FILE"
    exit 0
else
    rm "$LOG_FILE"
    exit 1
fi
