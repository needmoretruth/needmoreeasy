# 76 — Network: a mini chat

English | [한국어](76-net.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [72 — Weather](72-weather.md), [29 — Playlist](29-playlist.md)
- Topic: the internet
- Result: fetching messages from a local HTTP server and showing them like a mini chat

A chat client reads messages from a server and sends new ones. This guide
fetches a JSON message list from a local server, prints it like a chat log,
and shows exactly what a new reply would look like. A plain static server can
serve the messages; the reply step explains why sending needs a server that
accepts it.

## Steps

1. Create `messages.json` with a short chat log:

   ```nme
   [
     {"who": "Mina", "text": "hello"},
     {"who": "Jun", "text": "hi Mina"},
     {"who": "Mina", "text": "ready to code?"}
   ]
   ```

2. Serve the folder with Python's built-in HTTP server, as in guide
   [72](72-weather.md):

   ```sh
   python3 -m http.server 8000
   ```

3. The program fetches the list and prints each message:

   ```nme
   from urllib.request import urlopen
   from json import loads

   url = "http://localhost:8000/messages.json"
   messages = loads(urlopen(url).read().decode("utf-8"))

   for message in messages:
       show f"{message['who']}: {message['text']}"
   ```

   Each dict holds a `who` and a `text`; `message["who"]` reads one field.
   Reading a page is a GET — any web server can answer it.

4. A reply is the same shape: a dict that would be added to the list. The
   full program builds the reply and prints exactly what it would send:

   ```nme
   # chat.nme — fetch a chat log, then show the reply you would post.
   # Run: nme r chat
   # Serve this folder first: python3 -m http.server 8000

   from urllib.request import urlopen
   from json import loads, dumps

   url = "http://localhost:8000/messages.json"
   messages = loads(urlopen(url).read().decode("utf-8"))

   show "chat log:"
   for message in messages:
       show f"  {message['who']}: {message['text']}"

   reply = {"who": "you", "text": "nice to meet you"}
   payload = dumps(reply)
   show ""
   show "your reply would be POSTed as:"
   show payload
   ```

   Run it with the server running:

   ```sh
   nme r chat
   ```

   ```text
   chat log:
     Mina: hello
     Jun: hi Mina
     Mina: ready to code?
   your reply would be POSTed as:
   {"who": "you", "text": "nice to meet you"}
   ```

5. Sending the reply needs `urllib.request.Request(url, data=..., method="POST")`. A plain `http.server` does not accept POST, so the guide prints the
   payload instead of pretending to send it. A real chat server would accept
   the POST and append the message — the data format here is exactly that.

## Try it yourself

Add a second message to `messages.json` and run the chat again — the log grows.
Change the reply's `text` and watch the payload change.

## What you learned

- `urlopen(url).read().decode("utf-8")` fetches a page as text.
- `loads(...)` turns JSON text into a Python list of dicts.
- `message["who"]` reads a field from one dict.
- A reply is a dict; sending it as a POST needs a server that accepts it.
