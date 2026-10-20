# Handoff: wireframe split out of drawtool (2026-10-20)

## What this is

`wireframe` draws UI screen mockups as Unicode box art in the terminal, from an indented
outline on stdin (`wireframe Title width=N`, then `header:`, `input:`, `button:`, `card X:`,
`row:`, `- item | right`, ...). Like drawtool, it is **agent-only**: its user is an agent such
as Claude calling it through a quoted heredoc, not a person at a terminal.

It used to be part of drawtool (`~/Development/drawtool`, public at github.com/saklani/drawtool,
MIT). On 2026-10-20 the user decided to make it a **completely separate tool** ("keep wireframe
completely separate tool and see if we extend it"): its own repo, its own copy of the drawing
code, no shared engine or dependency on drawtool.

## State now (commit ade9b84, local only)

- `src/wireframe.rs`: the parser and renderer, moved from drawtool's `src/wireframe/mod.rs`
  unchanged except the entry points: `parse`, `render`, `render_fit`.
- `src/paint.rs`, `src/text.rs`: copies of drawtool's character-grid painter and text helpers.
  `paint.rs` got its own `Style` enum (drawtool's lives in its flowchart model).
- `src/lib.rs`: modules, re-exports, `Error`. `src/main.rs`: the stdin CLI, `-w N` budget,
  `--help` with the element list.
- `tests/wireframe.rs`: drawtool's wireframe tests (16), all passing; `examples/add-money.txt`.
- `cargo test` passes. `cargo build` gives 4 dead-code warnings, all in `paint.rs` (see below).
- **Not on GitHub.** No remote, no CI, no Claude Code plugin yet.

## Where to pick up

1. **Prune the painter copy** (left mid-way). Wireframes only use solid light and heavy lines,
   so these are dead: `Style::Dotted`/`Thick` (then the `style` field and `Style` itself),
   `Weight::Double` and the `DOUBLE*` glyph tables, `Stroke::DOTTED`, `Stroke::light`,
   `Painter::clear`, `Painter::is_free`, and probably `mark_crossings` (flowchart-only `╂`).
   Painter unit tests that exercise those go too. Check `cargo build` warnings reach zero.
2. **Ask the user before anything outward-facing:** creating the GitHub repo (name, likely
   `saklani/wireframe`, public, MIT like drawtool) and pushing.
3. Then, mirroring drawtool (copy its files and adapt):
   - `.github/workflows/binaries.yml`: builds 4 targets and puts them on a release named
     `v<Cargo.toml version>`, marked latest.
   - `plugin/`: `.claude-plugin/plugin.json` (no `version` field), `bin/wireframe` (downloads
     the release binary on first use), a skill, and the display mod `hooks/register.tsx`
     (matches commands starting with the tool's name).
   - `.claude-plugin/marketplace.json`, or list this plugin in drawtool's marketplace.
4. A README: the wireframe section of drawtool's README (as of 3584ee0) is the starting point.
5. Possibly a `clippy` test like drawtool's `tests/clippy.rs`, and a help-examples test.

## Conventions carried over from drawtool

- Heavy cargo jobs run under `lockf /tmp/heavy-jobs.lock` (other sessions build alongside).
- Commit or push only when the user asks; confirm before creating repos or publishing.
- drawtool has a 2,000-line budget for `src` (code lines, no tests/comments/blank lines);
  the user said each tool gets its own budget. wireframe is about 400 lines.
- Keep the user's internal app names out of public examples (the example's header was
  changed from an internal app name to "Wallet").
