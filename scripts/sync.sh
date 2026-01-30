#!/bin/bash 

ROOT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
source "${ROOT_DIR}/constants.sh"

cd "$PROJECT_DIR" || exit 1

has_changes() {
    [[ -n $(git status --porcelain) ]]
}

is_sunday() {
    [[ "$(date +%u)" -eq 7 ]]
}

sync_for_the_day() {
    local timestamp=$(date +'%Y-%m-%d')

    git add .

    local cses_count=$(git diff --cached --name-only | grep "problems/cses/.*\.rs" | wc -l)
    local cf_count=$(git diff --cached --name-only | grep "problems/codeforces/.*\.rs" | wc -l)
    local lc_count=$(git diff --cached --name-only | grep "problems/leetcode/.*\.rs" | wc -l)

    local commit_msg="daily-sync: $timestamp | cses: $cses_count | cf: $cf_count | lc: $lc_count"

    echo "[$(date)] Starting daily sync..." | tee -a "$GITHUB_LOGS"
    {
        git commit -m "$commit_msg" && \
        git push origin main
    } | tee -a "$GITHUB_LOGS" 2>&1

    return $?
}

sync_for_the_week() {
    local week_num=$(date +%V)
    local timestamp=$(date +'%Y-%m')
    local base_sha=$(git rev-list -n 1 --before="7 days ago" HEAD)

    if [[ -n "$base_sha" ]]; then 
        echo "[$(date)] Starting weekly squash..." | tee -a "$GITHUB_LOGS"
        {
            git reset --soft "$base_sha" && \
            git commit -m "weekly-sync(${week_num}): $timestamp" && \
            git push origin main --force-with-lease 
        } >> "$GITHUB_LOGS" 2>&1
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
    echo "$(date) - crontab(sync)" >> $CRONTAB_LOGS
    
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