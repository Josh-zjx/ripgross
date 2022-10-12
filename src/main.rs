use nu_ansi_term::Color::Red;
use ripgross::Style;

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
            .blink()
            .foreground_rgb(81, 105, 151)
            .padding_top(4)
            .padding_left(4)
            .render("Test Text"))
    );
    return;
}
