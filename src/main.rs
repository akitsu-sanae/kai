mod logic;

fn main() {
    use logic::{cnf::*, theory::*};
    println!(
        "{:?}",
        Cnf {
            clauses: vec![Clause {
                literals: vec![Literal::Pos(Atom {
                    pred: Pred::Equal,
                    args: vec![
                        Term::Var("a".to_string()),
                        Term::App(Func::Pred(Pred::BoolConst(true)), vec![]),
                    ],
                })],
            }],
        }
    );
}
