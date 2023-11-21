//! adequate
//!
//! ```
//! use zen_colour::*;
//! println!("{}this is red{}", RED, RESET);
//! println!("{BLUE}blue{RESET}");
//! println!("{r}r{g}g{r}r{g}g{r}r{g}g", r = RED, g = GREEN);
//! ```

pub const RESET: &str = "\x1B[00m";
pub const BOLD: &str = "\x1B[01m";
pub const FAINT: &str = "\x1B[02m";
pub const ITALIC: &str = "\x1B[03m";
pub const UNDERLINED: &str = "\x1B[04m";
pub const BLINK: &str = "\x1B[05m";
pub const EFFECT6: &str = "\x1B[06m";
pub const EFFECT7: &str = "\x1B[07m";
pub const HIDDEN: &str = "\x1B[08m";
pub const CROSSED: &str = "\x1B[09m";

pub const BLACK: &str = "\x1B[30m";
pub const RED: &str = "\x1B[31m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";
pub const WHITE: &str = "\x1B[37m";
pub const DEFAULT: &str = "\x1B[39m";

pub const BG_BLACK: &str = "\x1B[40m";
pub const BG_RED: &str = "\x1B[41m";
pub const BG_GREEN: &str = "\x1B[42m";
pub const BG_YELLOW: &str = "\x1B[43m";
pub const BG_BLUE: &str = "\x1B[44m";
pub const BG_MAGENTA: &str = "\x1B[45m";
pub const BG_CYAN: &str = "\x1B[46m";
pub const BG_WHITE: &str = "\x1B[47m";
pub const BG_DEFAULT: &str = "\x1B[49m";

