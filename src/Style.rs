use nu_ansi_term;
use nu_ansi_term::Color;
use std::{
    collections::{HashMap, HashSet},
    default,
    os::raw,
};

#[derive(Debug, Clone)]
pub struct Style {
    decorator_dict: HashSet<Decorator>,
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
enum Layout {
    MarginTop,
    MarginLeft,
    MarginBottom,
    MarginRight,
    PaddingTop,
    PaddingLeft,
    PaddingBottom,
    PaddingRight,
    AlignHorizontal,
    AlignVertical,
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

pub fn new() -> Style {
    return Style {
        decorator_dict: HashSet::new(),
        background_color: None,
        foreground_color: None,
        layout_dict: HashMap::new(),
    };
}

impl Drop for Style {
    fn drop(&mut self) {}
}
impl Style {
    pub fn render(&mut self, raw_string: &str) -> String {
        let content = raw_string.to_string();
        //content = self.apply_layout_style(content)
        return self.apply_text_style(content);
    }
    fn apply_layout_style(&self, raw_content: String) -> String {
        let mut raw_content = raw_content;
        for (k, v) in self.layout_dict.iter() {
            match k {
                Layout::MarginTop => {
                    raw_content = self.margin_top(raw_content, v);
                }
                _default => {}
            }
        }
        return String::new();
    }
    fn apply_text_style(&self, raw_content: String) -> String {
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
        let result = handler.paint(raw_content).to_string();
        drop(self);
        return result;
    }
    pub fn is_bold(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Bold);
    }
    pub fn bold(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Bold);
        return self;
    }
    pub fn italic(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Italic);
        return self;
    }
    pub fn underline(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Underline);
        return self;
    }
    pub fn reverse(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Reverse);
        return self;
    }
    pub fn strikethrough(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Strikethrough);
        return self;
    }
    pub fn blink(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Blink);
        return self;
    }
    pub fn foreground_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground_color = Some(Color::Rgb(r, g, b));
        return self;
    }
    pub fn background_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = Some(Color::Rgb(r, g, b));
        return self;
    }
    pub fn reset_foreground(mut self) -> Self {
        self.foreground_color = None;
        return self;
    }
    pub fn reset_background(mut self) -> Self {
        self.background_color = None;
        return self;
    }
    pub fn margin_top(&self, raw_content: String, margin_size: &u32) -> String {
        return raw_content;
    }
}
