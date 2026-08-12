# NME 0.0.1-beta.66

Beta.66 hardens native string assignment.

- Use overlap-safe copying for string assignments so `text = text` remains
  valid native behavior without overlapping `memcpy`.
