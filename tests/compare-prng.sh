#!/bin/bash
echo "=== PRNG Call Comparison - Seed 1 ===" 
echo ""
echo "Iteration | JS PRNG | Rust PRNG | Diff | JS HP Changes | Rust HP Changes"
echo "----------|---------|-----------|------|---------------|----------------"

# Parse JS output
while IFS= read -r js_line; do
    if [[ $js_line =~ ^\#([0-9]+):.*prng=([0-9]+)-\>([0-9]+).*P1=\[(.*)\].*P2=\[(.*)\] ]]; then
        iter="${BASH_REMATCH[1]}"
        js_prng="${BASH_REMATCH[2]}->${BASH_REMATCH[3]}"
        js_calls=$((${BASH_REMATCH[3]} - ${BASH_REMATCH[2]}))
        js_p1="${BASH_REMATCH[4]}"
        js_p2="${BASH_REMATCH[5]}"
        
        # Get corresponding Rust line
        rust_line=$(grep "^#${iter}:" /tmp/rust-battle-seed1.txt)
        if [[ $rust_line =~ ^\#([0-9]+):.*prng=([0-9]+)-\>([0-9]+).*P1=\[(.*)\].*P2=\[(.*)\] ]]; then
            rust_prng="${BASH_REMATCH[2]}->${BASH_REMATCH[3]}"
            rust_calls=$((${BASH_REMATCH[3]} - ${BASH_REMATCH[2]}))
            rust_p1="${BASH_REMATCH[4]}"
            rust_p2="${BASH_REMATCH[5]}"
            
            diff=$((js_calls - rust_calls))
            marker=""
            if [ $diff -ne 0 ]; then
                marker=" <<<< DIVERGENCE"
            fi
            
            printf "#%-8s | %-11s | %-13s | %-4s | %-25s | %-25s%s\n" \
                "$iter" "$js_prng ($js_calls)" "$rust_prng ($rust_calls)" "$diff" \
                "$(echo $js_p1 | cut -d'(' -f1)@$(echo $js_p2 | cut -d'(' -f1)" \
                "$(echo $rust_p1 | cut -d'(' -f1)@$(echo $rust_p2 | cut -d'(' -f1)" \
                "$marker"
        fi
    fi
done < /tmp/js-battle-seed1.txt | head -45
