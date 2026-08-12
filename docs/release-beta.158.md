# NME 0.0.1-beta.158

Beta158 extends the shared Python context checks across semicolon-separated
simple statements. A star import after an earlier statement in a one-line
function or class suite now receives `E0114`, and `break`, `continue`, or
`return` after an earlier statement in an `except*` handler now receives
`E0115` instead of falling through to the generic CLI diagnostic.
