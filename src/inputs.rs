#[derive(Clone, Debug)]
pub struct Input {
    pub offset: usize,
    pub source: String,
}

impl Input {
    pub fn new(offset: usize, source: String) -> Self {
        Input { offset, source }
    }

    pub fn char_offset(mut self, count: usize) -> Self {
        self.offset += count;
        self.source = self.source.chars().skip(count).collect();
        self
    }
}

impl Into<Input> for String {
    fn into(self) -> Input {
        Input {
            offset: 0,
            source: self,
        }
    }
}

impl Into<Input> for &str {
    fn into(self) -> Input {
        Input {
            offset: 0,
            source: self.to_string(),
        }
    }
}

#[cfg(test)]
mod test_input {
    use super::Input;

    #[test]
    fn test_offset() {
        let input = Input::new(3, "hello there!".to_string());
        let input = input.char_offset(4);
        assert_eq!(input.offset, 7);
        assert_eq!(input.source, "o there!".to_string());
    }
}
