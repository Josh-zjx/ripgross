use super::*;
impl Style {
    /// Set text block width
    /// This is the width where border would be drawed
    pub fn set_width(mut self, size: u32) -> Self {
        self.width = size;
        return self;
    }
    /// Set text block height
    /// This is the height where border would be drawed
    pub fn set_height(mut self, size: u32) -> Self {
        self.height = size;
        return self;
    }
    /// Set top padding inside text block
    pub fn padding_top(mut self, size: u32) -> Self {
        self.layout_dict.insert(Layout::PaddingTop, size);
        return self;
    }
    /// Set bottom padding inside text block
    pub fn padding_bottom(mut self, size: u32) -> Self {
        self.layout_dict.insert(Layout::PaddingBottom, size);
        return self;
    }
    /// Set left padding inside text block
    pub fn padding_left(mut self, size: u32) -> Self {
        self.layout_dict.insert(Layout::PaddingLeft, size);
        return self;
    }
    /// Set right padding inside text block
    pub fn padding_right(mut self, size: u32) -> Self {
        self.layout_dict.insert(Layout::PaddingRight, size);
        return self;
    }
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
}
