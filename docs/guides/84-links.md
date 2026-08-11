# 84 — Web: extracting links from a page

English | [한국어](84-links.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [55 — Network](55-net.md), [69 — Patterns](69-patterns.md)
- 주제 (Topic): 웹/텍스트 / web & text
- 결과물 (Result): 로컬 서버의 HTML 페이지를 받아 그 안의 링크를 모두 찾아 절대 주소로 보여 주기 / fetching an HTML page from a local server and listing every link on it as a full URL

Guide [55](55-net.md) fetched JSON; guide [69](69-patterns.md) matched
text with regex. A web page is just text too — HTML with tags around the
content. Extracting the links is one regex over the raw page, and joining
them with the server's address makes the full URLs a browser would use.

## Steps

1. Create a small page with three links. Save `page.html`:

   ```text
   <html>
   <head><title>my page</title></head>
   <body>
   <h1>Links</h1>
   <a href="story.txt">the story</a>
   <a href="notes.txt">notes</a>
   <a href="about.html">about us</a>
   </body>
   </html>
   ```

2. Serve the folder, as in guide [55](55-net.md):

   ```sh
   python3 -m http.server 8000
   ```

3. Fetching the page is the same `urlopen` call that fetched JSON; a page
   is text, so `.decode("utf-8")` gives the HTML:

   ```text
   from urllib.request import urlopen
   import re

   base = "http://localhost:8000"
   html = urlopen(base + "/page.html").read().decode("utf-8")
   ```

4. Each link lives between `href="` and the next `"`. The regex
   `r'href="([^"]+)"'` finds every one — `[^"]+` means "one or more
   characters that are not a quote", and `findall` returns just the
   captured parts, as in guide [69](69-patterns.md). Save `links.nme`:

   ```text
   # links.nme — find the links on a web page.
   # Run: nme r links
   # Serve this folder first: python3 -m http.server 8000

   from urllib.request import urlopen
   import re

   base = "http://localhost:8000"
   html = urlopen(base + "/page.html").read().decode("utf-8")
   links = re.findall(r'href="([^"]+)"', html)

   show f"found {len(links)} links:"
   for link in links:
       show base + "/" + link
   ```

   The page only knows `story.txt` — a short name relative to itself.
   Prepending `base + "/"` turns it into a full URL a browser could open:
   a page on the same server links with short names, and the client
   completes them.

5. Run it:

   ```sh
   nme r links
   ```

   ```text
   found 3 links:
   http://localhost:8000/story.txt
   http://localhost:8000/notes.txt
   http://localhost:8000/about.html
   ```

   The three `href`s became three complete addresses. This is the same
   extraction step a link checker or a web crawler performs before
   fetching the next page.

## Try it yourself

Add a link to another page that itself has links, then make the program
fetch each found link and count how many of them exist (a
`404 Not Found` answer means a broken link — the HTTP error appears as
an exception, guide [68](68-errors.md) shows how to catch it). Or extract
the page's `src="..."` image sources with the same pattern.

## What you learned

- HTML is text: fetch it with `urlopen`, decode it, search it with regex.
- `r'href="([^"]+)"'` finds link targets; `[^"]+` stops at the closing quote.
- Relative links are short names; the client prepends the server address.
- Link extraction is one regex — the first step of a link checker.
