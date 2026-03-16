use proc_macro2::Span;
use syn::{
    Block, Expr, Stmt,
    spanned::Spanned as _,
    visit::{self, Visit},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Genre {
    File,
    Block,
    Stmt,
    Expr,
}

#[derive(Debug)]
pub struct Node {
    pub parent: usize,
    pub span: Span,
    pub genre: Genre,
}
impl From<(Genre, Span, usize)> for Node {
    fn from((genre, span, parent): (Genre, Span, usize)) -> Self {
        Self { parent, span, genre }
    }
}

enum AstNode<'ast> {
    Block(&'ast syn::Block),
    Stmt(&'ast syn::Stmt),
    Expr(&'ast syn::Expr),
}

struct FlattenVisitor<'ast> {
    parent: usize,
    enfants: Vec<(Node, AstNode<'ast>)>,
    nodes: Vec<Node>,
}

impl<'ast> FlattenVisitor<'ast> {
    fn new() -> Self {
        FlattenVisitor {
            enfants: vec![],
            nodes: vec![(Genre::File, Span::call_site(), 0).into()],
            parent: 0,
        }
    }

    // fn flat_visit(&mut self) {
    //     loop {
    //         let parents_ast = std::mem::take(&mut self.enfants);
    //         let mut index = self.nodes.len();
    //         let mut parents = Vec::new();
    //         for (parent_node, ast) in parents_ast {
    //             self.parent = index;
    //             match ast {
    //                 // Generates its children
    //                 AstNode::Block(ast) => visit::visit_block(self, ast),
    //                 AstNode::Stmt(ast) => visit::visit_stmt(self, ast),
    //                 AstNode::Expr(ast) => visit::visit_expr(self, ast),
    //             }
    //             index += 1;
    //             parents.push(parent_node);
    //         }
    //         self.nodes.extend(parents);
    //         if self.enfants.len() == 0 {
    //             break;
    //         }
    //     }
    // }
    fn take_children_and_visit(&mut self) {
        // children become parents
        let children = std::mem::take(&mut self.enfants);
        for (child_node, ast) in children {
            let index = self.nodes.len();
            self.nodes.push(child_node);
            self.parent = index;
            match ast {
                // Generates its children
                AstNode::Block(ast) => visit::visit_block(self, ast),
                AstNode::Stmt(ast) => visit::visit_stmt(self, ast),
                AstNode::Expr(ast) => visit::visit_expr(self, ast),
            }
            if self.enfants.len() > 0 {
                // This child has become parent.
                // Let's handle its children to preserve the order.
                self.take_children_and_visit();
            }
        }
    }
    // #[allow(dead_code)]
    // fn sorted_range_nodes(self) -> Vec<NodeRange> {
    //     // We need to sort nodes to use it in cache.
    //     let mut nodes: Vec<(_, NodeRange)> = self
    //         .nodes
    //         .into_iter()
    //         .enumerate()
    //         .map(|(old_index, node)| (old_index, node.into()))
    //         .collect();
    //     nodes.sort_by(|(_, n_a), (_, n_b)| n_a.cmp(n_b));
    //     // Now that nodes are sorted we have to update the parent indexes
    //     // So first we gather the couple old/new of indexes
    //     let mut changes: Vec<_> = nodes
    //         .iter()
    //         .enumerate()
    //         .map(|(new_index, (old_index, _))| (old_index.clone(), new_index.clone()))
    //         .collect();
    //     // Then we sort these indexes to find them back easily
    //     changes.sort_by_cached_key(|indexes| indexes.0);
    //     // An finaly we update the indexes and return the node.
    //     nodes
    //         .into_iter()
    //         .map(|(_, mut node_range)| {
    //             let parent = &mut node_range.parent;
    //             *parent = changes[*parent].1;
    //             node_range
    //         })
    //         .collect()
    //     // Now we have sorted nodes, with ancerstors before children.
    // }
}

impl<'ast> Visit<'ast> for FlattenVisitor<'ast> {
    fn visit_block(&mut self, ast: &'ast Block) {
        // println!("Block");
        let parent = self.parent;
        let span = ast.brace_token.span;
        // println!("{span:?} ; {:?}", ast.span().byte_range());
        let node: Node = (Genre::Block, span.join(), parent).into();
        let node_ast = AstNode::Block(ast);
        self.enfants.push((node, node_ast));
        // println!();
    }

    fn visit_stmt(&mut self, ast: &'ast Stmt) {
        // println!("Stmt");
        let parent = self.parent;
        let span = ast.span();
        let node: Node = (Genre::Stmt, span, parent).into();
        let node_ast = AstNode::Stmt(ast);
        self.enfants.push((node, node_ast));
    }

    fn visit_expr(&mut self, ast: &'ast Expr) {
        // println!("Expr");
        let parent = self.parent;
        let span = ast.span();
        let node: Node = (Genre::Expr, span, parent).into();
        let node_ast = AstNode::Expr(ast);
        self.enfants.push((node, node_ast));
    }
}

pub fn parse_nodes(code: &[u8]) -> Vec<Node> {
    let ast = syn::parse_file(&str::from_utf8(code).unwrap()).unwrap();
    let mut flat_visitor = FlattenVisitor::new();
    flat_visitor.visit_file(&ast);
    // TODO: maybe some simple bench to see what's the best...
    // flat_visitor.parse_nodes_sort_during_collect()
    // flat_visitor.parse_nodes_sort_during_visit()
    flat_visitor.take_children_and_visit();
    flat_visitor.nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.parent == other.parent && self.genre == other.genre
        }
    }
    #[test]
    fn essai() {
        let content = r###"
        fn plop() {
            let x = 5;
            let y = 10;
            x + y
        }
    "###;
        // let content = std::fs::read_to_string("src/main.rs").unwrap();
        let ast = syn::parse_file(&content).unwrap();

        // Créer le visiteur et visiter le code
        let mut flat_visitor = FlattenVisitor::new();
        flat_visitor.visit_file(&ast);
        // flat_visitor.flat_visit();
        flat_visitor.take_children_and_visit();

        // Afficher les plages extraites
        for Node { parent, span, genre } in &flat_visitor.nodes {
            println!("{:?}: {:?} à {}", genre, span.source_text(), parent);
        }
        println!("{:#?}", flat_visitor.nodes);
        // assert_eq!(flat_visitor.nodes, vec![], "Error");
    }
}
