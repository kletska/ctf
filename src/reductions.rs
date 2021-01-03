use crate::internal_representation::AST;

pub fn name_change(tree: &AST, name: String, new_name: String) -> AST {
    todo!()
}

pub fn alpha_reduction(tree: &AST, new_name: String) -> Option<AST> {
    match tree {
        AST::Lam(arg, body) => todo!(),
        _ => None,
    }
}

fn beta_reduction() {
}

fn gamma_reduction() {
}
