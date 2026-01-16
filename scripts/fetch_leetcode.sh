#!/bin/bash
ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
. "${ROOT_DIR}/constants.sh"

cd "$TARGET_DIR" || exit 1

echo "$(date) - crontab(fetch_leetcode)" >> $CRONTAB_LOGS

"${SCRIPT_DIR}/fetch_leetcode/target/release/fetch_leetcode" >> $LEETCODE_LOGS 2>&1