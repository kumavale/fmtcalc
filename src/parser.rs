use super::ast::{Ast, AstKind};

pub fn gen(node: &Option<Box<Ast>>, mut vec: &mut Vec<f64>) {
    if let AstKind::NUM(num) = node.as_ref().unwrap().kind {
        vec.push(num);
        return;
    }

    gen(&node.as_ref().unwrap().lnode, &mut vec);
    gen(&node.as_ref().unwrap().rnode, &mut vec);

    let i = vec.pop().unwrap();
    let j = vec.pop().unwrap();

    match node.as_ref().unwrap().kind {
        AstKind::ADD => vec.push(j+i),
        AstKind::SUB => vec.push(j-i),
        AstKind::MUL => vec.push(j*i),
        AstKind::DIV => {
            if i == 0.0 { panic!("Divide by Zero"); }
            vec.push(j/i);
        },
        AstKind::REM => {
            if i == 0.0 { panic!("Divide by Zero"); }
            vec.push(j%i);
        }
        AstKind::NUM(_) => unreachable!(),
    }
}

