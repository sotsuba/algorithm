#!/bin/bash 

alert_user() {
    local title=$1
    local msg=$2
    notify-send -i "info" -t 5000 "$title" "$msg"
}

alert_user "github" "No local changes detected repo(algorithm)."
