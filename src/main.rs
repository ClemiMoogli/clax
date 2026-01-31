use crate::parse::parse_token_stream;

mod parse;

fn main() {
    let input = std::fs::read_to_string("example.clax").unwrap();
    let stream = parse_token_stream(&input).unwrap();
    println!("{:?}", stream)
}
