```text
                               Algorithm
                          ====================

This is a personal repository for tracking progress through data structures
and algorithms. It contains solutions implemented in Rust, Python, and C++.

REPOSITORY STRUCTURE
-------------
The repository is structured as follows:

  cses/          Practice ground for algorithms.
  leetcode/      Challenge solutions.
  logs/          Crontab, GitHub, and fetcher execution logs.
  marisaoj/      Algorithm practice platform.
  playground/    Personal sandbox for prototyping.
  scripts/       Automation and sync tools.

PREREQUISITES
-------------
  - Rust, Cargo.
  - C++17 or later.
  - Python 3.10+

AUTOMATION
----------

The automation scripts are designed to be scheduled via crontab. It is 
recommended to use absolute paths or to change into the script directory 
before execution to ensure environment variables are loaded correctly.

Example crontab configuration (This is my setup):
  # 1. Fetch data from LeetCode (21:55)
  55 21 * * * /bin/bash /path/to/repo/scripts/fetch_leetcode.sh

  # 2. Sync changes to GitHub (22:00)
  0 22 * * * /bin/bash /path/to/repo/scripts/sync.sh


THIS IS FOR YOU <3 
----------

If you are not interested in the notes, the 'fetch_leetcode' tool in the
scripts directory provides an automated engine for syncing LeetCode data.
This can be used as a source for external services, such as a Telegram bot.

The tool is currently in a "bare-bones" state and is not fully documented.
If you wish to use it, please raise an issue on GitHub, and documentation
will be provided upon request.