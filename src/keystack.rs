use crossterm::event::{KeyCode, KeyModifiers};
use std::cmp::PartialEq;
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct KeyCombination {
    pub modifier: KeyModifiers,
    pub key: KeyCode,
}

#[derive(PartialEq)]
pub enum Actions {
    Close,
    Ignore,
}
#[derive(Debug)]
pub struct KeyStack {
    keys: [Option<KeyCombination>; 2],
    top: usize, // points to the stack top
}

impl KeyStack {
    pub fn default() -> Self {
        KeyStack {
            keys: [None, None],
            top: 0,
        }
    }
    const CTRL_X: KeyCombination = KeyCombination {
        modifier: KeyModifiers::CONTROL,
        key: KeyCode::Char('x'),
    };

    const CTRL_C: KeyCombination = KeyCombination {
        modifier: KeyModifiers::CONTROL,
        key: KeyCode::Char('c'),
    };

    fn is_exit_combination(&self) -> bool {
        let first = self.keys[0].as_ref();
        let second = self.keys[1].as_ref();

        matches!(
            (first, second),
            (Some(x), Some(c)) if x == &Self::CTRL_X && c == &Self::CTRL_C
        )
    }

    pub fn push(&mut self, key_combo: KeyCombination) -> Actions {
        assert!(self.top < 2);

        self.keys[self.top] = Some(key_combo);
        self.top += 1;
        // shortcuts with only one combination
        if self.top < 2 {
            Actions::Ignore
        } else {
            // shortcuts with two combinations
            self.top = 0;
            if self.is_exit_combination() {
                Actions::Close
            } else {
                Actions::Ignore
            }
        }
    }
}

impl fmt::Display for KeyStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyStack[")?;
        for (i, key) in self.keys.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            match key {
                Some(combo) => {
                    let modifier_str = if combo.modifier.is_empty() {
                        String::new()
                    } else {
                        format!("{:?}+", combo.modifier)
                    };
                    write!(f, "{}{:?}", modifier_str, combo.key)?;
                }
                None => write!(f, "None")?,
            }
        }
        write!(f, "], Top: {}", self.top)
    }
}
