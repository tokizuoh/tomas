use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Input {
    text: String,
}

fn parse(text: &str) -> String {
    if text.starts_with('#') {
        let re = Regex::new(r"^#+").unwrap();
        let text_without_start_sharps = re.replace(text, "");
        let text_without_start_white_spaces = text_without_start_sharps.trim_start();
        text_without_start_white_spaces.to_string()
    } else if text.starts_with("//") {
        let re = Regex::new(r"^/+").unwrap();
        let text_without_start_slashes = re.replace(text, "");
        let text_without_start_white_spaces = text_without_start_slashes.trim_start();
        text_without_start_white_spaces.to_string()
    } else {
        // TODO: except '#'
        text.to_string()
    }
}

fn main() {
    let args: Input = Input::parse();
    let output = parse(&args.text);
    println!("{}", output);
}

#[test]
fn test_parse_single_line_includes_sharp() {
    let actual = parse("#Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_multiple_sharps() {
    let actual = parse("#####Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_sharp_and_white_space() {
    let actual = parse("# Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_multiple_sharps_and_white_space() {
    let actual = parse("##### Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_multiple_slashes() {
    let actual = parse("//Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_multiple_slashes_and_white_spaces() {
    let actual = parse("// Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}

#[test]
fn test_parse_single_line_includes_multiple_slashes_and_white_spaces_2() {
    let actual = parse("/// Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}
