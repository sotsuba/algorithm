#!/bin/bash 

SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(cd -- "${SCRIPT_DIR}/.." && pwd)
STAMP_LOGS="${PROJECT_DIR}/logs/stamps.log"

cd "$PROJECT_DIR" || exit 1

has_changes() {
    [[ -n $(git status --porcelain) ]]
}

is_sunday() {
    [[ "$(date +%u)" -eq 7 ]]
}

sync_for_the_day() {
    local timestamp=$(date +'%Y-%m-%d')

    echo "[$(date)] Starting daily sync..." | tee -a "$STAMP_LOGS"
    {
        git add . && \
        git commit -m "daily-sync: $timestamp" && \
        git push origin main
    } | tee -a "$STAMP_LOGS" 2>&1

    return $?
}

sync_for_the_week() {
    local week_num=$(date +%V)
    local timestamp=$(date +'%Y-%m')
    local base_sha=$(git rev-list -n 1 --before="7 days ago" HEAD)

    if [[ -n "$base_sha" ]]; then 
        echo "[$(date)] Starting weekly squash..." | tee -a "$STAMP_LOGS"
        {
            git reset --soft "$base_sha" && \
            git commit -m "weekly-sync(${week_num}): $timestamp" && \
            git push origin main --force-with-lease 
        } >> "$STAMP_LOGS" 2>&1
        return $?
    fi 
    return 0
}

alert_user() {
    local title=$1
    local msg=$2
    notify-send -i "info" -t 5000 "$title" "$msg"
}

main() {
    if has_changes; then 
        sync_for_the_day
    else 
        alert_user "github" "No local changes detected repo(algorithm)."
    fi 

    if is_sunday; then 
        sync_for_the_week 
    fi
}

main