#!/bin/bash 

PROJECT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)
cd "$PROJECT_DIR" || exit 1

mkdir -p "${PROJECT_DIR}/logs"
STAMP_LOGS="${PROJECT_DIR}/logs/stamps.log"

has_changes() {
    [[ -n $(git status --porcelain) ]]
}

is_sunday() {
    [[ "$(date +%u)" -eq 7 ]]
}

sync_for_the_day() {
    local timestamp=$(date +'%Y-%m-%d')

    if has_changes; then 
        echo "[$(date)] Starting daily sync..." >> "$STAMP_LOGS"
        {
            git add . && \
            git commit -m "daily-sync: $timestamp" && \
            git push origin main
        } >> "$STAMP_LOGS" 2>&1
        return $?
    fi
    return 0
}

sync_for_the_week() {
    local week_num=$(date +%V)
    local timestamp=$(date +'%Y-%m')
    local base_sha=$(git rev-list -n 1 --before="7 days ago" HEAD)

    if [[ -n "$base_sha" ]]; then 
        echo "[$(date)] Starting weekly squash..." >> "$STAMP_LOGS"
        {
            git reset --soft "$base_sha" && \
            git commit -m "weekly-sync(${week_num}): $timestamp" && \
            git push origin main --force-with-lease 
        } >> "$STAMP_LOGS" 2>&1
        return $?
    fi 
    return 0
}

main() {
    if sync_for_the_day; then
        if is_sunday; then 
            sync_for_the_week 
        fi
    else
        echo "Daily sync failed. Aborting weekly squash for safety." >&2
        exit 1
    fi
}

main