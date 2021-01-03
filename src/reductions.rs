use crate::internal_representation::AST;

fn name_change(tree: &AST, old_name: &String, new_name: &String) -> Option<AST> {
    match tree {
        AST::Sym(name) if name == old_name => Some(AST::Sym(new_name.clone())),
        AST::Sym(_name) => Some(tree.clone()),
        AST::Lam(arg, _body) if arg == old_name => Some(tree.clone()),
        AST::Lam(arg, _body) if arg == new_name => None,
        AST::Lam(arg, body) => Some(AST::Lam(arg.clone(), Box::new(name_change(body, old_name, new_name)?))),
        AST::App(func, args) => Some(AST::App(Box::new(name_change(func, old_name, new_name)?), Box::new(name_change(args, old_name, new_name)?))),
    }
}

pub fn alpha_reduction(tree: &AST, new_name: &String) -> Option<AST> {
    match tree {
        AST::Lam(arg, body) => Some(AST::Lam(new_name.clone(), Box::new(name_change(body, arg, new_name)?))),
        _ => None,
    }
}

fn variable_change(tree: &AST, variable: &String, value: &AST) -> AST {
    match tree {
        AST::Sym(name) if name == variable => value.clone(),
        AST::Sym(_name) => tree.clone(),
        AST::Lam(arg, _body) if arg == variable => tree.clone(),
        AST::Lam(arg, body) => AST::Lam(arg.clone(), Box::new(variable_change(body, variable, value))),
        AST::App(func, args) => AST::App(Box::new(variable_change(func, variable, value)), Box::new(variable_change(args, variable, value))),
    }
}

pub fn beta_reduction(tree: &AST, value: &AST) -> Option<AST> {
    match tree {
        AST::Lam(arg, body) => Some(variable_change(body, arg, value)),
        _ => None,
    }
}

fn gamma_reduction(tree: &AST) -> Option<AST> {
    match tree {
        AST::Lam(arg, body) => match &**body {
            AST::App(func, args) => match &**args {
                AST::Sym(variable) if arg == variable => Some((&**func).clone()),
                _ => None,
            }
            _ => None,
        }
        _ => None,
    }
}
