# wireframe

Draw UI screen mockups in the terminal, from an indented outline of elements. Several screens
are drawn side by side, to show a flow or before/after states.

```
┌─ Add money ──────────────────────────┐   ┌─ Home ───────────────────────────────┐
│ ← Back       Add money             ? │   │ ☰             Wallet           Help │
├──────────────────────────────────────┤   ├──────────────────────────────────────┤
│                                      │   │ ┌─ Balance ────────────────────────┐ │
│ Amount ($)                           │   │ │            $1,240.00             │ │
│ ┌──────────────────────────────────┐ │   │ │ [ Add money ]           [ Send ] │ │
│ │ 0.00                             │ │   │ └──────────────────────────────────┘ │
│ └──────────────────────────────────┘ │   │   Activity     Cards      Rewards    │
│ [ $20 ]        [ $50 ]      [ $100 ] │   │ ──━━━━━━━━────────────────────────── │
│                                      │   │ • Coffee at Blue Bottle       -$4.50 │
│ (•) Card   ( ) Bank                  │   │ • Salary from Acme        +$3,200.00 │
│ Card                                 │   │ Round-up savings               (  ●) │
│ ┌──────────────────────────────────┐ │   ├──────────────────────────────────────┤
│ │ Visa •••• 4242                 ▾ │ │   │     Home        Pay       Profile    │
│ └──────────────────────────────────┘ │   │     ━━━━                             │
│ [x] Save for next time               │   └──────────────────────────────────────┘
│                                      │
│ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ │
│ ┃             Continue             ┃ │
│ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ │
└──────────────────────────────────────┘
```

```
wireframe Add money
  header: ← Back | Add money | ?
  spacer
  input: Amount ($) = 0.00
  row: [ $20 ] | [ $50 ] | [ $100 ]
  spacer
  radio: *Card | Bank
  select: Card = Visa •••• 4242
  checkbox: Save for next time = on
  spacer
  button: Continue

wireframe Home
  header: ☰ | Wallet | Help
  card Balance:
    center: $1,240.00
    row: [ Add money ] | [ Send ]
  tabs: *Activity | Cards | Rewards
  - Coffee at Blue Bottle | -$4.50
  - Salary from Acme | +$3,200.00
  toggle: Round-up savings = on
  nav: *Home | Pay | Profile
```

## Install

```sh
cargo install --git https://github.com/saklani/wireframe
```

## Usage

```sh
wireframe <<'EOF'
wireframe Sign in width=40
  header: ← Back | Sign in | ?
  input: Email = you@example.com
  button: Continue
EOF
```

wireframe is made for agents: the outline always comes on stdin, as a quoted heredoc so labels
with `'`, `"`, `$` or backticks pass through untouched.

Each screen starts with `wireframe [Title] [width=N]` (width defaults to 40, at least 20). Its
elements follow, one per line; `card`, `row` and `col` take indented children. `-w N` sets the
column budget (default `$COLUMNS`, else the terminal width, else 120; `0` = unlimited): screens
that don't fit side by side wrap to a new row, and if they're still too wide wireframe prints a
`note:` on stderr. Errors name the line (and column) they're on.

| Element | Draws |
|---|---|
| `header: left \| centre \| right` | top bar with a divider under it |
| `text:` / `center:` / `right:` / plain line | text (wrapped) |
| `input: Label = placeholder`, `select: Label = value` | text field, dropdown |
| `button: Label` | full-width primary button; `[ Label ]` inline in text is a small one |
| `image: caption` | image placeholder |
| `card Title:` / `row:` / `col:` | bordered group / side-by-side columns / stack |
| `row: a \| b \| c` | inline columns (first left, last right, middle centred) |
| `tabs: *A \| B`, `nav: *Home \| Pay` | tab bar / bottom navigation, `*` = active |
| `toggle: Label = on`, `checkbox: Label = on`, `radio: *A \| B` | controls |
| `- item \| right` | list item, optional right-aligned part |
| `---`, `spacer` | divider, blank line |

## Claude Code plugin

The `wireframe` plugin (in `plugin/`) has three parts:

- wireframe itself: on first use it downloads the prebuilt binary for your platform (macOS or
  Linux, arm64 or x86-64) from the [latest release](https://github.com/saklani/wireframe/releases/latest),
  which CI rebuilds on every push that changes the crate. A `wireframe` already on your PATH wins.
- a skill that teaches Claude Code to sketch a mockup with wireframe when you ask what a screen
  looks like, or when one would explain UI code faster than prose
- a display mod that shows each mockup in full where Claude ran it, under a short
  `wireframe (Add money, Home)` row, so Claude doesn't retype it into its reply

No separate install is needed. Type at a Claude Code prompt:

```
/plugin install wireframe --marketplace saklani/wireframe
```

Answer `y` to add the marketplace, then pick a scope. The mod needs Claude Code 2.1.287 or later.
On an older version the skill still works; ask Claude to paste the mockup if you can't see it.

The mod has its own tests: `claude plugin test plugin`.

## Tests

```sh
cargo test    # rendering tests, plus a clippy gate
```

## License

MIT
