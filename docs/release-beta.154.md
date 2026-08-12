# NME 0.0.1-beta.154

Beta154 keeps valid `yield`, `await`, and bare `return` inside one-line Python
function suites in their proper function context. It also reports a value
return inside an inline async generator with the shared bilingual `E0118`
diagnostic, without leaking nested function bodies into their parents.
