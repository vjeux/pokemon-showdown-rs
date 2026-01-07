# Seed 21 Queue State Investigation

## The Mystery
When Aggron faints at iteration 35, the queue should contain remaining actions (like Residual).  
But in iteration 36, commit_choices shows `old_queue has 0 actions`.

## Evidence
From `/tmp/rust-battle-seed21-full.txt`:

**Iteration 35 Entry (line ~111xxx):**
- Queue should have [BeforeTurn, Move1, Move2, Residual]
- Processes BeforeTurn
- Processes moves, Aggron faints  
- request_state set to Switch
- Returns early WITHOUT clearing queue

**Iteration 36 Commit Choices (line 111453):**
- `[COMMIT_CHOICES] Old queue has 0 actions before clearing`
- ❌ Queue is EMPTY! Where did the actions go?

## Question
What happens to the queue between:
1. Iteration 35's turn_loop returning (queue should have remaining actions)
2. Iteration 36's commit_choices starting (queue is empty)

## Hypotheses
1. Maybe commit_choices is called INSIDE turn_loop when request_state changes?
2. Maybe the test harness clears the queue between iterations?
3. Maybe make_request or clear_request clears the queue?
4. Maybe the queue gets cleared in a different code path we haven't checked?

## Next Steps
1. Add logging to track EVERY queue modification (clear, extend, add, remove)
2. Check if commit_choices gets called recursively
3. Verify JavaScript behavior matches - does IT have an empty old_queue for the switch?
4. Add a queue state assertion before each commit_choices to catch when it gets cleared

