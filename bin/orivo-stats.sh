#!/usr/bin/env bash

set -euo pipefail

process_name="${1:-orivo}"

watch -n 1 "
pid=\$(pgrep -n \"$process_name\")

if [ -z \"\$pid\" ]; then
    echo \"Process '$process_name' is not running\"
    exit 0
fi

ps -p \$pid -o pid,%cpu,%mem,rss,command | awk '
NR == 1 { print; next }
{
    printf \"PID: %s | CPU: %s%% | Memory (%%): %s%% | Memory: %.2f MB | CMD: %s\\n\",
        \$1, \$2, \$3, \$4 / 1024, substr(\$0, index(\$0, \$5))
}'
"
