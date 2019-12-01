use std::ops::Add;

pub enum Keys {
    Null,
    Cancel,
    Help,
    Backspace,
    Tab,
    Clear,
    Return,
    Enter,
    Shift,
    Control,
    Alt,
    Pause,
    Escape,
    Space,
    PageUp,
    PageDown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    Semicolon,
    Equals,
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    Multiply,
    Add,
    Separator,
    Subtract,
    Decimal,
    Divide,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Meta,
    Command,
}

impl Keys {
    pub fn value(&self) -> char {
        match self {
            Keys::Null => '\u{e000}',
            Keys::Cancel => '\u{e001}',
            Keys::Help => '\u{e002}',
            Keys::Backspace => '\u{e003}',
            Keys::Tab => '\u{e004}',
            Keys::Clear => '\u{e005}',
            Keys::Return => '\u{e006}',
            Keys::Enter => '\u{e007}',
            Keys::Shift => '\u{e008}',
            Keys::Control => '\u{e009}',
            Keys::Alt => '\u{e00a}',
            Keys::Pause => '\u{e00b}',
            Keys::Escape => '\u{e00c}',
            Keys::Space => '\u{e00d}',
            Keys::PageUp => '\u{e00e}',
            Keys::PageDown => '\u{e00f}',
            Keys::End => '\u{e010}',
            Keys::Home => '\u{e011}',
            Keys::Left => '\u{e012}',
            Keys::Up => '\u{e013}',
            Keys::Right => '\u{e014}',
            Keys::Down => '\u{e015}',
            Keys::Insert => '\u{e016}',
            Keys::Delete => '\u{e017}',
            Keys::Semicolon => '\u{e018}',
            Keys::Equals => '\u{e019}',
            Keys::NumPad0 => '\u{e01a}',
            Keys::NumPad1 => '\u{e01b}',
            Keys::NumPad2 => '\u{e01c}',
            Keys::NumPad3 => '\u{e01d}',
            Keys::NumPad4 => '\u{e01e}',
            Keys::NumPad5 => '\u{e01f}',
            Keys::NumPad6 => '\u{e020}',
            Keys::NumPad7 => '\u{e021}',
            Keys::NumPad8 => '\u{e022}',
            Keys::NumPad9 => '\u{e023}',
            Keys::Multiply => '\u{e024}',
            Keys::Add => '\u{e025}',
            Keys::Separator => '\u{e026}',
            Keys::Subtract => '\u{e027}',
            Keys::Decimal => '\u{e028}',
            Keys::Divide => '\u{e029}',
            Keys::F1 => '\u{e031}',
            Keys::F2 => '\u{e032}',
            Keys::F3 => '\u{e033}',
            Keys::F4 => '\u{e034}',
            Keys::F5 => '\u{e035}',
            Keys::F6 => '\u{e036}',
            Keys::F7 => '\u{e037}',
            Keys::F8 => '\u{e038}',
            Keys::F9 => '\u{e039}',
            Keys::F10 => '\u{e03a}',
            Keys::F11 => '\u{e03b}',
            Keys::F12 => '\u{e03c}',
            Keys::Meta => '\u{e03d}',
            Keys::Command => '\u{e03d}',
        }
    }
}

impl Add for Keys {
    type Output = TypingData;

    fn add(self, rhs: Self) -> Self::Output {
        let data = vec![self.value(), rhs.value()];
        Self::Output { data }
    }
}

pub struct TypingData {
    data: Vec<char>,
}

impl TypingData {
    pub fn to_string(&self) -> String {
        self.data.iter().collect()
    }

    pub fn as_vec(&self) -> Vec<char> {
        self.data.clone()
    }
}

impl From<String> for TypingData {
    fn from(value: String) -> Self {
        TypingData {
            data: value.chars().collect(),
        }
    }
}

impl Add for TypingData {
    type Output = TypingData;

    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();
        data.extend(rhs.data.iter().cloned());
        Self::Output { data }
    }
}

impl Add<Keys> for TypingData {
    type Output = TypingData;

    fn add(self, rhs: Keys) -> Self::Output {
        let mut data = self.data.clone();
        data.push(rhs.value());
        Self::Output { data }
    }
}
