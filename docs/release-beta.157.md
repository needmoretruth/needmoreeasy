# NME 0.0.1-beta.157

Beta157 extends the inline Python scope checks across semicolon-separated simple
statements. `return`, `break`, and `continue` after an earlier statement now
receive their shared context diagnostic instead of falling through to E9012.
