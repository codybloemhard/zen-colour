use zen_colour::*;

fn main() {
    let ns = ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];
    let cs = [BLACK, RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN, WHITE];
    let bs = [BG_BLACK, BG_RED, BG_GREEN, BG_YELLOW, BG_BLUE, BG_MAGENTA, BG_CYAN, BG_WHITE];
    let ss = [RESET, BOLD, FAINT, ITALIC, UNDERLINED, BLINK, HIDDEN, CROSSED];

    for style in ss {
        print!("{}", RESET);
        for (i, colour) in cs.iter().enumerate() {
            println!("{}{} this is {}", style, colour, ns[i]);
        }
    }

    print!("{}", RESET);
    for (i, bg_colour) in bs.iter().enumerate() {
        println!(
            "{}{}{}{}{}{}{} this is {} {}",
            bg_colour, BLACK, BOLD, ITALIC, UNDERLINED, CROSSED, BLINK, ns[i], RESET
        );
    }

    println!("{}\nfin", RESET);
}
