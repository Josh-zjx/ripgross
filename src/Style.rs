use nu_ansi_term;
use nu_ansi_term::Color;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Style {
    decorator_dict: HashSet<Decorator>,
    decorated_space: bool,
    width: usize,
    height: usize,
    horizontal_alignment: Alignment,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
    layout_dict: HashMap<Layout, usize>,
    border_dict: HashSet<Border>,
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
    MarginTop,
    MarginLeft,
    MarginBottom,
    MarginRight,
    PaddingTop,
    PaddingLeft,
    PaddingBottom,
    PaddingRight,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Border {
    //BorderStyle,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
    //BorderTopForeground,
    //BorderRightForeground,
    //BorderBottomForeground,
    //BorderLeftForeground,
    //BorderTopBackground,
    //BorderRightBackground,
    //BorderBottomBackground,
    //BorderLeftBackground,
}

/// Create a new Style object
///
/// With default width 80 and height 24
pub fn new() -> Style {
    return Style {
        decorator_dict: HashSet::new(),
        border_dict: HashSet::new(),
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
        let content = raw_string.to_string();
        let mut wrapped_strings = self.text_wrap(content);
        for i in wrapped_strings.iter_mut() {
            let text_length = i.chars().count();
            let decorated_line = self.line_text_decoration(i.clone());
            *i = self.line_layout(decorated_line, text_length);
        }
        if self.layout_dict.contains_key(&Layout::PaddingTop) {
            wrapped_strings = self.pad_top(
                wrapped_strings,
                self.layout_dict.get(&Layout::PaddingTop).unwrap(),
            );
        }
        if self.layout_dict.contains_key(&Layout::PaddingBottom) {
            wrapped_strings = self.pad_bottom(
                wrapped_strings,
                self.layout_dict.get(&Layout::PaddingBottom).unwrap(),
            );
        }
        wrapped_strings = self.draw_border(wrapped_strings);
        wrapped_strings = self.draw_margin(wrapped_strings);

        // Finalize output
        let mut content = String::new();
        for i in wrapped_strings {
            content.push_str(i.as_str());
            content.push('\n');
        }
        return content;
    }

    fn draw_margin(&self, mut raw_content: Vec<String>) -> Vec<String> {
        match self.layout_dict.get(&Layout::MarginLeft) {
            Some(size) => {
                for i in raw_content.iter_mut() {
                    *i = self.pad_left(i.clone(), size);
                }
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginTop) {
            Some(size) => {
                for i in raw_content.iter_mut() {
                    *i = self.pad_right(i.clone(), size);
                }
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginTop) {
            Some(size) => {
                raw_content = self.pad_top(raw_content, size);
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginBottom) {
            Some(size) => {
                raw_content = self.pad_bottom(raw_content, size);
            }
            None => (),
        };
        return raw_content;
    }
    fn draw_border(&self, mut raw_content: Vec<String>) -> Vec<String> {
        let top_left = self.border_dict.contains(&Border::BorderLeft)
            && self.border_dict.contains(&Border::BorderTop);
        let top_right = self.border_dict.contains(&Border::BorderRight)
            && self.border_dict.contains(&Border::BorderTop);
        let bottom_left = self.border_dict.contains(&Border::BorderLeft)
            && self.border_dict.contains(&Border::BorderBottom);
        let bottom_right = self.border_dict.contains(&Border::BorderRight)
            && self.border_dict.contains(&Border::BorderBottom);
        for i in raw_content.iter_mut() {
            if self.border_dict.contains(&Border::BorderLeft) {
                i.insert(0, '│');
            }
            if self.border_dict.contains(&Border::BorderRight) {
                i.push('│');
            }
        }
        if self.border_dict.contains(&Border::BorderTop) {
            let mut borderline = "─".repeat(self.width);
            if top_left {
                borderline.insert(0, '┌');
            }
            if top_right {
                borderline.push('┐');
            }
            raw_content.insert(0, borderline);
        }
        if self.border_dict.contains(&Border::BorderBottom) {
            let mut borderline = "─".repeat(self.width);
            if bottom_left {
                borderline.insert(0, '└');
            }
            if bottom_right {
                borderline.push('┘');
            }
            raw_content.push(borderline);
        }
        return raw_content;
    }
    /// Apply text wrap
    /// Would panic if valid text length is negative
    fn text_wrap(&self, raw_content: String) -> Vec<String> {
        let left_pad = match self.layout_dict.get(&Layout::PaddingLeft) {
            Some(pad) => pad.to_owned(),
            None => 0 as usize,
        };
        let right_pad = match self.layout_dict.get(&Layout::PaddingRight) {
            Some(pad) => pad.to_owned(),
            None => 0 as usize,
        };
        assert!(self.width > left_pad + right_pad);
        let wrap_length = (self.width - left_pad - right_pad - 1) as usize;
        let mut new_wrapped_strings: Vec<String> = Vec::new();
        for i in raw_content.split("\n") {
            let mut to_wrap = i.to_owned();
            while to_wrap.chars().count() > wrap_length {
                let next = to_wrap.split_off(wrap_length);
                new_wrapped_strings.push(to_wrap.clone());
                to_wrap = next;
            }
            new_wrapped_strings.push(to_wrap);
        }

        return new_wrapped_strings;
    }

    /// Apply Layout Decoration with following order:
    /// Padding
    /// Alignment
    /// Border
    /// Margin
    fn line_layout(&self, mut raw_content: String, text_length: usize) -> String {
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
            let line_length = text_length
                + match self.layout_dict.get(&Layout::PaddingLeft) {
                    Some(i) => *i as usize,
                    None => 0 as usize,
                }
                + match self.layout_dict.get(&Layout::PaddingRight) {
                    Some(i) => *i as usize,
                    None => 0 as usize,
                };
            raw_content =
                self.align_horizontal(raw_content, &self.horizontal_alignment, line_length);
        }

        return raw_content;
    }

    /// Apply inline text decoration
    /// Mark decorated_space = true if you want space decorated
    fn line_text_decoration(&self, raw_content: String) -> String {
        let mut handler = nu_ansi_term::Style::new();
        handler.background = self.background_color;
        handler.foreground = self.foreground_color;

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
        let mut result = String::new();
        if !self.decorated_space {
            for word in raw_content.split(" ") {
                result.push_str(handler.paint(word).to_string().as_str());
                result.push_str(" ")
            }
            result.pop();
        } else {
            result = handler.paint(raw_content).to_string();
        }
        return result;
    }
    fn pad_top(&self, mut raw_content: Vec<String>, size: &usize) -> Vec<String> {
        let pad = " ".repeat(self.width);
        for _ in 0..*size {
            raw_content.insert(0, pad.clone());
        }
        return raw_content;
    }
    fn pad_bottom(&self, mut raw_content: Vec<String>, size: &usize) -> Vec<String> {
        let pad = " ".repeat(self.width);
        for _ in 0..*size {
            raw_content.push(pad.clone())
        }
        return raw_content;
    }
    fn pad_left(&self, raw_content: String, size: &usize) -> String {
        let mut padding_string = " ".repeat(*size as usize);
        padding_string.push_str(&raw_content);
        return padding_string;
    }
    fn pad_right(&self, mut raw_content: String, size: &usize) -> String {
        let padding_string = " ".repeat(*size as usize);
        raw_content.push_str(&padding_string);
        return raw_content;
    }
    fn align_horizontal(
        &self,
        raw_content: String,
        alignment: &Alignment,
        text_length: usize,
    ) -> String {
        let mut new_string = String::new();
        match *alignment {
            Alignment::Start => {
                new_string.push_str(raw_content.as_str());
                let pad = self.width - text_length;
                if pad > 0 {
                    let padding_string = " ".repeat(pad);
                    new_string.push_str(&padding_string);
                }
            }
            Alignment::End => {
                let pad = self.width - text_length;
                if pad > 0 {
                    let padding_string = " ".repeat(pad);
                    new_string.push_str(&padding_string);
                }
                new_string.push_str(raw_content.as_str());
            }
            Alignment::Center => {
                let double_pad = self.width - text_length;
                let pad = double_pad / 2;
                if double_pad > 0 {
                    let padding_string = " ".repeat(pad);
                    new_string.push_str(&padding_string);
                    new_string.push_str(raw_content.as_str());
                    let padding_string = " ".repeat(double_pad - pad);
                    new_string.push_str(&padding_string);
                } else {
                    new_string.push_str(raw_content.as_str());
                }
            }
        }
        return new_string;
    }
}
pub mod getter;
pub mod setter;
