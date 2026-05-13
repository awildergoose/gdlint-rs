#![allow(clippy::missing_errors_doc)]

use std::path::PathBuf;

use clap::Parser;
use type_sitter::Node;

mod gdscript;
mod visitor;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
    #[arg(short, long, default_value = "false")]
    test_all: bool,
}

pub struct Traverser {
    pub source: String,
}

#[macro_export]
macro_rules! wrap_err {
    ($v:expr, $($text:expr),+) => {
        $v.map_err(|_| anyhow::anyhow!($($text),+))
    };
}

#[macro_export]
macro_rules! str_of_node {
    ($self:expr, $node:expr) => {
        &$self.source[$node.byte_range()]
    };
}

macro_rules! traverse_cases {
    ($self:expr, $node:ident, $($name:ident),+) => {
        match $node.kind() {
            $(
                <$name>::KIND => {
                    #[allow(unused_variables)]
                    let $node = $node.downcast::<$name>().unwrap();
                    paste::paste! {
                        $self.[<visit_ $name:snake>]($node)?;
                    }
                }
            ),+
            _ => {
                println!("warn: unknown node type: {:?} kind: {:?}", $node, $node.kind());
            }
        }
    };
}

impl Traverser {
    fn emit_error(text: &str) {
        eprintln!("error: {text}");
    }

    fn emit_warn(text: &str) {
        println!("warn: {text}");
    }

    #[allow(clippy::too_many_lines)]
    fn traverse<'tree, T>(&self, node: &T) -> anyhow::Result<()>
    where
        T: type_sitter::Node<'tree>,
    {
        let mut cursor = node.walk();

        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();

                {
                    use crate::gdscript::{
                        symbols::{
                            Add, AddEq, And as AndSymbol, AndAnd, AndDoubleQuote, AndEq, At,
                            BitNot, BitXor, BitXorDoubleQuote, BitXorEq, Colon, ColonEq, Comma,
                            Div, DivEq, Dollar, Dot, DotDotDot, DoubleQuote, Eq, EqEq, Gt, GtEq,
                            GtGt, GtGtEq, Hashregion, LBrace, LBracket, LParen, LParenRParen, Lt,
                            LtEq, LtLt, LtLtEq, Mod, ModEq, Mul, MulEq, MulMul, MulMulEq,
                            Not as NotSymbol, NotEq, Or as OrSymbol, OrEq, OrOr, RBrace, RBracket,
                            RParen, Semicolon, Sub, SubEq, SubGt,
                        },
                        unnamed::{
                            And, As, Await, Break, Class, ClassName, Const, Continue, Elif, Else,
                            Enum, Export, Extends, For, Func, Get, If, In, Init, Is, Master,
                            Mastersync, Match, Not, Onready, Or, Pass, Puppet, Puppetsync, Remote,
                            Remotesync, Return, Set, Signal, Value, Var, When, While,
                        },
                        Annotation, Annotations, Arguments, Array, Assignment, Attribute,
                        AttributeCall, AttributeSubscript, AugmentedAssignment, AwaitExpression,
                        BaseCall, BinaryOperator, Body, BreakStatement, BreakpointStatement, Call,
                        ClassBody, ClassDefinition, ClassNameStatement, Comment,
                        ConditionalExpression, ConstStatement, ConstructorDefinition,
                        ContinueStatement, DefaultParameter, Dictionary, ElifClause, ElseClause,
                        EnumDefinition, Enumerator, EnumeratorList, EscapeSequence,
                        ExportVariableStatement, ExpressionStatement, ExtendsStatement, False,
                        Float, ForStatement, FunctionDefinition, GetBody, GetNode, Getter,
                        Identifier, IfStatement, InferredType, Integer, Lambda, LineContinuation,
                        MatchBody, MatchStatement, Name, NodePath, Null, OnreadyVariableStatement,
                        Pair, Parameters, Parameters_, ParenthesizedExpression, PassStatement,
                        PatternBinding, PatternGuard, PatternOpenEnding, PatternSection, RegionEnd,
                        RegionLabel, RegionStart, RemoteKeyword, ReturnStatement, SetBody, Setget,
                        Setter, SignalStatement, Source, StaticKeyword, String as GDString,
                        StringName, Subscript, SubscriptArguments, True, Type,
                        TypedDefaultParameter, TypedParameter, UnaryOperator, VariableStatement,
                        VariadicParameter, WhileStatement,
                    };

                    traverse_cases! {
                        self,
                        node,
                        Add, AddEq, And, AndSymbol, AndAnd, AndDoubleQuote, AndEq, Annotation, Annotations, Arguments, Array, As, Assignment, At, Attribute, AttributeCall, AttributeSubscript, AugmentedAssignment, Await, AwaitExpression,
                        BaseCall, BinaryOperator, BitNot, BitXor, BitXorDoubleQuote, BitXorEq, Body, Break, BreakpointStatement, BreakStatement,
                        Call, Class, ClassBody, ClassDefinition, ClassName, ClassNameStatement, Colon, ColonEq, Comma, Comment, ConditionalExpression, Const, ConstructorDefinition, ConstStatement, Continue, ContinueStatement,
                        DefaultParameter, Dictionary, Div, DivEq, Dollar, Dot, DotDotDot, DoubleQuote,
                        Elif, ElifClause, Else, ElseClause, Enum, EnumDefinition, Enumerator, EnumeratorList, Eq, EqEq, EscapeSequence, Export, ExportVariableStatement, ExpressionStatement, Extends, ExtendsStatement,
                        False, Float, For, ForStatement, Func, FunctionDefinition,
                        GDString, Get, GetBody, GetNode, Getter, Gt, GtEq, GtGt, GtGtEq,
                        Hashregion,
                        Identifier, If, IfStatement, In, InferredType, Init, Integer, Is,
                        Lambda, LBrace, LBracket, LineContinuation, LParen, LParenRParen, Lt, LtEq, LtLt, LtLtEq,
                        Master, Mastersync, Match, MatchBody, MatchStatement, Mod, ModEq, Mul, MulEq, MulMul, MulMulEq,
                        Name, NodePath, Not, NotSymbol, NotEq, Null,
                        Onready, OnreadyVariableStatement, Or, OrSymbol, OrEq, OrOr,
                        Pair, Parameters_, Parameters, ParenthesizedExpression, Pass, PassStatement, PatternBinding, PatternGuard, PatternOpenEnding, PatternSection, Puppet, Puppetsync,
                        RBrace, RBracket, RegionEnd, RegionLabel, RegionStart, Remote, RemoteKeyword, Remotesync, Return, ReturnStatement, RParen,
                        Semicolon, Set, SetBody, Setget, Setter, Signal, SignalStatement, Source, StaticKeyword, StringName, Sub, SubEq, SubGt, Subscript, SubscriptArguments,
                        True, Type, TypedDefaultParameter, TypedParameter,
                        UnaryOperator,
                        Value, Var, VariableStatement, VariadicParameter,
                        When, While, WhileStatement
                    }
                }

                self.traverse(&node)?;
                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            cursor.goto_parent();
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    fn lint_code(code: String) -> anyhow::Result<()> {
        let mut parser =
            type_sitter::Parser::<gdscript::Source>::new(&tree_sitter_gdscript::LANGUAGE.into())
                .unwrap();

        let traverser = Traverser {
            source: code.clone(),
        };
        let tree = parser.parse(code, None).unwrap();
        assert!(tree.root_node().is_ok());

        let tree = Box::leak(Box::new(tree));
        let root_node = tree.root_node()?;
        traverser.traverse(&root_node)?;

        Ok(())
    }

    let cli = Cli::parse();
    let mut files = vec![];

    if cli.test_all {
        for f in std::fs::read_dir("tests").unwrap() {
            files.push(f.unwrap().path());
        }
    } else {
        files.push(cli.path);
    }

    for file in files {
        println!("Processing {}", file.display());

        let code = std::fs::read_to_string(file)?;

        let stack_size = 32 * 1024 * 1024;
        let builder = std::thread::Builder::new().stack_size(stack_size);
        let handler = builder.spawn(move || -> anyhow::Result<()> {
            lint_code(code)?;
            Ok(())
        })?;
        handler.join().unwrap()?;
    }

    Ok(())
}
