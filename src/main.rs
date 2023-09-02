use regex::Regex;
use std::io::BufRead;

fn read_lines() -> Result<Vec<String>, std::io::Error> {
    let mut lines: Vec<String> = Vec::new();

    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(line) => {
                if line.trim().is_empty() {
                    break;
                }

                lines.push(line);
            }
            Err(error) => return Err(error),
        }
    }

    Ok(lines)
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
    let lines = read_lines().unwrap();

    let mut parsed_lines: Vec<String> = Vec::new();
    for line in &lines {
        let parsed = parse(&line);
        parsed_lines.push(parsed);
    }

    let output = parsed_lines.join(" ");

    println!("{:?}", output);
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
