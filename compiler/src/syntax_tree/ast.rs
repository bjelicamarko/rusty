use crate::util::expression::Expression;

pub struct SyntaxTree {
    root: Box<dyn Expression>,
}

pub struct Pair {
    pub node: Box<dyn Expression>,
    pub level: usize,
}

impl SyntaxTree {
    pub fn new(root: Box<dyn Expression>) -> Self {
        Self { root }
    }

    pub fn print_tree(&self, show_tree: bool) {
        if show_tree {
            let mut stack = Vec::new();

            stack.push(Pair {
                node: self.root.clone(),
                level: 0,
            });

            loop {
                if stack.is_empty() {
                    break;
                }
                let pair = stack.pop().unwrap();
                let expr = pair.node;

                println!("{}{:?}", "   ".repeat(pair.level), expr.get_kind());

                for expression in expr.get_children() {
                    stack.push(Pair {
                        node: expression,
                        level: pair.level + 1,
                    })
                }
            }
        }
    }
}
