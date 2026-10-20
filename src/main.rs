use clap::Parser;
use std::io::{IsTerminal, Read};
use std::process::ExitCode;

const SYNTAX: &str = "\
The wireframe comes on stdin, usually a quoted heredoc: wireframe <<'EOF' ... EOF

Each screen starts with `wireframe [title] [width=N]`; its elements follow, one per
line, and `card Title:`, `row:` and `col:` take indented children. Several screens
are drawn side by side, wrapping to the column budget.
  wireframe Sign in width=40
    header: ← Back | Sign in | ?
    input: Email = you@example.com
    row: [ Google ] | [ Apple ]
    button: Continue

Elements:
  header: left | centre | right     top bar with a divider under it
  text: / center: / right: / plain  text, wrapped
  input: Label = placeholder        text field (select: for a dropdown)
  button: Label                     full-width button; `[ Label ]` in text is a small one
  image: caption                    image placeholder
  card Title:  row:  col:           bordered group / side-by-side columns / stack
  row: a | b | c                    inline columns
  tabs: *A | B   nav: *Home | Pay   tab bar / bottom navigation, `*` = active
  toggle: Wi-Fi = on   checkbox: x = on   radio: *a | b
  - item | right                    list item, optional right-aligned part
  ---   spacer                      divider, blank line
";

/// Draw UI wireframes as screen mockups in the terminal.
#[derive(Parser)]
#[command(version, after_help = SYNTAX)]
struct Args {
    /// Column budget (default: $COLUMNS, else the terminal's width, else 120; 0 = unlimited).
    /// Screens that don't fit side by side wrap to a new row.
    #[arg(short = 'w', long, value_name = "N")]
    max_width: Option<usize>,
}

/// `$COLUMNS`, else the width of whichever of stdout/stderr is a terminal
/// (so `wireframe | less` still fits the window), else 120.
fn default_width() -> usize {
    let tty = || {
        terminal_size::terminal_size_of(std::io::stdout())
            .or_else(|| terminal_size::terminal_size_of(std::io::stderr()))
    };
    std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse().ok())
        .or_else(|| tty().map(|(w, _)| w.0 as usize))
        .unwrap_or(120)
}

/// The wireframe source, from stdin (a heredoc).
fn read_stdin() -> Result<String, String> {
    let mut stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Err("no input: pipe the wireframe in on stdin (see --help)".to_string());
    }
    let mut t = String::new();
    stdin.read_to_string(&mut t).map_err(|e| format!("reading stdin: {e}"))?;
    if t.trim().is_empty() {
        return Err("no wireframe on stdin".to_string());
    }
    Ok(t)
}

fn run(max_width: usize) -> Result<wireframe::Fit, Box<dyn std::error::Error>> {
    Ok(wireframe::render_fit(&wireframe::parse(&read_stdin()?)?, max_width))
}

fn main() -> ExitCode {
    let args = Args::parse();
    let budget = args.max_width.unwrap_or_else(default_width);
    match run(budget) {
        Ok(fit) => {
            if !fit.fits {
                eprintln!("wireframe: note: {} columns wide, over the {budget}-column budget", fit.width);
            }
            println!("{}", fit.text);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("wireframe: {e}");
            ExitCode::FAILURE
        }
    }
}
