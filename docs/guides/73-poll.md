# 73 — Network: polling a server

English | [한국어](73-poll.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [72 — Weather](72-weather.md), [71 — HTTP](71-http.md)
- Topic: network & timing
- Result: repeatedly fetching a status.json from a local server every few seconds and reporting changes

Programs that watch a server keep asking it for the latest state. This is
called polling. The loop is simple: fetch, show, wait, repeat — and stop when
the server says it is done.

## Steps

1. Create `status.json` with a state that a worker will change:

   ```nme
   {"status": "working", "step": 1}
   ```

2. Serve the folder, as in guide [76](76-net.md):

   ```sh
   python3 -m http.server 8000
   ```

3. Fetching the status is the same `urlopen` call as guide [76](76-net.md):

   ```nme
   from urllib.request import urlopen
   from json import loads

   url = "http://localhost:8000/status.json"
   status = loads(urlopen(url).read().decode("utf-8"))
   ```

4. The full program polls in a loop, reports each state, and stops when the
   server reports `"done"`. Save `poll.nme`:

   ```nme
   # poll.nme — watch status.json until it says done.
   # Run: nme r poll
   # Serve this folder first: python3 -m http.server 8000
   # Change status.json to {"status": "done", "step": 2} while it runs.

   from urllib.request import urlopen
   from json import loads
   from time import sleep

   url = "http://localhost:8000/status.json"

   while True:
       status = loads(urlopen(url).read().decode("utf-8"))
       show f"step {status['step']}: {status['status']}"
       if status["status"] == "done":
           show "worker finished"
           break
       sleep(1)
   ```

   Each loop fetches the current file, prints it, and waits one second. To
   see the change, edit `status.json` (the HTTP server re-reads the file on
   every request) and watch the program report the new step before stopping.

5. Run it, then edit `status.json` while it runs:

   ```sh
   nme r poll
   ```

   ```text
   step 1: working
   step 2: done
   worker finished
   ```

## Try it yourself

Add a `started` timestamp to `status.json` and report how many seconds the
worker ran, or poll two endpoints and report when either one finishes.

## What you learned

- Polling means fetching the latest state in a loop with a wait between tries.
- The HTTP server re-reads the file on every request, so edits appear live.
- A sentinel value like `"done"` tells the loop when to stop.
- `time.sleep` controls how often the program asks.
