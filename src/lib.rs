extern crate unidecode;
use unidecode::unidecode;

macro_rules! slugify {
    ($exp:expr) => (
        {
         slugify($exp, vec![], "-")
        }
    );
}

pub fn slugify(string: &str, stop_words: Vec<&str>, sep: &str) -> String {
    let char_vec: Vec<char> = sep.chars().collect();
    let mut string: String = unidecode(string.into()).to_lowercase().trim().trim_matches(char_vec[0]).replace(' ', &sep.to_string());

    // remove stop words
    for word in stop_words {
        string = string.replace(word, &sep.to_string());
    }
    // trim separator
    let mut slug = Vec::with_capacity(string.len());

    let mut is_sep = true;

    for x in string.chars() {
        match x {
            'a'...'z' | '0'...'9' => {
                is_sep = false;
                slug.push(x as u8);
            },
            _ => {
                if !is_sep {
                    is_sep = true;
                    slug.push(char_vec[0] as u8);
                } else {  }
            },
        }
    }

    if slug.last() == Some(&(char_vec[0] as u8)) {
        slug.pop();
    }

    String::from_utf8(slug).unwrap()

}



#[cfg(test)]
mod tests {
    use slugify;
    #[test]
    fn it_works() {
        assert_eq!(slugify("hello world", vec![], "-"), "hello-world".to_string());
        assert_eq!(slugify("hello world-", vec![], "-"), "hello-world".to_string());
        assert_eq!(slugify("hello world ", vec![], "-"), "hello-world".to_string());
    }

    #[test]
    fn test_email(){
        assert_eq!(slugify!("alice@bob.com"), "alice-bob-com");
    }

    #[test]
    fn test_starts_with_number(){
        assert_eq!(slugify!("10 amazing secrets"), "10-amazing-secrets");
    }

    #[test]
    fn test_contains_numbers(){
        assert_eq!(slugify!("the 101 dalmatians"), "the-101-dalmatians");
    }

    #[test]
    fn test_ends_with_number(){
        assert_eq!(slugify!("lucky number 7"), "lucky-number-7");
    }

    #[test]
    fn test_numbers_only(){
        assert_eq!(slugify!("101"), "101");
    }

    #[test]
    fn test_numbers_and_symbols(){
        assert_eq!(slugify!("1000 reasons you are #1"), "1000-reasons-you-are-1");
    }

    #[test]
    fn test_stop_words(){
        assert_eq!(slugify("hello world", vec!["world"], "-"), "hello");
    }

    #[test]
    fn test_differently_cased_stopword_match(){
        assert_eq!(slugify("Foo A FOO B foo C", vec!["foo"], "-"), "a-b-c");
    }

    #[test]
    fn test_multiple_stop_words(){
        assert_eq!(slugify("the quick brown fox jumps over the lazy dog", vec!["the"], "-"), "quick-brown-fox-jumps-over-lazy-dog");
    }

    #[test]
    fn test_stopwords_with_different_separator(){
        assert_eq!(slugify("the quick brown fox jumps over the lazy dog", vec!["the"], " "), "quick brown fox jumps over lazy dog");
    }

    #[test]
    fn test_separator(){
        assert_eq!(slugify("hello world", vec![], "."), "hello.world");
    }

    #[test]
    fn test_phonetic_conversion() {
        assert_eq!(slugify("影師嗎", vec![], "-"), "ying-shi-ma");
    }

    #[test]
    fn test_accented_text() {
        assert_eq!(slugify("Æúű--cool?", vec![], "-"), "aeuu-cool");
        assert_eq!(slugify("Nín hǎo. Wǒ shì zhōng guó rén", vec![], "-"), "nin-hao-wo-shi-zhong-guo-ren");
    }

    #[test]
    fn test_accented_text_non_word_chars() {
        assert_eq!(slugify("jaja---lol-méméméoo--a", vec![], "-"), "jaja-lol-mememeoo-a");
    }

    #[test]
    fn test_cyrillic_text() {
        assert_eq!(slugify("Компьютер", vec![], "-"), "komp-iuter");
    }

    #[test]
    fn test_macro() {
        assert_eq!(slugify!("Компьютер"), "komp-iuter");
    }
}
