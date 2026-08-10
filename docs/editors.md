# VS Code, Cursor, and Zed

English | [한국어](editors.ko.md)

The repository includes ready-to-use run, check, and build tasks for all three
editors. Install NME first, then open the repository folder rather than a
single file.

## VS Code

The tracked `.vscode/settings.json` associates `*.nme` with Python for basic
highlighting. `.vscode/tasks.json` provides:

- `NME: run current file`
- `NME: check current file`
- `NME: build current file`

Open the Command Palette, choose **Tasks: Run Task**, and select one. VS Code
stores folder tasks and settings in `.vscode`, as described in its official
[workspace documentation](https://code.visualstudio.com/docs/editing/workspaces/workspaces)
and [settings guide](https://code.visualstudio.com/docs/configure/settings).

Because the editor sees Python while NME adds extra syntax, a Python extension
may underline valid beginner or sentence lines. `nme check` is authoritative.
An NME language server is not shipped in this beta.

## Cursor

Open the same folder in Cursor. Use the included VS Code-compatible settings
and tasks, or run `nme run`, `nme check`, and `nme build` in the integrated
terminal.

For Cursor Agent, paste the handoff prompt from
[AI coding assistants](ai-assistants.md). Cursor can attach a web link directly
with `@Link`; its official [context guide](https://docs.cursor.com/context/%40-symbols/overview)
documents link and file context.

Cursor project rules normally live in `.cursor/rules`, according to the
official [Cursor Rules guide](https://docs.cursor.com/context/rules). NME does
not require or track a rule file: the shared handoff URL is enough and avoids
placing tool-specific metadata in an NME project.

## Zed

The tracked `.zed/settings.json` associates `*.nme` with Python, and
`.zed/tasks.json` defines the same three tasks. Open the task picker with
`Cmd+Shift+R` on macOS or `Ctrl+Shift+R` on Linux/Windows and choose an NME
task.

Zed documents custom file associations in
[Configuring Languages](https://zed.dev/docs/configuring-languages) and local
`.zed/tasks.json` files in its [Tasks guide](https://zed.dev/docs/tasks).

As with VS Code, Python diagnostics do not understand sentence syntax; trust
`nme check` until a dedicated NME extension exists.

## Use the terminal in any editor

These commands work even if an editor does not import the provided tasks:

```sh
nme run path/to/program.nme
nme check path/to/program.nme
nme build path/to/program.nme -o program.py
nme compile path/to/program.nme -o program
```

On Windows, add `--python py` to `run` or `compile` if `python3` is not your
Python command.
