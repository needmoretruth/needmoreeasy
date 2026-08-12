# NME 0.0.1-beta.146

Beta146 adds shared bilingual `E0113` for Python `nonlocal` without an
enclosing function. Valid nested functions and classes remain byte-identical,
while CPython continues to validate whether the requested outer name exists.
