use ripgross::Style::*;

fn main() {
    test1();
    test2();
    test3();
    return;
}

fn test2() {
    println!(
        "This is a {}",
        Style::new()
            .ignore_space()
            .italic()
            .foreground_rgb(81, 105, 151)
            .underline()
            .blink()
            .render("decorated line !")
    )
}
fn test1() {
    println!(
        "{}",
        dbg!(Style::new()
            .ignore_space()
            .italic()
            .strikethrough()
            .underline()
            .blink()
            .foreground_rgb(81, 105, 151)
        .render_to_block("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque ac lectus non magna aliquam aliquet. Sed mattis est convallis commodo finibus. Pellentesque orci justo, dictum eget justo nec, condimentum porttitor tellus. Morbi suscipit sodales accumsan. Cras blandit, ligula ut efficitur luctus, enim est pretium mauris, et sagittis dolor magna quis augue. Nunc dictum imperdiet tortor, ac feugiat diam viverra et. Aenean pharetra, mi in congue maximus, ipsum justo ornare arcu, a ultrices enim erat eu eros. Donec sit amet laoreet arcu, tempor ultrices ante. Donec semper mi eu nisi ullamcorper, a consectetur ex ullamcorper. Fusce imperdiet libero malesuada, vestibulum erat consectetur, dignissim dolor. Cras sit amet lacus ex.

Sed est ex, volutpat id sapien sed, pretium ornare tellus. Quisque laoreet, nulla non bibendum eleifend, ex augue ultrices nibh, eget commodo risus orci non enim. Maecenas congue nec velit nec cursus. Donec a dapibus est. Praesent tristique dui id orci convallis bibendum. Donec dolor erat, tempor sed dolor eget, ultrices eleifend felis. Duis a magna aliquet, pharetra tellus eu, tincidunt orci.")
            .padding_top(4)
            .padding_left(8)
            .padding_right(8)
            .padding_bottom(4)
            .margin_top(3)
            .margin_right(4)
            .margin_left(5)
            .margin_bottom(6)
            .border_top()
            .border_right()
            .border_bottom()
            .align_end())
                .join_bottom(Style::new()
            .underline()
                .bold()
            .foreground_rgb(181, 105, 51)
                .render_to_block("
Phasellus fringilla bibendum condimentum. Cras eros quam, viverra nec finibus ac, aliquet eu purus. Cras non fermentum lorem. Phasellus viverra eros mi, eu vestibulum nisi aliquam ac. Morbi pharetra urna ut cursus sodales. In dapibus enim id laoreet tincidunt. Mauris et ante velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce lobortis mauris interdum laoreet ultricies. Nulla a eros convallis, auctor nisi non, lobortis neque. Nullam a dui a quam tempor congue. Vestibulum sit amet nisl lacus. Nulla a rhoncus neque.
")
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
            .align_start())
            .finalize()
    );
}
fn test3() {
    println!("{}{}",Style::new()
        .blink()
        .reverse()
        .render_to_block("Use securing confined his shutters. Delightful as he it acceptance an solicitude discretion reasonably. Carriage we husbands advanced an perceive greatest. Totally dearest expense on demesne ye he. Curiosity excellent commanded in me. Unpleasing impression themselves to at assistance acceptance my or. On consider laughter civility offended oh.").finalize()
        ,
        Style::new()
        .underline()
        .bold()
        .strikethrough()
        .render("She exposed painted fifteen are noisier mistake led waiting. Surprise not wandered speedily husbands although yet end. Are court tiled cease young built fat one man taken. We highest ye friends is exposed equally in. Ignorant had too strictly followed. Astonished as travelling assistance or unreserved oh pianoforte ye. Five with seen put need tore add neat. Bringing it is he returned received raptures.")
    )
}
