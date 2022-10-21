use super::*;
impl StyleBlock {
    /// Set text block width
    /// This is the width where border would be drawed
    pub fn set_width(mut self, size: usize) -> Self {
        self.width = size;
        return self;
    }
    /// Set text block height
    /// This is the height where border would be drawed
    pub fn set_height(mut self, size: usize) -> Self {
        self.height = size;
        return self;
    }
    /// Set top padding inside text block
    pub fn padding_top(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::PaddingTop, size);
        return self;
    }
    /// Set bottom padding inside text block
    pub fn padding_bottom(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::PaddingBottom, size);
        return self;
    }
    /// Set left padding inside text block
    pub fn padding_left(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::PaddingLeft, size);
        return self;
    }
    /// Set right padding inside text block
    pub fn padding_right(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::PaddingRight, size);
        return self;
    }
    /// Set text alignment to START
    pub fn align_start(mut self) -> Self {
        self.horizontal_alignment = Alignment::Start;
        return self;
    }
    /// Set text alignment to CENTER
    pub fn align_center(mut self) -> Self {
        self.horizontal_alignment = Alignment::Center;
        return self;
    }
    /// Set text alignment to END
    pub fn align_end(mut self) -> Self {
        self.horizontal_alignment = Alignment::End;
        return self;
    }
    /// Set left border
    pub fn border_left(mut self) -> Self {
        self.border_dict.insert(Border::BorderLeft);
        return self;
    }
    /// Set right border
    pub fn border_right(mut self) -> Self {
        self.border_dict.insert(Border::BorderRight);
        return self;
    }
    /// Set top border
    pub fn border_top(mut self) -> Self {
        self.border_dict.insert(Border::BorderTop);
        return self;
    }
    /// Set bottom border
    pub fn border_bottom(mut self) -> Self {
        self.border_dict.insert(Border::BorderBottom);
        return self;
    }
    /// Set top padding inside text block
    pub fn margin_top(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::MarginTop, size);
        return self;
    }
    /// Set bottom padding inside text block
    pub fn margin_bottom(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::MarginBottom, size);
        return self;
    }
    /// Set left padding inside text block
    pub fn margin_left(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::MarginLeft, size);
        return self;
    }
    /// Set right padding inside text block
    pub fn margin_right(mut self, size: usize) -> Self {
        self.layout_dict.insert(Layout::MarginRight, size);
        return self;
    }
    /// Set Bold to text
    pub fn bold(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Bold);
        return self;
    }
    /// Set Italic to text
    pub fn italic(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Italic);
        return self;
    }
    /// Set Underline to text
    /// would Underline space if decorator_space is true
    pub fn underline(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Underline);
        return self;
    }
    /// Set Reverse to text
    pub fn reverse(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Reverse);
        return self;
    }
    /// Set Strikethrough/Crossout to text
    /// Would stribethrough/Crossout space if decorator_space is true
    pub fn strikethrough(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Strikethrough);
        return self;
    }
    /// Set Blink to text
    pub fn blink(mut self) -> Self {
        self.style.decorator_dict.insert(Decorator::Blink);
        return self;
    }
    pub fn foreground_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style.foreground_color = Some(Color::Rgb(r, g, b));
        return self;
    }
    pub fn background_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.style.background_color = Some(Color::Rgb(r, g, b));
        return self;
    }
    pub fn reset_foreground(mut self) -> Self {
        self.style.foreground_color = None;
        return self;
    }
    pub fn reset_background(mut self) -> Self {
        self.style.background_color = None;
        return self;
    }
    /// Set decorated_space = false
    ///
    /// ### This would aggregate space between words !
    pub fn ignore_space(mut self) -> Self {
        self.style.decorated_space = false;
        return self;
    }
}
impl Style {
    /// Set Bold to text
    pub fn bold(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Bold);
        return self;
    }
    /// Set Italic to text
    pub fn italic(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Italic);
        return self;
    }
    /// Set Underline to text
    /// would Underline space if decorator_space is true
    pub fn underline(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Underline);
        return self;
    }
    /// Set Reverse to text
    pub fn reverse(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Reverse);
        return self;
    }
    /// Set Strikethrough/Crossout to text
    /// Would stribethrough/Crossout space if decorator_space is true
    pub fn strikethrough(mut self) -> Self {
        self.decorator_dict.insert(Decorator::Strikethrough);
        return self;
    }
    /// Set Blink to text
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
    /// Set decorated_space = false
    ///
    /// ### This would aggregate space between words !
    pub fn ignore_space(mut self) -> Self {
        self.decorated_space = false;
        return self;
    }
}
