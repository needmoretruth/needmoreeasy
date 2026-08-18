# 72 — Modules: a project across files

English | [한국어](72-project-files.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [67 — Grade book](67-grade-book.md), [23 — Modules](23-modules.md)
- Topic: modules & structure
- Result: a small weather-report project split into three .nme modules (fetch, analyze, report) with clear interfaces, imported by a main program

Guide [23](23-modules.md) split a helper out of a program, and guide
[67](67-grade-book.md) used one storage module. A real project is bigger than
one module: this guide builds a weather report from three modules — one that
loads data, one that analyzes it, and one that prints. Each module does one
job, and the main program imports only the names it needs.

## Steps

1. The project needs data first. Create `weather.json` — a list of one-day
   records, each with a `day` and a `temp` (temperature):

   ```text
   [
     {"day": "Mon", "temp": 21},
     {"day": "Tue", "temp": 25},
     {"day": "Wed", "temp": 18},
     {"day": "Thu", "temp": 24},
     {"day": "Fri", "temp": 27}
   ]
   ```

2. `fetch.nme` is the input module. Its only job is to read the file; the
   `use file` module from guide [14](14-json.md) does the reading:

   ```text
   # fetch.nme — loads the weather data.

   use file latest

   def load_weather():
       return json_load("weather.json")
   ```

   The module's interface is one name: `load_weather`.

3. `analyze.nme` is the brains. `average` adds the temperatures and divides,
   like guide [54](54-stats.md); `hottest` walks the days with an `if`,
   remembering the warmest one:

   ```text
   # analyze.nme — computes the average and the hottest day.

   def average(temps):
       total = 0
       for t in temps:
           total = total + t
       return total / len(temps)

   def hottest(days):
       best = days[0]
       for day in days:
           if day["temp"] > best["temp"]:
               best = day
       return best
   ```

4. `report.nme` is the output module. It knows how to print a report and
   nothing else — no file reading, no math:

   ```text
   # report.nme — prints the weather report.

   def print_report(days, avg, best):
       show f"Weather report: {len(days)} days"
       show f"Average: {avg:.1f}C"
       show f"Hottest: {best['day']} at {best['temp']}C"
   ```

5. `main.nme` ties the project together. The import lines list each module's
   interface — `from "fetch.nme" import load_weather`, then the two analyze
   functions, then the report function. The main program decides the order:
   load, gather temperatures, compute, print. Save it next to the three
   modules:

   ```text
   # main.nme — the weather report project.
   # Run: nme r main
   # weather.json must be in the same folder.

   from "fetch.nme" import load_weather
   from "analyze.nme" import average, hottest
   from "report.nme" import print_report

   days = load_weather()

   temps = []
   for day in days:
       temps.append(day["temp"])

   avg = average(temps)
   best = hottest(days)
   print_report(days, avg, best)
   ```

   `days` is the loaded list, `temps` the column of temperatures, `avg` the
   average, and `best` the warmest record — each value belongs to exactly one
   module, and main only moves values between the interfaces.

6. Run the main program with the data file present:

   ```sh
   nme r main
   ```

   ```text
   Weather report: 5 days
   Average: 23.0C
   Hottest: Fri at 27C
   ```

   The average of 21, 25, 18, 24, 27 is 23.0, and Friday at 27 is the hottest
   day.

7. Only imported names cross the module boundary. `json_load` lives inside
   `fetch.nme` because of its `use file latest`, but main never sees it — main
   can use only the names in its import lists. A module's own helpers stay
   private the same way: add a `def _celsius(f)` helper to `analyze.nme` and
   it will stay inside. The import list *is* the interface, so changing a
   module's internals can never break the program that imports it.

8. Korean writes the same project with `파일 사용 최신`, `json읽기`, `말해`, and
   `_ko` module names such as `fetch_ko.nme`. The four Korean files are in the
   [Korean guide](72-project-files.ko.md).

## Try it yourself

Add a `coldest(days)` function to `analyze.nme`, import it in `main.nme`, and
pass its result to `print_report` — the same walk as `hottest` but comparing
with `<`. Then give `report.nme` a header line that prints the city name.

## What you learned

- A project splits into modules by job: input, logic, output.
- Each module exports a small interface — the names in the import list.
- `from "fetch.nme" import load_weather` brings exactly one name across.
- The main program moves values between module interfaces and nothing else.
