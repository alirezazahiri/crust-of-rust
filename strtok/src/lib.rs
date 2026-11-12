pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn check_is_static(_: &'static str) {}

        let mut x = "hello world";
        check_is_static(x); // its fine because it can be inferred 

        let z = &mut x; // new mutable reference and its not used 
        // so z's mutable reference's lifetime ends here

        let hello = strtok(&mut x, ' ');

        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}

pub mod dropcheck;
