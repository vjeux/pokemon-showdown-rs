#!/bin/bash

# Efficient batch bisect for all failing seeds
# Tests all seeds at each bisect step to group them by failing commit
# Usage: ./tests/batch-bisect.sh
# Output: bisect-results.txt with seed -> commit mapping

GOOD_COMMIT="6c844c41"
BAD_COMMIT="33613114"
RESULTS_FILE="bisect-results.txt"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"

cd "$REPO_DIR"

# Extract seeds from failing-seeds.txt into an array (bash 3 compatible)
all_seeds=()
while IFS= read -r line; do
    seed=$(echo "$line" | grep -oE '[0-9]+$')
    if [ -n "$seed" ]; then
        all_seeds+=("$seed")
    fi
done < failing-seeds.txt

echo "Total failing seeds: ${#all_seeds[@]}"

# Function to test which seeds fail at a given commit
test_seeds_at_commit() {
    local commit=$1
    shift
    local seeds=("$@")

    # Checkout commit
    git checkout --quiet "$commit" 2>/dev/null

    # Build
    docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_unified 2>&1" > /tmp/build-output.txt
    if [ $? -ne 0 ]; then
        echo "BUILD_FAILED"
        return
    fi

    local failing_seeds=""

    for seed in "${seeds[@]}"; do
        # Run JS and Rust tests
        node tests/test-unified-parallel.js $seed $seed > /tmp/js-bisect.txt 2>&1
        docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_unified $seed $seed 2>/dev/null" > /tmp/rust-bisect.txt

        js_line=$(cat /tmp/js-bisect.txt)
        rust_line=$(cat /tmp/rust-bisect.txt)

        if [ "$js_line" != "$rust_line" ]; then
            if [ -n "$failing_seeds" ]; then
                failing_seeds="$failing_seeds $seed"
            else
                failing_seeds="$seed"
            fi
        fi
    done

    echo "$failing_seeds"
}

# Function to bisect a group of seeds that fail at the same commit range
bisect_group() {
    local good=$1
    local bad=$2
    shift 2
    local seeds=("$@")

    if [ ${#seeds[@]} -eq 0 ]; then
        return
    fi

    echo "Bisecting ${#seeds[@]} seeds between $(git rev-parse --short $good) and $(git rev-parse --short $bad)..."

    # Get list of commits in range (bash 3 compatible)
    commits=()
    while IFS= read -r c; do
        commits+=("$c")
    done < <(git rev-list --reverse "$good".."$bad")

    if [ ${#commits[@]} -eq 0 ]; then
        # Adjacent commits - bad is the culprit
        local commit_msg=$(git log --format="%s" -1 "$bad")
        for seed in "${seeds[@]}"; do
            echo "Seed $seed: $(git rev-parse --short $bad) - $commit_msg" | tee -a $RESULTS_FILE
        done
        return
    fi

    if [ ${#commits[@]} -le 2 ]; then
        # One or two commits between - test first commit to determine which is culprit
        local first_commit="${commits[0]}"
        local failing_at_first=$(test_seeds_at_commit "$first_commit" "${seeds[@]}")

        if [ "$failing_at_first" = "BUILD_FAILED" ]; then
            # Can't test, assume first commit is culprit
            local commit_msg=$(git log --format="%s" -1 "$first_commit")
            for seed in "${seeds[@]}"; do
                echo "Seed $seed: $(git rev-parse --short $first_commit) - $commit_msg" | tee -a $RESULTS_FILE
            done
            return
        fi

        # Check which seeds fail at first commit
        for seed in "${seeds[@]}"; do
            local found=0
            for f in $failing_at_first; do
                if [ "$seed" = "$f" ]; then
                    found=1
                    break
                fi
            done
            if [ $found -eq 1 ]; then
                # Seed fails at first commit - culprit is first commit
                local commit_msg=$(git log --format="%s" -1 "$first_commit")
                echo "Seed $seed: $(git rev-parse --short $first_commit) - $commit_msg" | tee -a $RESULTS_FILE
            else
                # Seed passes at first commit - culprit is in remaining commits
                if [ ${#commits[@]} -eq 1 ]; then
                    local commit_msg=$(git log --format="%s" -1 "$bad")
                    echo "Seed $seed: $(git rev-parse --short $bad) - $commit_msg" | tee -a $RESULTS_FILE
                else
                    local second_commit="${commits[1]}"
                    local commit_msg=$(git log --format="%s" -1 "$second_commit")
                    echo "Seed $seed: $(git rev-parse --short $second_commit) - $commit_msg" | tee -a $RESULTS_FILE
                fi
            fi
        done
        return
    fi

    # Find midpoint
    local mid_idx=$(( ${#commits[@]} / 2 ))
    local mid="${commits[$mid_idx]}"

    echo "Testing at midpoint $(git rev-parse --short $mid) (${#commits[@]} commits in range)..."

    # Test seeds at midpoint
    local failing_at_mid=$(test_seeds_at_commit "$mid" "${seeds[@]}")

    if [ "$failing_at_mid" = "BUILD_FAILED" ]; then
        echo "Build failed at $mid, skipping to next..."
        # Try the next half
        bisect_group "$mid" "$bad" "${seeds[@]}"
        return
    fi

    # Partition seeds into two groups
    local early_failures=()
    local late_failures=()

    for seed in "${seeds[@]}"; do
        local found=0
        for f in $failing_at_mid; do
            if [ "$seed" = "$f" ]; then
                found=1
                break
            fi
        done
        if [ $found -eq 1 ]; then
            early_failures+=("$seed")
        else
            late_failures+=("$seed")
        fi
    done

    echo "  Failing at mid (${#early_failures[@]}): ${early_failures[*]:0:5}..."
    echo "  Passing at mid (${#late_failures[@]}): ${late_failures[*]:0:5}..."

    # Recursively bisect each group
    if [ ${#early_failures[@]} -gt 0 ]; then
        bisect_group "$good" "$mid" "${early_failures[@]}"
    fi
    if [ ${#late_failures[@]} -gt 0 ]; then
        bisect_group "$mid" "$bad" "${late_failures[@]}"
    fi
}

# Initialize results file
echo "Git Bisect Results for Failing Seeds" > $RESULTS_FILE
echo "=====================================" >> $RESULTS_FILE
echo "Good commit: $GOOD_COMMIT (all pass)" >> $RESULTS_FILE
echo "Bad commit: $BAD_COMMIT (HEAD)" >> $RESULTS_FILE
echo "Total seeds: ${#all_seeds[@]}" >> $RESULTS_FILE
echo "" >> $RESULTS_FILE

# Start the bisect process
bisect_group "$GOOD_COMMIT" "$BAD_COMMIT" "${all_seeds[@]}"

# Return to HEAD
git checkout --quiet "$BAD_COMMIT"

echo ""
echo "======================================"
echo "Results written to $RESULTS_FILE"
echo "======================================"

# Summary - count unique commits
echo "" >> $RESULTS_FILE
echo "Summary:" >> $RESULTS_FILE
echo "--------" >> $RESULTS_FILE
grep "^Seed" $RESULTS_FILE | cut -d':' -f2 | sort | uniq -c | sort -rn >> $RESULTS_FILE
