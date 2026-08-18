# 74 — Network: downloading files

English | [한국어](74-download.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [72 — Weather](72-weather.md), [37 — Files](37-files.md)
- Topic: network & files
- Result: downloading a file from a local HTTP server and saving it while showing progress

Guide [76](76-net.md) fetches data and shows it; guide [73](73-poll.md) fetched
it again and again. This guide fetches a file once and **saves it** — the
whole point of a downloader. Reading a web page and downloading a file are
the same call; the only new ideas are reading in chunks and writing to disk.

## Steps

1. Create a file to download. `story.txt` is the file you would normally
   fetch from a real website:

   ```text
   Once upon a time, a learner opened a terminal.
   The terminal asked for one small program at a time.
   One day the learner wrote a downloader,
   and the file arrived safely on disk.
   ```

2. Serve the folder with Python's built-in HTTP server, as in guide
   [76](76-net.md):

   ```sh
   python3 -m http.server 8000
   ```

3. `urlopen` gives you a response object. Its `.read()` call returns the
   whole body at once:

   ```nme
   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   data = urlopen(url).read()
   text = data.decode("utf-8")
   ```

   The response is raw bytes; `.decode("utf-8")` turns them into text, the
   same call that decoded JSON in guide [72](72-weather.md).

4. Saving the text is one `file_write` call — guide [37](37-files.md).
   The full program downloads and saves. Save `download.nme`:

   ```nme
   # download.nme — fetch a file from the local server and save it.
   # Run: nme r download
   # Serve this folder first: python3 -m http.server 8000

   use file latest

   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   text = urlopen(url).read().decode("utf-8")
   show f"downloaded {len(text)} characters"
   file_write("story-copy.txt", text)
   show "saved as story-copy.txt"
   ```

5. Run it while the server is running:

   ```sh
   nme r download
   ```

   ```text
   downloaded 176 characters
   saved as story-copy.txt
   ```

   Open `story-copy.txt`: it is an exact copy of the original, written by
   your program — your first downloaded file.

6. A big download should not vanish into silence. The response can be read
   in chunks, and `Content-Length` (a header the server sends) tells you the
   total size. The chunks stay **raw bytes** until the end: a 64-byte slice
   can cut a Korean or emoji character in half, and decoding a half-cut
   character fails. Keep bytes whole, decode once. Save
   `download-progress.nme`:

   ```nme
   # download-progress.nme — download in chunks and report progress.
   # Run: nme r download-progress
   # Serve this folder first: python3 -m http.server 8000

   use file latest

   from urllib.request import urlopen

   url = "http://localhost:8000/story.txt"
   response = urlopen(url)
   size = int(response.headers["Content-Length"])
   show f"size: {size} bytes"

   chunks = []
   received = 0
   while True:
       chunk = response.read(64)
       if not chunk:
           break
       received = received + len(chunk)
       chunks.append(chunk)
       show f"received {received} / {size}"

   text = b"".join(chunks).decode("utf-8")
   file_write("story-copy.txt", text)
   show "saved"
   ```

   Reading 64 bytes at a time means the progress line updates as data
   arrives; `break` stops when an empty chunk says the file is over.
   `b"".join(chunks)` glues the byte pieces, and `.decode("utf-8")` runs
   once, on the complete file. The loop is the same pattern that polls a
   server in guide [73](73-poll.md), but here the server sends until it is
   done.

7. Run it:

   ```sh
   nme r download-progress
   ```

   ```text
   size: 176 bytes
   received 64 / 176
   received 128 / 176
   received 176 / 176
   saved
   ```

## Try it yourself

Add `content.txt` and `notes.txt` next to `story.txt` and make the program
download every file it finds — guide [48](48-files-folder.md) lists a
folder, and each file name becomes one `urlopen` call. Or download a page
that changes (like `status.json` in guide [73](73-poll.md)) and save one
snapshot per poll.

## What you learned

- A download is an `urlopen` call plus a `file_write` call.
- The response body is bytes; `.decode("utf-8")` makes text from it.
- `Content-Length` tells you the size before the data arrives.
- Reading in chunks lets the program report progress while it works.
- A chunk can cut a character in half — decode bytes only after joining
  them all.
- The same loop-and-`break` pattern that polls servers also receives files.
