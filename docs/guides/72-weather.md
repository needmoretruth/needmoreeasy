# 72 — Weather: reading JSON from the web

English | [한국어](72-weather.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [71 — HTTP](71-http.md), [58 — Data](58-data.md)
- Topic: the internet
- Result: fetching a local HTTP server's JSON and printing a mini weather report

Guide [71](71-http.md) fetched a text file over HTTP; [58](58-data.md) loaded JSON
from disk. This guide fetches JSON over HTTP, parses it, and prints a mini weather
report.

## Steps

1. Create `weather.json` — a dict holding another dict (`current`) and a
   list of dicts (`forecast`):

   ```nme
   {
     "city": "Seoul",
     "current": {"temperature": 21, "condition": "sunny", "humidity": 55},
     "forecast": [
       {"day": "Mon", "temperature": 22},
       {"day": "Tue", "temperature": 19},
       {"day": "Wed", "temperature": 24}
     ]
   }
   ```

2. Serve the folder with Python's built-in HTTP server, in one terminal:

   ```sh
   python3 -m http.server 8000
   ```

   `weather.json` is now at `http://localhost:8000/weather.json`.

3. The whole program. Save `weather.nme`:

   ```nme
   # weather.nme — a mini weather report from a local web server.
   # Run: nme r weather — serve this folder first: python3 -m http.server 8000

   import urllib.request
   import json

   url = "http://localhost:8000/weather.json"
   with urllib.request.urlopen(url) as response:
       body = response.read().decode("utf-8")
   data = json.loads(body)

   city = data["city"]
   current = data["current"]
   forecast = data["forecast"]
   show f"Weather in {city}"
   show f"  now: {current['temperature']} C, {current['condition']}"
   show f"  humidity: {current['humidity']}%"
   show "forecast:"
   for day in forecast:
       show f"  {day['day']}: {day['temperature']} C"

   warmest = forecast[0]
   for day in forecast:
       if day["temperature"] > warmest["temperature"]:
           warmest = day
   total = 0
   for day in forecast:
       total = total + day["temperature"]
   average = total / len(forecast)
   show f"warmest day: {warmest['day']} at {warmest['temperature']} C"
   show f"average forecast temperature: {round(average, 1)} C"
   ```

   `urlopen` opens the connection; `.decode("utf-8")` turns the bytes into a
   string. `json.loads` parses it into a dict; the `for` loops find the warmest day.

4. With the server still running, run it:

   ```sh
   nme r weather
   ```

   ```text
   Weather in Seoul
     now: 21 C, sunny
     humidity: 55%
   forecast:
     Mon: 22 C
     Tue: 19 C
     Wed: 24 C
   warmest day: Wed at 24 C
   average forecast temperature: 21.7 C
   ```

5. Korean writes the same report with `말해`; the full program is in the [Korean guide](72-weather.ko.md).

## Try it yourself

Change `weather.json` to your own city and temperature, then rerun — the server
reads the file on every request. Add a `"wind"` number to `current` and print it.

## What you learned

- `python3 -m http.server 8000` serves JSON files just like text files.
- `json.loads(body)` turns JSON text into a dict or list.
- A JSON dict can hold dicts and lists, and NME picks them out the same way.
- A report loop and a `warmest` loop read the whole document step by step.
