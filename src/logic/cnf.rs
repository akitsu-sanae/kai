use super::theory::*;

#[derive(Debug, Clone)]
pub enum Term {
    Var(String),
    App(Func, Vec<Term>),
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub pred: Pred,
    pub args: Vec<Term>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Pos(Atom),
    Neg(Atom),
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug, Clone)]
pub struct Cnf {
    pub clauses: Vec<Clause>,
}
