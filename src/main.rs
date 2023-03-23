mod lexer;

fn main() {
    println!("{:?}", lexer::tokenize("asd = 123 + 345 - 4353 ;").0);
}
