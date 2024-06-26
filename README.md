# zen-colour

The most minimal terminal colour and style crate.
Contains constants with colour and style codes.
Just throw them into your string and bob's your uncle.

```rust
use zen_colour::*;
println!("{}this is red{}", RED, RESET);
println!("{BLUE}blue{RESET}");
println!("{r}r{g}g{r}r{g}g{r}r{g}g", r = RED, g = GREEN);
```

## constants

Styles:
- RESET
- BOLD
- FAINT
- ITALIC
- UNDERLINED
- BLINK
- EFFECT6
- EFFECT7
- HIDDEN
- CROSSED

Text colours:
- BLACK
- RED
- GREEN
- YELLOW
- BLUE
- MAGENTA
- CYAN
- WHITE
- DEFAULT

Background colours:
- BG_BLACK
- BG_RED
- BG_GREEN
- BG_YELLOW
- BG_BLUE
- BG_MAGENTA
- BG_CYAN
- BG_WHITE
- BG_DEFAULT

## names

If you don't like the constant names.

```rust
use zen_colours::*;
const R: &str = RED;
```

## non standard colours

The standard colours are chosen by the user.
They want to see those colours.
The non standard colours (RGB for example) are not chosen by the user.
They will just clash with the standard colours and your application will not look consistent
with other programs on the users system.
They must never be used unless you are doing some kind of graphics, as opposed to UI.
This is why this crate just does not support them.
This battle is unfortunately lost in the GUI space.
May the terminal stay strong.

## why

There are many crates that do the same.
With all kinds of handy functions.
There are different styles of library design.

```rust
println!("red".colour(red))
print_colour("red", red);
```

But in the end, it all annoyed me, especially when do doing more complex colouring.
For example, if you want to colour a date string but want all the '/' to be faint.
Then have more colours in the same line for different values.
This is just a mess or mega verbose with any library design no matter how reasonable.
I just ended up throwing the colour codes right into the `print!` or `format!`.
And now we are here.
Nice, easy, clean, no complexity, just colours and styles.

## more features?

no.

