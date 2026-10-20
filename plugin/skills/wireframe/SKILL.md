---
name: wireframe
description: Sketch a UI screen mockup in the terminal. Use when the user asks what a screen, page, form, dialog or flow of screens looks like or should look like ("mock up the sign-in page", "what would this settings screen look like", "sketch the checkout flow", "show the layout of this component"), when reading UI code (a component tree, a view, a template) and a picture of the screen would explain it faster than prose, or when comparing layouts or before/after states.
---

# wireframe: terminal UI mockups

`wireframe` turns an indented outline of UI elements into a Unicode screen mockup and shows it to
the user.

```bash
wireframe <<'EOF'
wireframe Sign in width=40
  header: ← Back | Sign in | ?
  input: Email = you@example.com
  input: Password = ••••••••
  row: [ Google ] | [ Apple ]
  button: Continue
EOF
```

**The user sees the mockup where the Bash call ran.** This plugin's display mod draws a
`wireframe` call's output in full in the transcript, under a short `wireframe (Sign in)` row.
Don't copy the mockup into your reply: refer to it and explain it in a few sentences. Run each
mockup as its own Bash command that starts with `wireframe` (no pipes, no `&&` chains), so the
mod knows the call. If the user says they can't see it (Claude Code before 2.1.287 doesn't load
mods), copy it verbatim into a plain ``` code block from then on.

## Input

Each screen starts with `wireframe [Title] [width=N]` (width defaults to 40). Elements follow,
one per line, indented under it; `card Title:`, `row:` and `col:` take further-indented children.
Several `wireframe` blocks are drawn side by side, wrapping to the column budget: use that for a
flow (step 1, step 2, ...) or before/after states.

| Write | Draws |
|---|---|
| `header: left \| centre \| right` | top bar with a divider under it |
| `text:` / `center:` / `right:` / a plain line | text, wrapped |
| `input: Label = placeholder`, `select: Label = value` | text field, dropdown |
| `button: Label` | full-width primary button; `[ Label ]` inline in text is a small one |
| `image: caption` | image placeholder |
| `card Title:` / `row:` / `col:` | bordered group / side-by-side columns / stack |
| `row: a \| b \| c` | inline columns (first left, last right, middle centred) |
| `tabs: *A \| B`, `nav: *Home \| Pay` | tab bar / bottom navigation, `*` = active |
| `toggle: Label = on`, `checkbox: Label = on`, `radio: *A \| B` | controls |
| `- item \| right` | list item, optional right-aligned part |
| `---`, `spacer` | divider, blank line |

`wireframe --help` prints the same list. Flag: `-w N` column budget (default `$COLUMNS`, else
the terminal width, else 120; `0` = unlimited). If the screens still don't fit, wireframe prints
a `note:` on stderr. Never paste that note into the reply.

This plugin puts `wireframe` on PATH (it downloads the binary on first use). If that fails, it
says so on stderr; then use `~/.cargo/bin/wireframe`, or build it with
`cargo install --git https://github.com/saklani/wireframe`.

## Drawing a good mockup

- Show the real screen, read from the code: the actual labels, fields, buttons and their order.
  Don't invent elements; mark a guess as one in your reply.
- Keep it to the parts that matter for the question. Collapse a long list to two or three items.
- For a flow, draw one small screen per step, side by side, in order.
- Keep widths modest (32 to 48) so two or three screens fit side by side.
- If wireframe reports an error, fix the line it names and rerun.
