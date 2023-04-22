type Ident = &str;

pub enum Exp {
    BinOp {
        left: Box<Exp>,
        op: BinOp,
        right: Box<Exp>,
    },
    Ident(Ident),
    Int(i64),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Gt,
    Geq,
    Lt,
    Leq,
}

pub struct Block(Vec<Stm>);

pub enum Stm {
    Assign {
        var: Ident,
        exp: Exp,
    },
    Exp(Exp),
    If {
        cond: Exp,
        thn: Block,
        els: Option<Block>
    },
    Print(Exp),
}

pub struct Prog(Vec<Stm>);