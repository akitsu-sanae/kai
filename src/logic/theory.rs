#[derive(Debug, Clone)]
pub enum Sort {
    Bool,
    Int,
}

#[derive(Debug, Clone)]
pub enum Pred {
    BoolConst(bool),
    Equal,
    Less,
    Greater,
}

#[derive(Debug, Clone)]
pub enum Func {
    Pred(Pred),
    IntConst(i32),
    Add,
    Sub,
}
