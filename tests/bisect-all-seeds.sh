#!/bin/bash

# Bisect all failing seeds and record which commits caused them
# Usage: ./tests/bisect-all-seeds.sh
# Output: bisect-results.txt with seed -> commit mapping

GOOD_COMMIT="6c844c41"
BAD_COMMIT="33613114"
RESULTS_FILE="bisect-results.txt"

# Extract seeds from failing-seeds.txt
seeds=$(grep -oE '[0-9]+$' failing-seeds.txt | tr '\n' ' ')

echo "Git Bisect Results for Failing Seeds" > $RESULTS_FILE
echo "=====================================" >> $RESULTS_FILE
echo "Good commit: $GOOD_COMMIT" >> $RESULTS_FILE
echo "Bad commit: $BAD_COMMIT" >> $RESULTS_FILE
echo "" >> $RESULTS_FILE

for seed in $seeds; do
    echo "Bisecting seed $seed..."

    # Start bisect
    git bisect start $BAD_COMMIT $GOOD_COMMIT --no-checkout

    # Run the bisect
    result=$(git bisect run sh -c "git checkout --quiet BISECT_HEAD && cd /Users/vjeux/random/showdown/pokemon-showdown-rs && ./tests/bisect-seed.sh $seed" 2>&1)

    # Extract the first bad commit
    bad_commit=$(echo "$result" | grep -oE '[a-f0-9]{40} is the first bad commit' | cut -d' ' -f1)

    if [ -n "$bad_commit" ]; then
        short_commit=$(git rev-parse --short $bad_commit)
        commit_msg=$(git log --format="%s" -1 $bad_commit)
        echo "Seed $seed: $short_commit - $commit_msg" | tee -a $RESULTS_FILE
    else
        echo "Seed $seed: BISECT FAILED" | tee -a $RESULTS_FILE
    fi

    # Reset bisect for next seed
    git bisect reset

done

echo ""
echo "Results written to $RESULTS_FILE"
