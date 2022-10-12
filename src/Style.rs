use nu_ansi_term;
use nu_ansi_term::Color;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Style {
    decorator_dict: HashSet<Decorator>,
    decorated_space: bool,
    width: u32,
    height: u32,
    horizontal_alignment: Alignment,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
    layout_dict: HashMap<Layout, u32>,
    //_border_dict: HashMap<Border, i32>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Decorator {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Reverse,
    Blink,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Alignment {
    Start,
    Center,
    End,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Layout {
    //MarginTop,
    //MarginLeft,
    //MarginBottom,
    //MarginRight,
    PaddingTop,
    PaddingLeft,
    PaddingBottom,
    PaddingRight,
}
/*
enum Border {
    borderStyleKey,
    borderTopKey,
    borderRightKey,
    borderBottomKey,
    borderLeftKey,
    borderTopForegroundKey,
    borderRightForegroundKey,
    borderBottomForegroundKey,
    borderLeftForegroundKey,
    borderTopBackgroundKey,
    borderRightBackgroundKey,
    borderBottomBackgroundKey,
    borderLeftBackgroundKey,
}
    */

/// Create a new Style object
///
/// With default width 80 and height 24
pub fn new() -> Style {
    return Style {
        decorator_dict: HashSet::new(),
        decorated_space: true,
        width: 80,
        height: 24,
        horizontal_alignment: Alignment::Start,
        background_color: None,
        foreground_color: None,
        layout_dict: HashMap::new(),
    };
}

impl Style {
    ///
    /// Use the style to render a &str
    ///
    /// The styler would wrap the text first
    ///
    /// render core text with setted decorator
    ///
    /// render padding alignment border and margin
    /// # Example
    ///
    /// ```
    /// style.underline()
    ///     .strikethrough()
    ///     .blink()
    ///     .padding_top(1)
    ///     .padding_bottom(2)
    ///     .render("Hello World")
    ///
    /// ```
    pub fn render(&mut self, raw_string: &str) -> String {
        let mut content = raw_string.to_string();
        content = self.apply_text_wrap(content);
        content = self.apply_text_style(content);
        content = self.apply_layout_style(content);
        return content;
    }

    /// Apply text wrap
    /// Would panic if valid text length is negative
    fn apply_text_wrap(&self, raw_content: String) -> String {
        let left_pad = match self.layout_dict.get(&Layout::PaddingLeft) {
            Some(pad) => pad.to_owned(),
            None => 0 as u32,
        };
        let right_pad = match self.layout_dict.get(&Layout::PaddingRight) {
            Some(pad) => pad.to_owned(),
            None => 0 as u32,
        };
        assert!(self.width > left_pad + right_pad);
        let wrap_length = (self.width - left_pad - right_pad) as usize;
        let mut new_string = String::new();
        for i in raw_content.split("\n") {
            let mut to_wrap = i.to_owned();
            while to_wrap.chars().count() > wrap_length {
                let next = to_wrap.split_off(wrap_length);
                to_wrap.push('\n');
                new_string.push_str(&to_wrap);
                to_wrap = next;
            }
        }

        return raw_content;
    }

    /// Apply Layout Decoration with following order:
    /// Padding
    /// Alignment
    /// Border
    /// Margin
    fn apply_layout_style(&self, mut raw_content: String) -> String {
        if !self.layout_dict.is_empty() {
            // Add Padding
            if self.layout_dict.contains_key(&Layout::PaddingLeft) {
                raw_content = self.pad_left(
                    raw_content,
                    self.layout_dict.get(&Layout::PaddingLeft).unwrap(),
                );
            }
            if self.layout_dict.contains_key(&Layout::PaddingRight) {
                raw_content = self.pad_right(
                    raw_content,
                    self.layout_dict.get(&Layout::PaddingRight).unwrap(),
                );
            }
            raw_content = self.align_horizontal(raw_content, &self.horizontal_alignment);
            if self.layout_dict.contains_key(&Layout::PaddingTop) {
                raw_content = self.pad_top(
                    raw_content,
                    self.layout_dict.get(&Layout::PaddingTop).unwrap(),
                );
            }
            if self.layout_dict.contains_key(&Layout::PaddingBottom) {
                raw_content = self.pad_bottom(
                    raw_content,
                    self.layout_dict.get(&Layout::PaddingBottom).unwrap(),
                );
            }
        }

        return raw_content;
    }

    /// Apply inline text decoration
    /// Mark decorated_space = true if you want space decorated
    fn apply_text_style(&self, raw_content: String) -> String {
        let mut handler = nu_ansi_term::Style::new();
        handler.background = self.background_color;
        handler.foreground = self.foreground_color;

        if self.decorated_space {
            for x in self.decorator_dict.iter() {
                match x {
                    Decorator::Bold => {
                        handler = handler.bold();
                    }
                    Decorator::Italic => {
                        handler = handler.italic();
                    }
                    Decorator::Underline => {
                        handler = handler.underline();
                    }
                    Decorator::Strikethrough => {
                        handler = handler.strikethrough();
                    }
                    Decorator::Blink => {
                        handler = handler.blink();
                    }
                    Decorator::Reverse => {
                        handler = handler.reverse();
                    }
                }
            }
            let result = handler.paint(raw_content).to_string();
            return result;
        }
        return raw_content;
    }
    fn pad_top(&self, raw_content: String, size: &u32) -> String {
        let mut padding_string = "\n".repeat(*size as usize);
        padding_string.push_str(&raw_content);

        return padding_string;
    }
    fn pad_bottom(&self, mut raw_content: String, size: &u32) -> String {
        let padding_string = "\n".repeat(*size as usize);
        raw_content.push_str(&padding_string);
        return raw_content;
    }
    fn pad_left(&self, raw_content: String, size: &u32) -> String {
        let padding_string = " ".repeat(*size as usize);
        let mut new_string = String::new();
        for i in raw_content.split("\n") {
            new_string.push_str(&padding_string);
            new_string.push_str(i);
            new_string.push('\n');
        }
        return new_string;
    }
    fn pad_right(&self, raw_content: String, size: &u32) -> String {
        let padding_string = " ".repeat(*size as usize);
        let mut new_string = String::new();
        for i in raw_content.split("\n") {
            new_string.push_str(i);
            new_string.push_str(&padding_string);
            new_string.push('\n');
        }
        return new_string;
    }
    fn align_horizontal(&self, raw_content: String, alignment: &Alignment) -> String {
        let mut new_string = String::new();
        match *alignment {
            Alignment::Start => {
                for i in raw_content.split("\n") {
                    new_string.push_str(i);
                    let pad = self.width as usize - i.chars().count();
                    if pad > 0 {
                        let padding_string = " ".repeat(pad);
                        new_string.push_str(&padding_string);
                    }
                    new_string.push('\n');
                }
            }
            Alignment::End => {
                for i in raw_content.split("\n") {
                    let pad = self.width as usize - i.chars().count();
                    if pad > 0 {
                        let padding_string = " ".repeat(pad);
                        new_string.push_str(&padding_string);
                    }
                    new_string.push_str(i);
                    new_string.push('\n');
                }
            }
            Alignment::Center => {
                for i in raw_content.split("\n") {
                    let double_pad = self.width as usize - i.chars().count();
                    let pad = double_pad / 2;
                    if double_pad > 0 {
                        let padding_string = " ".repeat(pad);
                        new_string.push_str(&padding_string);
                        new_string.push_str(i);
                        let padding_string = " ".repeat(double_pad - pad);
                        new_string.push_str(&padding_string);
                        new_string.push('\n');
                    } else {
                        new_string.push_str(i);
                        new_string.push('\n');
                    }
                }
            }
        }
        return raw_content;
    }
}
pub mod getter;
pub mod setter;
