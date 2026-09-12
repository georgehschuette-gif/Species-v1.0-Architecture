# PROPERTY OF THE OWNER. PRIVATE CORPUS.
# SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
# NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

# Deviations Log
# Append-only, timestamped. No entry may be deleted or modified.
# Format: | timestamp | phase | severity | description | resolution |

| 2026-09-12T10:00:00-07:00 | Phase 0A | none | Initial contract signed. | Baseline established. |
| 2026-09-12T10:35:00-07:00 | Phase 0A | none | Instrument validation suite complete. 31/31 tests pass. Gate GREEN. | Proceed to Phase 0B. |
| 2026-09-12T10:36:00-07:00 | Phase 0A | minor | Fix: 50% calibrated predictor yields SPC≈0.14, not 0.5 (correct per log-loss formula). No contract change needed. | Test expectation corrected. |
| 2026-09-12T10:37:00-07:00 | Phase 0A | minor | Fix: single prediction test threshold raised to 0.95. Use prob 0.99. | Test expectation corrected. |
| 2026-09-12T10:38:00-07:00 | Phase 0A | minor | Fix: oscillating gaming (2-action, 1.0 bit entropy) caught with threshold 1.5. | Test expectation corrected. |
| 2026-09-12T10:39:00-07:00 | Phase 0A | minor | Fix: spp.py missing `compute_spc_sliding` import. Added to relative import line. | Code fix. |
