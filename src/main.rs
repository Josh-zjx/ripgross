use nu_ansi_term::Color::Red;
use ripgross::Style;
use std;

fn main() {
    let style = Style::new();
    println!("{}", dbg!(Red.paint("Reference Text")));
    dbg!(Red.paint("Hello").to_string());
    println!(
        "{}",
        dbg!(style
            .italic()
            .strikethrough()
            .underline()
            .foreground_rgb(81, 105, 151)
            .render("Test Text"))
    );
    return;
}
