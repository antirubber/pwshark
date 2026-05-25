use rand::Rng;
use zeroize::Zeroize;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=,.<>?";

pub const WORD_LIST: &str = include_str!("../wordlist.txt");

#[derive(Clone, Debug)]
pub enum Mode {
    Random,
    Memorable,
}

#[derive(Clone, Debug)]
pub struct RandomConfig {
    pub length: u8,
    pub uppercase: bool,
    pub lowercase: bool,
    pub numbers: bool,
    pub symbols: bool,
}

impl Default for RandomConfig {
    fn default() -> Self {
        Self {
            length: 16,
            uppercase: true,
            lowercase: true,
            numbers: true,
            symbols: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemorableConfig {
    pub word_count: u8,
    pub separator: String,
    pub capitalize: bool,
    pub add_numbers: bool,
    pub truncate: bool,
}

impl Default for MemorableConfig {
    fn default() -> Self {
        Self {
            word_count: 4,
            separator: "-".into(),
            capitalize: true,
            add_numbers: true,
            truncate: true,
        }
    }
}

#[derive(Zeroize)]
#[zeroize(drop)]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn next_u32_bounded(rng: &mut impl Rng, bound: u32) -> u32 {
    rng.random_range(0..bound)
}

pub fn generate_random(rng: &mut impl Rng, cfg: &RandomConfig) -> Password {
    let mut charset = String::new();
    if cfg.uppercase {
        charset.push_str(UPPERCASE);
    }
    if cfg.lowercase {
        charset.push_str(LOWERCASE);
    }
    if cfg.numbers {
        charset.push_str(NUMBERS);
    }
    if cfg.symbols {
        charset.push_str(SYMBOLS);
    }
    if charset.is_empty() {
        charset.push_str(LOWERCASE);
    }
    let chars: Vec<char> = charset.chars().collect();
    let len = cfg.length.clamp(8, 64) as usize;
    let password: String = (0..len)
        .map(|_| {
            let idx = next_u32_bounded(rng, chars.len() as u32) as usize;
            chars[idx]
        })
        .collect();
    Password(password)
}

pub fn generate_memorable(rng: &mut impl Rng, cfg: &MemorableConfig) -> Password {
    let words_raw: Vec<&str> = WORD_LIST.lines().collect();
    if words_raw.is_empty() {
        return Password("error-empty-wordlist".into());
    }
    let count = cfg.word_count.clamp(3, 8) as usize;
    let indices: Vec<usize> = (0..count)
        .map(|_| next_u32_bounded(rng, words_raw.len() as u32) as usize)
        .collect();
    let mut words: Vec<String> = indices.iter().map(|&i| words_raw[i].to_string()).collect();

    if cfg.truncate {
        for word in &mut words {
            *word = truncate_word(word);
        }
    }

    let mut password = words.join(&cfg.separator);

    if cfg.capitalize {
        password = apply_random_capitalization(rng, &password);
    }

    if cfg.add_numbers {
        password = apply_random_numbers(rng, &password);
    }

    Password(password)
}

fn truncate_word(word: &str) -> String {
    const MAX: usize = 5;
    if word.len() <= MAX {
        return word.to_string();
    }
    let vowels = "aeiouAEIOU";
    let mut result = String::new();
    let mut first_vowel = false;
    for ch in word.chars() {
        if result.len() >= MAX {
            break;
        }
        if vowels.contains(ch) {
            if !first_vowel {
                result.push(ch);
                first_vowel = true;
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn apply_random_capitalization(rng: &mut impl Rng, password: &str) -> String {
    let letter_positions: Vec<usize> = password
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_alphabetic())
        .map(|(i, _)| i)
        .collect();
    if letter_positions.is_empty() {
        return password.to_string();
    }
    let num_caps = (next_u32_bounded(rng, 3) + 1).min(letter_positions.len() as u32) as usize;
    let mut chars: Vec<char> = password.chars().collect();
    let mut selected = std::collections::HashSet::new();
    for _ in 0..num_caps {
        let pos = letter_positions[next_u32_bounded(rng, letter_positions.len() as u32) as usize];
        selected.insert(pos);
    }
    for pos in selected {
        let mut char_iter = password.chars();
        let char_at_pos = char_iter.nth(pos).unwrap();
        if let Some(c) = chars.get_mut(pos) {
            *c = char_at_pos.to_ascii_uppercase();
        }
    }
    chars.into_iter().collect()
}

fn apply_random_numbers(rng: &mut impl Rng, password: &str) -> String {
    let count = (next_u32_bounded(rng, 3) + 1) as usize;
    let mut chars: Vec<char> = password.chars().collect();
    for _ in 0..count {
        let digit = char::from_digit(next_u32_bounded(rng, 10), 10).unwrap();
        let pos = next_u32_bounded(rng, chars.len() as u32 + 1) as usize;
        chars.insert(pos, digit);
    }
    chars.into_iter().collect()
}

pub fn calculate_entropy(password: &str) -> f64 {
    if password.is_empty() {
        return 0.0;
    }
    let mut charset_size = 0u64;
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());
    if has_lower {
        charset_size += 26;
    }
    if has_upper {
        charset_size += 26;
    }
    if has_digit {
        charset_size += 10;
    }
    if has_symbol {
        charset_size += SYMBOLS.len() as u64;
    }
    if charset_size == 0 {
        return 0.0;
    }
    (charset_size as f64).log2() * password.len() as f64
}

pub fn strength_label(entropy: f64) -> &'static str {
    if entropy < 40.0 {
        "Weak"
    } else if entropy < 60.0 {
        "Fair"
    } else if entropy < 80.0 {
        "Good"
    } else {
        "Strong"
    }
}

pub fn separator_presets() -> Vec<&'static str> {
    vec!["-", ".", "_", "/", " ", ""]
}
