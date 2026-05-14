use type_sitter::Node;

use crate::astgen::ast::GDNode;

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

pub struct Traverser {
    pub source: String,
    pub root: GDNode,
}

impl Traverser {
    #[allow(clippy::too_many_lines)]
    pub fn traverse<'tree, T>(&mut self, node: &T) -> anyhow::Result<()>
    where
        T: type_sitter::Node<'tree>,
    {
        let mut cursor = node.walk();

        if cursor.goto_first_child() {
            loop {
                let node = cursor.node();

                {
                    use crate::astgen::gdscript::{
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

                // self.traverse(&node)?;
                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            cursor.goto_parent();
        }

        Ok(())
    }
}
