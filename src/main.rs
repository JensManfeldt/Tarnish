use pest::Parser;

mod parser;

fn main() {
    //println!("{:?}", lexer::tokenize("asd = 123 + 345 - 4353 ;").0);
    let program = "asd == 123 + 345 - 4353";
    let res = parser::Parser::parse(parser::Rule::exp, &program).unwrap();
    for xd in res {
        println!("{xd:?}");
    }
}
