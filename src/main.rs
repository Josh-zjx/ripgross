use nu_ansi_term::Color::Red;
use ripgross::Style;

fn main() {
    let style = Style::new();
    println!("{}", dbg!(Red.paint("Reference Text")));
    dbg!(Red.paint("Hello").to_string());
    println!(
        "{}",
        dbg!(style
            .ignore_space()
            .italic()
            .strikethrough()
            .underline()
            .blink()
            .foreground_rgb(81, 105, 151)
            .padding_top(4)
            .padding_left(8)
            .padding_right(8)
            .padding_bottom(4)
            .margin_top(3)
            .margin_right(4)
            .margin_left(5)
            .margin_bottom(6)
            .border_top()
            .border_left()
            .border_bottom()
            .align_end())
        .render("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque ac lectus non magna aliquam aliquet. Sed mattis est convallis commodo finibus. Pellentesque orci justo, dictum eget justo nec, condimentum porttitor tellus. Morbi suscipit sodales accumsan. Cras blandit, ligula ut efficitur luctus, enim est pretium mauris, et sagittis dolor magna quis augue. Nunc dictum imperdiet tortor, ac feugiat diam viverra et. Aenean pharetra, mi in congue maximus, ipsum justo ornare arcu, a ultrices enim erat eu eros. Donec sit amet laoreet arcu, tempor ultrices ante. Donec semper mi eu nisi ullamcorper, a consectetur ex ullamcorper. Fusce imperdiet libero malesuada, vestibulum erat consectetur, dignissim dolor. Cras sit amet lacus ex.

Phasellus fringilla bibendum condimentum. Cras eros quam, viverra nec finibus ac, aliquet eu purus. Cras non fermentum lorem. Phasellus viverra eros mi, eu vestibulum nisi aliquam ac. Morbi pharetra urna ut cursus sodales. In dapibus enim id laoreet tincidunt. Mauris et ante velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce lobortis mauris interdum laoreet ultricies. Nulla a eros convallis, auctor nisi non, lobortis neque. Nullam a dui a quam tempor congue. Vestibulum sit amet nisl lacus. Nulla a rhoncus neque.

Sed est ex, volutpat id sapien sed, pretium ornare tellus. Quisque laoreet, nulla non bibendum eleifend, ex augue ultrices nibh, eget commodo risus orci non enim. Maecenas congue nec velit nec cursus. Donec a dapibus est. Praesent tristique dui id orci convallis bibendum. Donec dolor erat, tempor sed dolor eget, ultrices eleifend felis. Duis a magna aliquet, pharetra tellus eu, tincidunt orci.")
    );
    return;
}
