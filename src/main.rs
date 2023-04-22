use pest::Parser;

mod parser;
mod ast;

fn main() {
    //println!("{:?}", lexer::tokenize("asd = 123 + 345 - 4353 ;").0);
    let program = "asd = 124 + 345 - 4353;";
    let res = parser::Parser::parse(parser::Rule::prog, &program).unwrap();
    for xd in res {
        println!("{xd:?} \n");
    }
}
