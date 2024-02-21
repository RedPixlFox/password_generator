#![allow(dead_code)]

use rand::seq::IteratorRandom;

const LETTERS_L: &str = "abcdefghijklmnopqrstuvwxyz";
const LETTERS_U: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";
const SIMPLE_SPECIAL_CHARS: &str = ".!?_-";
const ALL_SPECIAL_CHARS: &str = ".!?_-#,;:+*~=&";

// --------------------------------------------------------
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PasswordGenerator {
    pub settings: PasswordGeneratorSettings,

    pub custom_charset: Option<String>,
    charset: Option<String>,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            custom_charset: None,
            charset: None,
        }
    }
}

impl PasswordGenerator {
    pub fn from_settings(settings: PasswordGeneratorSettings) -> Self {
        Self {
            settings,
            custom_charset: None,
            charset: None,
        }
    }

    pub fn set_custom_chars(&mut self, custom_chars: Option<String>) {
        self.custom_charset = custom_chars;
    }

    /// returns the count of used chars
    fn update_available_chars(&mut self) -> usize {
        let mut available_chars = String::new();
        if self.settings.use_lowercase_letters {
            available_chars += LETTERS_L;
        }
        if self.settings.use_uppercase_letters {
            available_chars += LETTERS_U;
        }
        if self.settings.use_numbers {
            available_chars += NUMBERS;
        }
        match self.settings.special_character_usage {
            SpecialCharacterUsage::None => {}
            SpecialCharacterUsage::Simple => available_chars += SIMPLE_SPECIAL_CHARS,
            SpecialCharacterUsage::All => available_chars += ALL_SPECIAL_CHARS,
        }

        self.charset = Some(available_chars.clone());
        available_chars.len()
    }

    pub fn generate(&mut self) -> Result<String, ()> {
        self.update_available_chars();

        let charset = {
            if self.settings.use_custom_charset {
                match &self.custom_charset {
                    Some(charset) => charset,
                    None => return Err(()),
                }
            } else {
                match &self.charset {
                    Some(charset) => charset,
                    None => return Err(()),
                }
            }
        };
        if charset.len() == 0 {
            return Err(());
        }
        let mut password: String = String::new();

        let mut rng = rand::thread_rng();
        for _ in 0..self.settings.pw_length {
            let ch: char = match charset.chars().choose(&mut rng) {
                Some(ch) => ch,
                None => return Err(()),
            };

            password.insert(0, ch);
        }

        Ok(password)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct PasswordGeneratorSettings {
    pub use_custom_charset: bool,

    pub use_lowercase_letters: bool,
    pub use_uppercase_letters: bool,
    pub use_numbers: bool,
    pub special_character_usage: SpecialCharacterUsage,

    pub pw_length: usize,
}

impl Default for PasswordGeneratorSettings {
    fn default() -> Self {
        Self {
            use_custom_charset: false,
            use_lowercase_letters: true,
            use_uppercase_letters: true,
            use_numbers: true,
            special_character_usage: SpecialCharacterUsage::None,
            pw_length: 8,
        }
    }
}

impl PasswordGeneratorSettings {
    pub fn new(
        use_custom_charset: bool,
        use_lowercase_letters: bool,
        use_uppercase_letters: bool,
        use_numbers: bool,
        special_character_usage: SpecialCharacterUsage,
        length: usize,
    ) -> Self {
        Self {
            use_custom_charset,
            use_lowercase_letters,
            use_uppercase_letters,
            use_numbers,
            special_character_usage,
            pw_length: length,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
pub enum SpecialCharacterUsage {
    None,
    Simple,
    All,
}
