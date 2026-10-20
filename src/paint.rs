//! The one place that turns geometry into characters. Renderers draw text,
//! symbols and lines (box borders are lines too); the painter derives every
//! corner, tee and junction from which directions meet in a cell.

use crate::text::char_width;
use std::collections::HashMap;

const U: u8 = 1;
const D: u8 = 2;
const L: u8 = 4;
const R: u8 = 8;
/// Right half of a double-width character.
const WIDE: char = '\0';

// Glyphs by the directions present in a cell (U = 1, D = 2, L = 4, R = 8);
// a lone direction is a line's end, drawn as the full line.
const LIGHT: &str = " │││─┘┐┤─└┌├─┴┬┼";
const HEAVY: &str = " ┃┃┃━┛┓┫━┗┏┣━┻┳╋";
// Mixed weights: the vertical arms' weight, then the horizontal arms'.
const LIGHT_HEAVY: &str = " │││━┙┑┥━┕┍┝━┷┯┿";
const HEAVY_LIGHT: &str = " ┃┃┃─┚┒┨─┖┎┠─┸┰╂";

pub type Pt = (i64, i64);

/// How a line is drawn.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Stroke {
    Light,
    Heavy,
}

/// One net's line directions in a cell. Lines of the same net join; lines of
/// different nets that pass straight through each other cross.
#[derive(Clone, Copy)]
struct Arm {
    net: usize,
    bits: u8,
    stroke: Stroke,
}

#[derive(Default)]
pub struct Painter {
    /// Text and symbols.
    glyphs: HashMap<Pt, char>,
    lines: HashMap<Pt, Vec<Arm>>,
    nets: usize,
}

impl Painter {
    #[must_use]
    pub fn new() -> Self {
        Painter::default()
    }

    /// A fresh net id.
    pub fn net(&mut self) -> usize {
        self.nets += 1;
        self.nets
    }

    pub fn text(&mut self, mut x: i64, y: i64, s: &str) {
        for ch in s.chars() {
            self.glyphs.insert((x, y), ch);
            if char_width(ch) == 2 {
                self.glyphs.insert((x + 1, y), WIDE);
            }
            x += char_width(ch);
        }
    }

    pub fn symbol(&mut self, x: i64, y: i64, ch: char) {
        self.glyphs.insert((x, y), ch);
    }

    /// An axis-aligned polyline through `pts`.
    pub fn line(&mut self, pts: &[Pt], net: usize, stroke: Stroke) {
        for w in pts.windows(2) {
            let ((x0, y0), (x1, y1)) = (w[0], w[1]);
            let vertical = x0 == x1;
            let (lo, hi) = if vertical { (y0.min(y1), y0.max(y1)) } else { (x0.min(x1), x0.max(x1)) };
            let (back, fwd) = if vertical { (U, D) } else { (L, R) };
            for t in lo..=hi {
                let bits = if t > lo { back } else { 0 } | if t < hi { fwd } else { 0 };
                let cell = if vertical { (x0, t) } else { (t, y0) };
                let arms = self.lines.entry(cell).or_default();
                match arms.iter_mut().find(|a| a.net == net) {
                    Some(a) => a.bits |= bits,
                    None => arms.push(Arm { net, bits, stroke }),
                }
            }
        }
    }

    /// A square-cornered rectangle `w`×`h` with its top-left at (`x`, `y`).
    pub fn rect(&mut self, (x, y): Pt, (w, h): Pt, stroke: Stroke) {
        let (x1, y1) = (x + w - 1, y + h - 1);
        let net = self.net();
        self.line(&[(x, y), (x1, y), (x1, y1), (x, y1), (x, y)], net, stroke);
    }

    /// Top-left and bottom-right of everything drawn.
    pub(crate) fn bounds(&self) -> Option<(Pt, Pt)> {
        let cells = self.glyphs.keys().chain(self.lines.keys());
        let (xs, ys): (Vec<i64>, Vec<i64>) = cells.copied().unzip();
        Some(((*xs.iter().min()?, *ys.iter().min()?), (*xs.iter().max()?, *ys.iter().max()?)))
    }

    /// The character a cell shows: text or symbol, else line.
    #[must_use]
    pub fn cell(&self, pt: Pt) -> char {
        if let Some(&ch) = self.glyphs.get(&pt) {
            return ch;
        }
        self.lines.get(&pt).map_or(' ', |arms| resolve(arms))
    }

    /// Everything drawn so far, as trimmed lines (top-left at the smallest
    /// coordinates used).
    #[must_use]
    pub fn finish(&self) -> String {
        let Some(((x0, y0), (x1, y1))) = self.bounds() else { return String::new() };
        let row = |y| (x0..=x1).map(|x| self.cell((x, y))).filter(|&c| c != WIDE).collect::<String>();
        (y0..=y1).map(|y| row(y).trim_end().to_string()).collect::<Vec<_>>().join("\n")
    }
}

fn straight(stroke: Stroke, vertical: bool) -> char {
    let pair = match stroke {
        Stroke::Light => ['│', '─'],
        Stroke::Heavy => ['┃', '━'],
    };
    pair[usize::from(!vertical)]
}

/// The glyph where lines of one net meet, by the strokes of its vertical and
/// horizontal arms.
fn junction(bits: u8, vertical: Stroke, horizontal: Stroke) -> char {
    use Stroke::{Heavy, Light};
    let table = match (vertical, horizontal) {
        (Light, Light) => LIGHT,
        (Heavy, Heavy) => HEAVY,
        (Light, Heavy) => LIGHT_HEAVY,
        (Heavy, Light) => HEAVY_LIGHT,
    };
    table.chars().nth(usize::from(bits)).unwrap_or('┼')
}

fn resolve(arms: &[Arm]) -> char {
    let (vert, horz) = (U | D, L | R);
    let pure = arms.iter().all(|a| a.bits & vert == 0 || a.bits & horz == 0);
    let through = arms.iter().find(|a| a.bits == vert);
    if let Some(v) = through.filter(|_| pure && arms.iter().any(|a| a.bits == horz)) {
        // A crossing, not a join: the vertical line runs through.
        return straight(v.stroke, true);
    }
    let bits = arms.iter().fold(0, |b, a| b | a.bits);
    let stroke = |dirs: u8| arms.iter().filter(|a| a.bits & dirs != 0).map(|a| a.stroke).max();
    let heaviest = stroke(vert | horz).unwrap_or(Stroke::Light);
    match (stroke(vert), stroke(horz)) {
        (_, None) => straight(heaviest, true),
        (None, _) => straight(heaviest, false),
        (Some(v), Some(h)) => junction(bits, v, h),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn painted(draw: impl FnOnce(&mut Painter)) -> String {
        let mut p = Painter::new();
        draw(&mut p);
        p.finish()
    }

    #[test]
    fn an_edge_leaving_a_box_border_makes_a_tee() {
        let out = painted(|p| {
            p.rect((0, 0), (5, 3), Stroke::Light);
            let net = p.net();
            p.line(&[(2, 2), (2, 4)], net, Stroke::Light);
        });
        assert_eq!(out, "┌───┐\n│   │\n└─┬─┘\n  │\n  │");
    }

    #[test]
    fn heavy_corners() {
        assert!(painted(|p| p.rect((0, 0), (3, 3), Stroke::Heavy)).starts_with('┏'));
    }

    #[test]
    fn a_light_edge_on_a_heavy_border_is_a_mixed_tee() {
        let out = painted(|p| {
            p.rect((0, 0), (5, 3), Stroke::Heavy);
            let net = p.net();
            p.line(&[(2, 2), (2, 3)], net, Stroke::Light);
        });
        assert_eq!(out.lines().nth(2), Some("┗━┯━┛"));
    }

    #[test]
    fn crossing_vs_join() {
        let cross = |same_net| {
            painted(|p| {
                let (a, b) = (p.net(), p.net());
                p.line(&[(0, 1), (2, 1)], a, Stroke::Light);
                p.line(&[(1, 0), (1, 2)], if same_net { a } else { b }, Stroke::Light);
            })
        };
        assert_eq!(cross(true).lines().nth(1), Some("─┼─"), "one net: a join");
        assert_eq!(cross(false).lines().nth(1), Some("─│─"), "two nets: the vertical runs through");
    }

    #[test]
    fn wide_text_takes_two_cells_and_negative_coordinates_shift_to_zero() {
        let out = painted(|p| {
            p.text(-3, -1, "数据");
            p.symbol(1, -1, '!');
        });
        assert_eq!(out, "数据!");
    }
}
