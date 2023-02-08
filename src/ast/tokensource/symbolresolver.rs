use std::rc::Rc;

use once_cell::sync::Lazy;
use strum::IntoEnumIterator;

use crate::ast::symbol::{Symbol, SymbolDiscriminants};
use crate::ast::tokensource::tokenparser::{Node, NodeBuilder};
use crate::module::{Token, TokenTypeDiscriminants};

pub trait SymbolResolverType {
    fn resolve(self: Rc<Self>, tokens: &[&Token]) -> Symbol;
}

pub type SymbolResolver = Rc<dyn SymbolResolverType>;

const ROOT_NODE: Lazy<Node> = Lazy::new(|| {
    let mut rootNode = Node::new();
    for symbol in SymbolDiscriminants::iter() {
        rootNode.mergeNode(getNodeMatchForSymbol(symbol));
    }
    return rootNode;
});

pub fn getRootNode() -> Node {
    return ROOT_NODE.to_owned();
}

fn addExprNode(builder: &mut NodeBuilder) {
    // variable
    // parenthesis
    // block
    // operator
    // function call
    todo!()
}

fn getNodeMatchForSymbol(symbol: SymbolDiscriminants) -> Node {
    return match symbol {
        SymbolDiscriminants::Block => {
            // { instructions }
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Parenthesis
            ], getSymbolResolverFor(SymbolDiscriminants::Block))
        }
        SymbolDiscriminants::Break => {
            // break
            // break name
            let mut node = Node::newWithResolved(&[
                TokenTypeDiscriminants::Keyword, // break
            ], getSymbolResolverFor(SymbolDiscriminants::Break));
            node.addNode(TokenTypeDiscriminants::Identifier, Node::new()).addResolver(getSymbolResolverFor(SymbolDiscriminants::Break));
            node
        }
        SymbolDiscriminants::ClassDefinition => {
            // class name { definition }
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Keyword, // class name
                TokenTypeDiscriminants::Identifier, // function name
                TokenTypeDiscriminants::Parenthesis, // definition
            ], getSymbolResolverFor(SymbolDiscriminants::ClassDefinition))
        }
        SymbolDiscriminants::FunctionDefinition => {
            // returntype name(args) { instructions }
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Identifier, // return type
                TokenTypeDiscriminants::Identifier, // function name
                TokenTypeDiscriminants::Parenthesis, // args
                TokenTypeDiscriminants::Parenthesis, // definition
            ], getSymbolResolverFor(SymbolDiscriminants::FunctionDefinition))
        }
        SymbolDiscriminants::IfSym => {
            // if expr instruction
            // if expr instruction else instruction
            let mut node = Node::newWith(&[
                TokenTypeDiscriminants::Keyword, // if
            ]);
            let mut builder = NodeBuilder::new(&mut node);
            addExprNode(&mut builder); // instruction
            builder.addResolver(getSymbolResolverFor(SymbolDiscriminants::FunctionDefinition));
            builder.addToken(TokenTypeDiscriminants::Keyword); // else
            addExprNode(&mut builder); // instruction
            node
        }
        SymbolDiscriminants::ImportSym => {
            // import name
            // import name as importedName
            let mut node = Node::newWithResolved(&[
                TokenTypeDiscriminants::Keyword,
                TokenTypeDiscriminants::Identifier
            ], getSymbolResolverFor(SymbolDiscriminants::ImportSym));
            let mut builder = NodeBuilder::new(&mut node);
            builder.addToken(TokenTypeDiscriminants::Keyword);
            builder.addToken(TokenTypeDiscriminants::Identifier);
            builder.addResolver(getSymbolResolverFor(SymbolDiscriminants::ImportSym));
            node
        }
        SymbolDiscriminants::FunctionCall => {
            // functionName(args)
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Identifier,
                TokenTypeDiscriminants::Parenthesis
            ], getSymbolResolverFor(SymbolDiscriminants::FunctionCall))
        }
        SymbolDiscriminants::Operator => {
            // operator expr
            // operator expr operator
            // expr operator
            todo!()
        }
        SymbolDiscriminants::Parenthesis => {
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Parenthesis
            ], getSymbolResolverFor(SymbolDiscriminants::Parenthesis))
        }
        SymbolDiscriminants::VariableDeclaration => {
            // let value
            // type value
            let mut node = Node::new();
            let mut builder = NodeBuilder::new(&mut node);

            builder.mergeNode({
                let mut inferred = Node::newWith(&[
                    TokenTypeDiscriminants::Keyword,
                    TokenTypeDiscriminants::Identifier,
                ]);
                inferred.addResolver(getSymbolResolverFor(SymbolDiscriminants::VariableDeclaration));
                inferred
            });
            builder.mergeNode({
                let mut explicit = Node::newWith(&[
                    TokenTypeDiscriminants::Keyword,
                    TokenTypeDiscriminants::Identifier,
                ]);
                explicit.addResolver(getSymbolResolverFor(SymbolDiscriminants::VariableDeclaration));
                explicit
            });

            node
        }
        SymbolDiscriminants::Variable => {
            // name
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Identifier,
            ], getSymbolResolverFor(SymbolDiscriminants::Variable))
        }
        SymbolDiscriminants::LiteralArray => {
            // [a, b, c]
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Parenthesis,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralArray))
        }
        SymbolDiscriminants::LiteralBool => {
            // true, false
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Keyword,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralBool))
        }
        SymbolDiscriminants::LiteralChar => {
            // 'c'
            Node::newWithResolved(&[
                TokenTypeDiscriminants::String,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralChar))
        }
        SymbolDiscriminants::LiteralFixed => {
            // 0.0
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Number,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralFixed))
        }
        SymbolDiscriminants::LiteralInteger => {
            // 0
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Number,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralInteger))
        }
        SymbolDiscriminants::LiteralString => {
            // "c"
            Node::newWithResolved(&[
                TokenTypeDiscriminants::String,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralString))
        }
        SymbolDiscriminants::LiteralVoid => {
            // void
            Node::newWithResolved(&[
                TokenTypeDiscriminants::Keyword,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralVoid))
        }
        SymbolDiscriminants::LiteralTuple => {
            // a,b,c
            Node::newWithResolved(&[
                TokenTypeDiscriminants::CommaList,
            ], getSymbolResolverFor(SymbolDiscriminants::LiteralTuple))
        }
    };
}

fn getSymbolResolverFor(symbol: SymbolDiscriminants) -> SymbolResolver {
    return match symbol {
        SymbolDiscriminants::Block => {
            todo!()
        }
        SymbolDiscriminants::Break => {
            todo!()
        }
        SymbolDiscriminants::ClassDefinition => {
            todo!()
        }
        SymbolDiscriminants::FunctionDefinition => {
            todo!()
        }
        SymbolDiscriminants::IfSym => {
            todo!()
        }
        SymbolDiscriminants::ImportSym => {
            todo!()
        }
        SymbolDiscriminants::FunctionCall => {
            todo!()
        }
        SymbolDiscriminants::Operator => {
            todo!()
        }
        SymbolDiscriminants::Parenthesis => {
            todo!()
        }
        SymbolDiscriminants::VariableDeclaration => {
            todo!()
        }
        SymbolDiscriminants::Variable => {
            todo!()
        }
        SymbolDiscriminants::LiteralArray => {
            todo!()
        }
        SymbolDiscriminants::LiteralBool => {
            todo!()
        }
        SymbolDiscriminants::LiteralChar => {
            todo!()
        }
        SymbolDiscriminants::LiteralFixed => {
            todo!()
        }
        SymbolDiscriminants::LiteralInteger => {
            todo!()
        }
        SymbolDiscriminants::LiteralString => {
            todo!()
        }
        SymbolDiscriminants::LiteralVoid => {
            todo!()
        }
        SymbolDiscriminants::LiteralTuple => {
            todo!()
        }
    };
}
