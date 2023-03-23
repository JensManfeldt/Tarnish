use std::collections::HashMap;

#[derive(Debug)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Mul,
    Div,
    Assign,
    Eq,
    Gt,
    Geq,
    Lt,
    Leq,

    Fn,
    LeftBrack,
    RightBrack,
    LeftParen,
    RightParen,
    Return,
    Id(Id),

    If,
    Else,
    Print,
    Semicolon,
}

/// Note deal with lifetime lazy
pub struct IdentMap {
    idents: Vec<String>,
    ids: HashMap<String, Id>,
}

impl IdentMap {
    fn add_ident(&mut self, ident: &str) -> Id {
        let id = Id(self.idents.len() as u64);
        self.ids.insert(ident.to_string(), id);
        self.idents.push(ident.to_string());
        id
    }

    pub fn get_ident(&self, id: Id) -> Option<&str> {
        self.idents.get(id.0 as usize).map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Id(u64);

pub fn tokenize(program: &str) -> (Vec<Token>, IdentMap) {
    let mut tokens = vec![];
    let mut ident_map = IdentMap {
        idents: vec![],
        ids: HashMap::new(),
    };

    for token in program.split_whitespace() {
        let token = match token {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Mul,
            "/" => Token::Div,
            "=" => Token::Assign,
            "==" => Token::Eq,
            ">" => Token::Gt,
            ">=" => Token::Geq,
            "<" => Token::Lt,
            "<=" => Token::Leq,
            "(" => Token::LeftParen,
            ")" => Token::RightParen,
            "{" => Token::LeftBrack,
            "}" => Token::RightBrack,
            "return" => Token::Return,
            ";" => Token::Semicolon,
            "if" => Token::If,
            "else" => Token::Else,
            "fn" => Token::Fn,
            "print" => Token::Print,
            _ => match token.parse() {
                Ok(i) => Token::Int(i),
                Err(_) => Token::Id(ident_map.add_ident(token)),
            },
        };
        tokens.push(token);
    }

    (tokens, ident_map)
}
