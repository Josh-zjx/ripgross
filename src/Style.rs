use nu_ansi_term;
use nu_ansi_term::Color;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Style {
    decorator_dict: HashSet<Decorator>,
    decorated_space: bool,
    background_color: Option<Color>,
    foreground_color: Option<Color>,
}

#[derive(Debug, Clone)]
pub struct StyleBlock {
    width: usize,
    height: usize,
    raw_string: String,
    content: Vec<String>,
    style: Style,
    horizontal_alignment: Alignment,
    layout_dict: HashMap<Layout, usize>,
    border_dict: HashSet<Border>,
    paragraph_fixed: bool,
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
}

/// Create a new Style object
///
/// With default width 80 and height 24
impl StyleBlock {
    pub fn new() -> StyleBlock {
        return StyleBlock {
            style: Style {
                decorator_dict: HashSet::new(),
                decorated_space: true,
                background_color: None,
                foreground_color: None,
            },
            width: 80,
            height: 24,
            content: Vec::new(),
            raw_string: String::new(),
            horizontal_alignment: Alignment::Start,
            layout_dict: HashMap::new(),
            border_dict: HashSet::new(),
            paragraph_fixed: false,
        };
    }
    /// Put Target Block to the Right of self
    pub fn join_right(mut self, mut target: StyleBlock) -> StyleBlock {
        self.render_paragraph();
        target.render_paragraph();
        let new_height = if self.height > target.height {
            self.height
        } else {
            target.height
        };
        for i in 0..new_height {
            if i >= self.height {
                let padding_string = " ".repeat(self.width);
                self.content.push(padding_string);
            }
            if i >= target.height {
                let padding_string = " ".repeat(target.width);
                target.content.push(padding_string);
            }
        }
        for i in 0..new_height {
            self.content[i].push_str(target.content[i].as_str());
        }
        self.width += target.width;
        self.height = new_height;
        return self;
    }
    /// Put Target Block to the bottom of self
    pub fn join_bottom(mut self, mut target: StyleBlock) -> StyleBlock {
        self.render_paragraph();
        target.render_paragraph();
        if self.width > target.width {
            let width_difference = self.width - target.width;
            let padding_string = " ".repeat(width_difference);
            for i in 0..target.height {
                target.content[i].push_str(padding_string.clone().as_str());
            }
        } else {
            let width_difference = target.width - self.width;
            let padding_string = " ".repeat(width_difference);
            for i in 0..target.height {
                target.content[i].push_str(padding_string.clone().as_str());
            }
            self.width = target.width;
        };

        self.content.append(&mut target.content);
        self.height += target.height;
        return self;
    }
    pub fn finalize(mut self) -> String {
        self.render_paragraph();
        if self.layout_dict.contains_key(&Layout::PaddingTop) {
            let size = self.layout_dict.get(&Layout::PaddingTop).unwrap();
            self.pad_top(&size.clone());
        }
        if self.layout_dict.contains_key(&Layout::PaddingBottom) {
            let size = self.layout_dict.get(&Layout::PaddingBottom).unwrap();
            self.pad_bottom(&size.clone());
        }
        self.draw_border();
        self.draw_margin();

        // Finalize output to String
        let mut content = String::new();
        for i in self.content {
            content.push_str(i.as_str());
            content.push('\n');
        }
        return content;
    }
    fn render_paragraph(&mut self) {
        if self.paragraph_fixed {
            return;
        }
        self.content = self.text_wrap();
        for i in 0..self.content.len() {
            let text_length = dbg!(self.content[i].chars().count());
            let decorated_line = self.style.line_text_decoration(self.content[i].clone());
            self.content[i] = self.line_layout(decorated_line, text_length);
        }
        self.height = self.content.len();
        self.paragraph_fixed = true;
    }
    fn draw_margin(&mut self) {
        match self.layout_dict.get(&Layout::MarginLeft) {
            Some(size) => {
                for i in self.content.iter_mut() {
                    *i = Self::pad_left(i.clone(), size);
                }
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginRight) {
            Some(size) => {
                for i in self.content.iter_mut() {
                    *i = Self::pad_right(i.clone(), size);
                }
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginTop) {
            Some(size) => {
                self.pad_top(&size.clone());
            }
            None => (),
        };
        match self.layout_dict.get(&Layout::MarginBottom) {
            Some(size) => {
                self.pad_bottom(&size.clone());
            }
            None => (),
        };
    }
    fn draw_border(&mut self) {
        let top_left = self.border_dict.contains(&Border::BorderLeft)
            && self.border_dict.contains(&Border::BorderTop);
        let top_right = self.border_dict.contains(&Border::BorderRight)
            && self.border_dict.contains(&Border::BorderTop);
        let bottom_left = self.border_dict.contains(&Border::BorderLeft)
            && self.border_dict.contains(&Border::BorderBottom);
        let bottom_right = self.border_dict.contains(&Border::BorderRight)
            && self.border_dict.contains(&Border::BorderBottom);
        for i in self.content.iter_mut() {
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
            self.content.insert(0, borderline);
        }
        if self.border_dict.contains(&Border::BorderBottom) {
            let mut borderline = "─".repeat(self.width);
            if bottom_left {
                borderline.insert(0, '└');
            }
            if bottom_right {
                borderline.push('┘');
            }
            self.content.push(borderline);
        }
    }
    /// Apply text wrap
    /// Would panic if valid text length is negative
    fn text_wrap(&self) -> Vec<String> {
        let left_pad = match self.layout_dict.get(&Layout::PaddingLeft) {
            Some(pad) => pad.to_owned(),
            None => 0 as usize,
        };
        let right_pad = match self.layout_dict.get(&Layout::PaddingRight) {
            Some(pad) => pad.to_owned(),
            None => 0 as usize,
        };
        assert!(self.width > left_pad + right_pad);
        let wrap_length = (self.width - left_pad - right_pad) as usize;
        let mut new_wrapped_strings: Vec<String> = Vec::new();
        for i in self.raw_string.split("\n") {
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
    fn line_layout(&self, mut raw_content: String, text_length: usize) -> String {
        // Add Padding
        if self.layout_dict.contains_key(&Layout::PaddingLeft) {
            raw_content = Self::pad_left(
                raw_content,
                self.layout_dict.get(&Layout::PaddingLeft).unwrap(),
            );
        }
        if self.layout_dict.contains_key(&Layout::PaddingRight) {
            raw_content = Self::pad_right(
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
            self.align_horizontal(raw_content, &self.horizontal_alignment, dbg!(line_length));

        return raw_content;
    }
    fn pad_top(&mut self, size: &usize) {
        let pad = " ".repeat(self.width);
        for _ in 0..*size {
            self.content.insert(0, pad.clone());
        }
    }
    fn pad_bottom(&mut self, size: &usize) {
        let pad = " ".repeat(self.width);
        for _ in 0..*size {
            self.content.push(pad.clone())
        }
    }
    fn pad_left(raw_content: String, size: &usize) -> String {
        let mut padding_string = " ".repeat(*size as usize);
        padding_string.push_str(&raw_content);
        return padding_string;
    }
    fn pad_right(mut raw_content: String, size: &usize) -> String {
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
                    let padding_string = " ".repeat(dbg!(pad));
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

impl Style {
    pub fn new() -> Style {
        return Style {
            decorator_dict: HashSet::new(),
            decorated_space: true,
            background_color: None,
            foreground_color: None,
        };
    }
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
    pub fn render(self, raw_string: &str) -> String {
        let content = raw_string.to_string();
        let decorated_line = self.line_text_decoration(content);

        return decorated_line;
    }

    pub fn render_to_block(self, raw_string: &str) -> StyleBlock {
        return StyleBlock {
            raw_string: raw_string.to_owned(),
            style: self,
            width: 80,
            height: 24,
            paragraph_fixed: false,
            content: Vec::new(),
            horizontal_alignment: Alignment::Start,
            layout_dict: HashMap::new(),
            border_dict: HashSet::new(),
        };
    }
    pub fn to_block(self) -> StyleBlock {
        return StyleBlock {
            raw_string: String::new(),
            style: self,
            width: 80,
            height: 24,
            paragraph_fixed: false,
            content: Vec::new(),
            horizontal_alignment: Alignment::Start,
            layout_dict: HashMap::new(),
            border_dict: HashSet::new(),
        };
    }

    /// Apply Layout Decoration with following order:
    /// Padding
    /// Alignment
    /// Border
    /// Margin

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
}
pub mod getter;
pub mod setter;
