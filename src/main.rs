use clap::Parser;

#[derive(Parser)]
struct Input {
    text: String,
}
fn main() {
    let args: Input = Input::parse();
}
}
