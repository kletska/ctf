pub enum RawLambda {
    Args(Vec<RawAST>),
    Full(Vec<RawAST>, Vec<RawAST>),
}

pub enum RawAST {
    Sym(String),
    Lam(RawLambda),
    App(Vec<RawAST>),
}

impl RawAST {
    pub fn push(&mut self, elem: RawAST) {
        match self {
            RawAST::Lam(RawLambda::Args(vec)) => vec.push(elem),
            RawAST::Lam(RawLambda::Full(args, vec)) => vec.push(elem),
            RawAST::App(vec) => vec.push(elem),
            _ => (),
        }
    }
}

pub enum AST {
    Sym(String),
    Lam(String, Box<AST>),
    App(Box<AST>, Box<AST>),
}

pub enum MachineAST {
    Sym(u64, Option<Box<MachineAST>>),
    Lam(u64, Box<MachineAST>),
    App(Box<MachineAST>, Box<MachineAST>)
}


