# Slugify

[![Build Status](https://travis-ci.org/mattgathu/slugify.svg?branch=master)](https://travis-ci.org/mattgathu/slugify)
[![Build status](https://ci.appveyor.com/api/projects/status/tytk45snx8aqoctt?svg=true)](https://ci.appveyor.com/project/mattgathu/slugify)
[![Crates.io](https://img.shields.io/crates/v/slugify.svg)](https://crates.io/crates/slugify)
[![docs.rs](https://docs.rs/slugify/badge.svg)](https://docs.rs/slugify)
[![License: MIT](https://img.shields.io/crates/l/slugify.svg)](LICENSE)

**A utility macro for flexible slug genereation that handles unicode.**

The `slugify!` macro implements a flexible slug generator, allowing for stop words, custom separator
and maximum length options. The macro provides both a simple call format that has sane default parameters
but also allows there default parameters to be overriden when needed.

Features:
* Unicode strings support (phonetic conversion).
* Support for custom slug separator.
* Stop words filtering.
* Slug maximum length support.

 

 # Usage

This crate is on crates.io and can be used by adding `slugify` to the dependencies in your project's
`Cargo.toml`

 ```toml
 [dependencies]
 slugify = "0.1.0"
 ```
 
 and this to your crate root:
 
 ```rust
 #[macro_use] extern  crate  slugify;
use slugify::slugify;
```

 # Examples

## Basic slug generation

 ```rust
assert_eq!(slugify!("hello world"), "hello-world");

 ```
 
## Using a custom separator

 ```rust
 assert_eq!(slugify!("hello world", separator = "."), "hello.world");
 assert_eq!(slugify!("hello world", separator = " "), "hello world");
 ```
 
## Stop words filtering
 
```rust
assert_eq!(slugify!("the quick brown fox jumps over the lazy dog", stop_words = "the,fox"), "quick-brown-jumps-over-lazy-dog");
``` 
 
## Maximum slug length 
 
```rust
assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
 ```

## Phonetic Conversion and accented text

 ```rust
 assert_eq!(slugify!("影師嗎"), "ying-shi-ma");
 assert_eq!(slugify!("Æúű--cool?"), "aeuu-cool");
 assert_eq!(slugify!("Nín hǎo. Wǒ shì zhōng guó rén"), "nin-hao-wo-shi-zhong-guo-ren");
 ``` 
 
## Passing multiple optional parameters. 
 **NOTE:** the order of optional parameters matters: stop_words, separator
 and then max_length. All of them are optional, however when specifying more than one optional parameter, this
 order must be adhered.
 
 ```rust
assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-"), "hello-world");
assert_eq!(slugify!("the hello world", separator = ".", max_length = 10), "the.hello");
assert_eq!(slugify!("the hello world", stop_words = "the", max_length = 5), "hello");
assert_eq!(slugify!("the hello world", stop_words = "the", separator = "-", max_length = 20), "hello-world");
 ``` 
 



