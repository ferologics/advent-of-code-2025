# Run a day's solution (e.g., just run 1 or just run 01)
run day:
    cargo run -p day_$(printf "%02d" {{day}})

# Create a new day's crate
new:
    #!/usr/bin/env bash
    set -euo pipefail
    # Find highest existing day number
    last=$(ls -d day_* 2>/dev/null | sed 's/day_//' | sort -n | tail -1)
    if [ -z "$last" ]; then
        next=1
    else
        next=$((10#$last + 1))
    fi
    name=$(printf "day_%02d" $next)
    cargo new "$name"
    echo "Created $name"
