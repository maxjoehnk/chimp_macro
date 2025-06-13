use std::ops::Range;
use std::time::Duration;
use rhai::CustomType;

pub enum PresetType {
    Intensity,
    Color,
    Position,
    Beam,
    Control,
    Special,
}

impl From<PresetType> for Key {
    fn from(value: PresetType) -> Self {
        match value {
            PresetType::Intensity => Key::Intensity,
            PresetType::Color => Key::Color,
            PresetType::Position => Key::Position,
            PresetType::Beam => Key::Beam,
            PresetType::Control => Key::Control,
            PresetType::Special => Key::Special,
        }
    }
}

pub trait ChimpConnection {
    fn send_bool(&self, msg: String, value: bool);
    fn send_msg(&self, msg: String);

    fn press_key(&self, key: Key) {
        println!("Press {key:?}");
        self.hold_key(key);
        self.release_key(key);
    }

    fn hold_key(&self, key: Key) {
        self.send_bool(key.to_osc(), true);
        std::thread::sleep(Duration::from_millis(10));
    }

    fn release_key(&self, key: Key) {
        self.send_bool(key.to_osc(), false);
        std::thread::sleep(Duration::from_millis(20));
    }

    fn press_key_times(&self, key: Key, times: u16) {
        for _ in 0..times {
            self.press_key(key);
        }
    }

    fn enter(&self) {
        self.press_key(Key::Enter);
        std::thread::sleep(Duration::from_millis(50));
    }

    fn send_number(&self, value: u16) {
        let digits = value.to_string();

        for digit in digits.chars() {
            let num = digit as u8 - b'0';
            self.press_key(Key::Number(num));
        }
    }

    fn sync(&self) {
        self.send_msg("/chimp/sync".to_string());
    }

    fn clear(&self) {
        self.press_key(Key::Clear);
    }

    fn select_group(&self, group: u16) {
        self.press_key(Key::Group);
        self.send_number(group);
        self.enter();
    }

    fn select_preset(&self, preset_type: PresetType, preset: u16) {
        self.press_key(Key::Preset);
        self.press_key(preset_type.into());
        self.send_number(preset);
        self.enter();
    }

    fn delete(&self, key: Key, number: Range<u16>) {
        self.press_key(Key::Delete);
        self.press_key(key);
        self.send_number(number.start);
        self.press_key(Key::Thru);
        self.send_number(number.end);
        self.enter();
        std::thread::sleep(Duration::from_millis(500));
        self.enter();
        std::thread::sleep(Duration::from_millis(1000));
    }

    fn record(&self, key: Key, number: u16, mode: RecordMode) {
        self.press_key(Key::Record);
        self.hold_key(Key::Shift);
        let times = match mode {
            RecordMode::Merge => 1,
            RecordMode::Remove => 2,
            RecordMode::Replace => 3,
            RecordMode::Insert => 4,
        };
        self.press_key_times(Key::Record, times);
        self.release_key(Key::Shift);
        self.press_key(key);
        self.send_number(number);
        self.enter();
    }
}

#[derive(Debug, Default, CustomType)]
pub enum RecordMode {
    #[default]
    Merge,
    Remove,
    Replace,
    Insert,
}

#[derive(Debug, Copy, Clone, CustomType)]
pub enum Key {
    Record,
    Edit,
    Delete,
    Copy,
    Move,
    Name,
    Open,
    Select,
    Link,
    Load,
    Off,
    Skip,
    GoTo,
    Time,
    Fixture,
    Group,
    Preset,
    CueList,
    Cue,
    Effect,
    Minus,
    Plus,
    Thru,
    Full,
    At,
    FwSlash,
    Backspace,
    Number(u8),
    Dot,
    Enter,
    Shift,
    Home,
    Set,
    Clear,
    Intensity,
    Position,
    Color,
    Gobo,
    Beam,
    Shaper,
    Control,
    Special,
}

impl Key {
    fn to_osc(&self) -> String {
        let suffix = match self {
            Self::Record => "/keypad/record".to_string(),
            Self::Edit => "/keypad/edit".to_string(),
            Self::Delete => "/keypad/delete".to_string(),
            Self::Copy => "/keypad/copy".to_string(),
            Self::Move => "/keypad/move".to_string(),
            Self::Name => "/keypad/name".to_string(),
            Self::Open => "/keypad/open".to_string(),
            Self::Select => "/keypad/select".to_string(),
            Self::Link => "/keypad/link".to_string(),
            Self::Load => "/keypad/load".to_string(),
            Self::Off => "/keypad/off".to_string(),
            Self::Skip => "/keypad/skip".to_string(),
            Self::GoTo => "/keypad/goto".to_string(),
            Self::Time => "/keypad/time".to_string(),
            Self::Fixture => "/keypad/fixture".to_string(),
            Self::Group => "/keypad/group".to_string(),
            Self::Preset => "/keypad/preset".to_string(),
            Self::CueList => "/keypad/cuelist".to_string(),
            Self::Cue => "/keypad/cue".to_string(),
            Self::Effect => "/keypad/effect".to_string(),
            Self::Minus => "/keypad/minus".to_string(),
            Self::Plus => "/keypad/plus".to_string(),
            Self::Thru => "/keypad/thru".to_string(),
            Self::Full => "/keypad/full".to_string(),
            Self::At => "/keypad/at".to_string(),
            Self::FwSlash => "/keypad/fw_slash".to_string(),
            Self::Backspace => "/keypad/backspace".to_string(),
            Self::Number(n) => format!("/keypad/{n}"),
            Self::Dot => "/keypad/dot".to_string(),
            Self::Enter => "/keypad/enter".to_string(),
            Self::Shift => "/keypad/shift".to_string(),
            Self::Home => "/keypad/home".to_string(),
            Self::Set => "/keypad/set".to_string(),
            Self::Clear => "/clear/btn".to_string(),
            Self::Intensity => "/feature/select/intensity".to_string(),
            Self::Position => "/feature/select/position".to_string(),
            Self::Color => "/feature/select/color".to_string(),
            Self::Gobo => "/feature/select/gobo".to_string(),
            Self::Beam => "/feature/select/beam".to_string(),
            Self::Shaper => "/feature/select/shaper".to_string(),
            Self::Control => "/feature/select/control".to_string(),
            Self::Special => "/feature/select/special".to_string(),
        };

        format!("/chimp/programmer{suffix}")
    }
}
