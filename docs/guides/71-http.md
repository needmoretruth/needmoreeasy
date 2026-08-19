# 71 — HTTP — asking a web server

English | [한국어](71-http.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [37 — Files](37-files.md), [64 — Python packages](64-python-packages.md)
- Topic: the internet
- Result: fetching a page from a local server

HTTP is how programs ask web servers for pages. The example
`examples/http-client.nme` asks your own computer for a file over
`http://localhost:8000`. It is a learning project, not advice.

Serve a folder first, then run the example in a second terminal:

```sh
python3 -m http.server 8000
```

```sh
nme run examples/http-client
```

```text
the server answered: hello from http!
```

## Steps

1. `urllib` is a standard Python library, so it needs no installation. The
   program imports the part that talks to servers and picks a URL:

   ```nme
   # part of examples/http-client.nme
   import urllib.request

   url = "http://localhost:8000/hello.txt"
   ```

   `localhost` means "this computer", and `8000` is the port the server from
   the first terminal listens on.

2. `urlopen` opens a connection to that URL and returns a response. The
   response is not text yet — it is bytes — so it must be read and decoded:

   ```nme
   # part of examples/http-client.nme
   with urllib.request.urlopen(url) as response:
       body = response.read().decode("utf-8")
   ```

   The `with` block closes the connection when it ends, and
   `.decode("utf-8")` turns the bytes into a string.

3. The last line shows the answer. `body.strip()` removes the newline the
   server sent with the file:

   ```nme
   # part of examples/http-client.nme
   show "the server answered: " + body.strip()
   ```

4. `nme check` verifies the program even without a server — checking only
   needs the syntax, running needs the server:

   ```sh
   nme check examples/http-client
   ```

5. The Korean twin `examples/http-client.ko.nme` writes the same program; only
   the final `show` becomes `말해 "서버 응답: " + body.strip()`. Run
   `nme r examples/http-client.ko` against the same folder for the same result
   in Korean.

## Try it yourself

Add a second file, `hello2.txt`, to the folder you serve and change `url` to
point at it. Restart nothing — the server reads files on demand — then run the
example again to see the new file.

## What you learned

- `python3 -m http.server 8000` serves the current folder on `localhost:8000`.
- `urllib.request.urlopen(url)` opens a connection and returns a response.
- A response must be read with `.read()` and decoded with `.decode("utf-8")`.
- `body.strip()` removes the newline the server sent with the file.
