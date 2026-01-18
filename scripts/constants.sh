SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)
PROJECT_DIR=$(cd -- "${SCRIPT_DIR}/.." && pwd)
TARGET_DIR="${SCRIPT_DIR}/fetch_leetcode"

LEETCODE_LOGS="${PROJECT_DIR}/logs/leetcode.log"
CRONTAB_LOGS="${PROJECT_DIR}/logs/crontab.log"
GITHUB_LOGS="${PROJECT_DIR}/logs/github.log"
export GIT_SSH_COMMAND="ssh -i /home/sotsuba/.ssh/ssh_key -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no"
