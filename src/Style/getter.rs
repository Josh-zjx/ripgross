use super::*;

impl Style {
    pub fn get_width(&self) -> u32 {
        return self.width;
    }
    pub fn get_height(&self) -> u32 {
        return self.height;
    }
    pub fn is_bold(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Bold);
    }
    pub fn is_italic(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Italic);
    }
    pub fn is_strikethrough(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Strikethrough);
    }
    pub fn is_underline(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Underline);
    }
    pub fn is_blink(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Blink);
    }
    pub fn is_reverse(&self) -> bool {
        return self.decorator_dict.contains(&Decorator::Reverse);
    }
}
