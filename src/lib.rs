//! **A utility macro for flexible slug genereation that handles unicode.**
//! 
//! The `slugify!` macro implements a flexible slug generator, allowing for stop words, custom separator
//! and maximum length options. The macro provides both a simple interface with sane default parameters
//! but also allows the parameters to be overriden when needed.
//! 
//! Features:
//!
//!* Unicode strings support (phonetic conversion).
//!* Support for custom slug separator.
//!* Stop words filtering.
//!* Slug maximum length support.
//!
//! 
//!# Usage
//! 
//! This crate is on crates.io and can be used by adding `slugify` to the dependencies in your project's
//! `Cargo.toml`
//! 
//! ```toml
//! [dependencies]
//! slugify = "0.1.0"
//! ```
//! 
//!  and this to your crate root:
//! 
//!```rust,ignore
//! #[macro_use] extern crate slugify;
//!use slugify::slugify;
//!```
//! 
//!# Examples
//!
//!## Basic slug generation
//! 
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world"), "hello-world");
//! # }
//!```
//! 
//!## Using a custom separator
//! 
//! ```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", separator = "."), "hello.world");
//!assert_eq!(slugify!("hello world", separator = " "), "hello world");
//!assert_eq!(slugify!("hello world", separator = ""), "helloworld");
//! # }
//! ```
//! 
//!## Stop words filtering
//! 
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the quick brown fox jumps over the lazy dog", stop_words = "the,fox"), "quick-brown-jumps-over-lazy-dog");
//! # }
//!```
//! 
//!## Maximum length
//! 
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("hello world", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//! # }
//!```
//! 
//!## Phonetic Conversion and accented text
//! 
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("影師嗎"), "ying-shi-ma");
//!assert_eq!(slugify!("Æúű--cool?"), "aeuu-cool");
//!assert_eq!(slugify!("Nín hǎo. Wǒ shì zhōng guó rén"), "nin-hao-wo-shi-zhong-guo-ren");
//! # }
//!```
//! 
//!## Passing multiple optional parameters.
//!
//! **NOTE:** the order of optional parameters matters: **stop_words**, **separator**
//! and then **max_length**. All of them are optional, however when specifying more than one optional parameter, this
//! order must be adhered.
//! 
//!```rust
//! # #[macro_use] extern crate slugify;
//! # use slugify::slugify;
//! # fn main() {
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"), "hello-world");
//!assert_eq!(slugify!("the hello world", separator = ".", max_length = 10), "the.hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
//!assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-", max_length = 20), "hello-world");
//! # }
//!```
//!
extern crate unidecode;
use unidecode::unidecode;


#[macro_export]
macro_rules! slugify {
    ($text:expr) => (
        {
         slugify($text, "", "-", None)
        }
    );

    ($text:expr, stop_words=$stopwords:expr) => (
        {
         slugify($text, $stopwords, "-", None)
        }
    );

    ($text:expr, separator=$sep:expr) => (
        {
         slugify($text, "", $sep, None)
        }
    );

    ($text:expr, max_length=$len:expr) => (
        {
         slugify($text, "", "-", Some($len))
        }
    );

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr) => (
        {
         slugify($text, $stopwords, $sep, None)
        }
    );

    ($text:expr, stop_words=$stopwords:expr, max_length=$len:expr) => (
        {
         slugify($text, $stopwords, "-", Some($len))
        }
    );

    ($text:expr, separator=$sep:expr, max_length=$len:expr) => (
        {
         slugify($text, "", $sep, Some($len))
        }
    );

    ($text:expr, stop_words=$stopwords:expr, separator=$sep:expr, max_length=$len:expr) => (
        {
         slugify($text, $stopwords, $sep, Some($len))
        }
    );


}

pub fn slugify(string: &str, stop_words: &str, sep: &str, max_length: Option<usize>) -> String {
    let char_vec: Vec<char> = sep.chars().collect();
    let mut string: String = unidecode(string.into())
        .to_lowercase()
        .trim()
        // guard against an empty char
        // None result is redundant with .trim() but is nondestructive/safe
        .trim_matches(match char_vec.get(0) {
            Some(a) => a.to_owned(),
            None => ' ',
        })
        .replace(' ', &sep.to_string());

    // remove stop words
    for word in stop_words.split(",") {
        if !word.is_empty() {
            string = string.replace(word, &sep.to_string());
        }

    }

    let mut slug = Vec::with_capacity(string.len());

    let mut is_sep = true;

    for x in string.chars() {
        match x {
            'a'...'z' | '0'...'9' => {
                is_sep = false;
                slug.push(x as u8);
            }
            _ => {
                if !is_sep {
                    is_sep = true;
                    slug.push(char_vec[0] as u8);
                } else {
                }
            }
        }
    }

    if char_vec.len() > 0 && slug.last() == Some(&(char_vec[0] as u8)) {
        slug.pop();
    }

    let mut s = String::from_utf8(slug).unwrap();

    match max_length {
        Some(x) => {
            s.truncate(x);
            s = s.trim_right_matches(char_vec[0]).to_string();
        }
        None => {}
    }

    s

}



#[cfg(test)]
mod tests {
    use slugify;
    #[test]
    fn basic() {
        assert_eq!(slugify("hello world", "", "-", None), "hello-world");
        assert_eq!(slugify("hello world-", "", "-", None), "hello-world");
        assert_eq!(slugify("hello world ", "", "-", None), "hello-world");
    }

    #[test]
    fn test_email() {
        assert_eq!(slugify!("alice@bob.com"), "alice-bob-com");
    }

    #[test]
    fn test_starts_with_number() {
        assert_eq!(slugify!("10 amazing secrets"), "10-amazing-secrets");
    }

    #[test]
    fn test_contains_numbers() {
        assert_eq!(slugify!("the 101 dalmatians"), "the-101-dalmatians");
    }

    #[test]
    fn test_ends_with_number() {
        assert_eq!(slugify!("lucky number 7"), "lucky-number-7");
    }

    #[test]
    fn test_numbers_only() {
        assert_eq!(slugify!("101"), "101");
    }

    #[test]
    fn test_numbers_and_symbols() {
        assert_eq!(slugify!("1000 reasons you are #1"),
                   "1000-reasons-you-are-1");
    }

    #[test]
    fn test_stop_words() {
        assert_eq!(slugify("hello world", "world", "-", None), "hello");
        assert_eq!(slugify!("hello world", stop_words = "world"), "hello");
    }

    #[test]
    fn test_differently_cased_stopword_match() {
        assert_eq!(slugify("Foo A FOO B foo C", "foo", "-", None), "a-b-c");
    }

    #[test]
    fn test_multiple_stop_words() {
        assert_eq!(slugify("the quick brown fox jumps over the lazy dog",
                           "the",
                           "-",
                           None),
                   "quick-brown-fox-jumps-over-lazy-dog");
        assert_eq!(slugify("the quick brown fox jumps over the lazy dog",
                           "the,fox",
                           "-",
                           None),
                   "quick-brown-jumps-over-lazy-dog");
        assert_eq!(slugify!("the quick brown fox jumps over the lazy dog",
                            stop_words = "the,fox"),
                   "quick-brown-jumps-over-lazy-dog");
    }

    #[test]
    fn test_stopwords_with_different_separator() {
        assert_eq!(slugify("the quick brown fox jumps over the lazy dog",
                           "the",
                           " ",
                           None),
                   "quick brown fox jumps over lazy dog");
        assert_eq!(slugify!("the quick brown fox jumps over the lazy dog",
                            stop_words = "the",
                            separator = " "),
                   "quick brown fox jumps over lazy dog");
    }

    #[test]
    fn test_separator() {
        assert_eq!(slugify("hello world", "", ".", None), "hello.world");
        assert_eq!(slugify("hello world", "", "_", None), "hello_world");
        assert_eq!(slugify!("hello world", separator = "_"), "hello_world");
    }

    #[test]
    fn test_phonetic_conversion() {
        assert_eq!(slugify("影師嗎", "", "-", None), "ying-shi-ma");
    }

    #[test]
    fn test_accented_text() {
        assert_eq!(slugify("Æúű--cool?", "", "-", None), "aeuu-cool");
        assert_eq!(slugify("Nín hǎo. Wǒ shì zhōng guó rén", "", "-", None),
                   "nin-hao-wo-shi-zhong-guo-ren");
    }

    #[test]
    fn test_accented_text_non_word_chars() {
        assert_eq!(slugify!("jaja---lol-méméméoo--a"), "jaja-lol-mememeoo-a");
    }

    #[test]
    fn test_cyrillic_text() {
        assert_eq!(slugify!("Компьютер"), "komp-iuter");
    }

    #[test]
    fn test_macro() {
        assert_eq!(slugify!("Компьютер"), "komp-iuter");
        assert_eq!(slugify!("hello world", separator = "-"), "hello-world");
        assert_eq!(slugify!("hello world", separator = " "), "hello world");
        assert_eq!(slugify!("hello world", max_length = 5), "hello");
        assert_eq!(slugify!("hello world", max_length = 6), "hello");
        assert_eq!(slugify!("hello world", separator = " ", max_length = 8),
                   "hello wo");
        assert_eq!(slugify!("hello world", separator = "x", max_length = 8),
                   "helloxwo");
        assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"),
                   "hello-world");
        assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5),
                   "hello");
        assert_eq!(slugify!("the hello world",
                            stop_words = "the",
                            separator = "-",
                            max_length = 10),
                   "hello-worl");
        assert_eq!(slugify!("the hello world",
                            stop_words = "the",
                            separator = "-",
                            max_length = 20),
                   "hello-world");
    }
}
