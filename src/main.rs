use clap::Parser;

#[derive(Parser)]
struct Input {
    text: String,
}

fn parse(text: &str) -> String {
    text.to_string()
}

fn main() {
    let args: Input = Input::parse();
}

#[test]
fn test_parse_single_line() {
    let actual = parse("# Hello");
    let expected = "Hello";
    assert_eq!(actual, expected)
}
