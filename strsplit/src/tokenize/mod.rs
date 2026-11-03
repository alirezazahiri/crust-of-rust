pub struct Tokenize<'txt> {
    text: Option<&'txt str>,
}

impl<'txt> Tokenize<'txt> {
    pub fn new(text: &'txt str) -> Self {
        Self { text: Some(text) }
    }
}

pub trait TokenizeHelper {
    fn find_next_nonalnum(self) -> Option<(usize, usize)>;
}

impl TokenizeHelper for &str {
    fn find_next_nonalnum(self) -> Option<(usize, usize)> {
        for (i, ch) in self.char_indices() {
            if !ch.is_alphanumeric() {
                for (j, ch) in self[i..].char_indices() {
                    if ch.is_alphanumeric() {
                        return Some((i, i + j));
                    }
                }
                return Some((i, self.len()));
            }
        }
        None
    }
}

impl<'txt> Iterator for Tokenize<'txt> {
    type Item = &'txt str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.text.as_mut()?;
        if let Some((start, end)) = remainder.find_next_nonalnum() {
            let token = &remainder[..start];
            *remainder = &remainder[end..];
            if token.is_empty() {
                return self.next();
            } else {
                return Some(token);
            }
        } else {
            if !remainder.is_empty() {
                self.text.take()
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test_tokenize {
    use crate::tokenize::Tokenize;

    #[test]
    fn empty() {
        let mut t = Tokenize::new("");
        assert_eq!(t.next(), None)
    }
    #[test]
    fn whitespace() {
        let mut t = Tokenize::new(" ");
        assert_eq!(t.next(), None)
    }

    #[test]
    fn one() {
        let mut t = Tokenize::new("thisisasingletoken");
        assert_eq!(t.next(), Some("thisisasingletoken"))
    }

    #[test]
    fn complete() {
        let mut t = Tokenize::new("Hello, World!");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }
    #[test]
    fn empty_end() {
        let mut t = Tokenize::new("Hello, World");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }

    #[test]
    fn space_end() {
        let mut t = Tokenize::new("Hello, World ");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }
    
    #[test]
    fn na_start() {
        let mut t = Tokenize::new("'Hello, World");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }
    #[test]
    fn space_start() {
        let mut t = Tokenize::new(" Hello, World");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }
    #[test]
    fn long_na_start() {
        let mut t = Tokenize::new("./>. ?>?> > Hello, World");
        assert_eq!(t.next(), Some("Hello"));
        assert_eq!(t.next(), Some("World"));
        assert_eq!(t.next(), None)
    }
}
