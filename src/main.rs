use clap::Parser;

#[derive(Parser)]
struct Input {
    text: String,
}

fn parse(text: &str) -> String {
    if text.starts_with('#') {
        let text_without_start_sharp = &text[1..];
        let text_without_start_white_spaces = text_without_start_sharp.trim_start();
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
fn test_parse_single_line_includes_sharp_and_white_space() {
    let actual = parse("# Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}
