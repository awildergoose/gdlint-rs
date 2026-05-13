#![allow(clippy::missing_errors_doc)]
use clap::Parser;
use type_sitter::Node;

mod gdscript;

#[derive(Parser)]
struct Cli {
    path: String,
}

pub struct Traverser {
    pub source: String,
}

macro_rules! wrap_err {
    ($v:expr, $($text:expr),+) => {
        $v.map_err(|_| anyhow::anyhow!($($text),+))
    };
}

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
                println!("warn: unknown node type: {:?}", $node);
            }
        }
    };
}

impl Traverser {
    fn emit_error(&self, text: &str) {
        eprintln!("error: {text}");
    }

    fn emit_warn(&self, text: &str) {
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
                // println!("Child node: {}", node.kind());

                {
                    use gdscript::{
                        symbols::{
                            Add, AddEq, AndAnd, AndDoubleQuote, AndEq, At, BitNot, BitXor,
                            BitXorDoubleQuote, BitXorEq, Colon, ColonEq, Comma, Div, DivEq, Dollar,
                            Dot, DotDotDot, DoubleQuote, Eq, EqEq, Gt, GtEq, GtGt, GtGtEq,
                            Hashregion, LBrace, LBracket, LParen, LParenRParen, Lt, LtEq, LtLt,
                            LtLtEq, Mod, ModEq, Mul, MulEq, MulMul, MulMulEq, NotEq, OrEq, OrOr,
                            RBrace, RBracket, RParen, Semicolon, Sub, SubEq, SubGt,
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
                        Pair, Parameters, ParenthesizedExpression, PassStatement, PatternBinding,
                        PatternGuard, PatternOpenEnding, PatternSection, RegionEnd, RegionLabel,
                        RegionStart, RemoteKeyword, ReturnStatement, SetBody, Setter,
                        SignalStatement, Source, StaticKeyword, StringName, Subscript,
                        SubscriptArguments, True, Type, TypedDefaultParameter, TypedParameter,
                        UnaryOperator, VariableStatement, VariadicParameter, WhileStatement,
                    };

                    traverse_cases! {
                        self,
                        node,
                        Annotation,
                        Annotations,
                        Arguments,
                        Array,
                        Assignment,
                        Attribute,
                        AttributeCall,
                        AttributeSubscript,
                        AugmentedAssignment,
                        AwaitExpression,
                        BaseCall,
                        BinaryOperator,
                        Body,
                        BreakStatement,
                        Call,
                        ClassBody,
                        ClassDefinition,
                        ClassNameStatement,
                        ConditionalExpression,
                        ConstStatement,
                        ConstructorDefinition,
                        ContinueStatement,
                        DefaultParameter,
                        Dictionary,
                        ElifClause,
                        ElseClause,
                        EnumDefinition,
                        Enumerator,
                        EnumeratorList,
                        ExportVariableStatement,
                        ExpressionStatement,
                        ExtendsStatement,
                        ForStatement,
                        FunctionDefinition,
                        GetBody,
                        GetNode,
                        Getter,
                        Identifier,
                        IfStatement,
                        InferredType,
                        Lambda,
                        MatchBody,
                        MatchStatement,
                        Name,
                        NodePath,
                        OnreadyVariableStatement,
                        Pair,
                        Parameters,
                        ParenthesizedExpression,
                        PassStatement,
                        PatternBinding,
                        PatternGuard,
                        PatternSection,
                        RegionStart,
                        RemoteKeyword,
                        ReturnStatement,
                        SetBody,
                        // Setget,
                        Setter,
                        SignalStatement,
                        Source,
                        // String,
                        StringName,
                        Subscript,
                        SubscriptArguments,
                        Type,
                        TypedDefaultParameter,
                        TypedParameter,
                        UnaryOperator,
                        Value,
                        VariableStatement,
                        VariadicParameter,
                        WhileStatement,
                        Init,
                        And,
                        As,
                        Await,
                        Break,
                        BreakpointStatement,
                        Class,
                        ClassName,
                        Comment,
                        Const,
                        Continue,
                        Elif,
                        Else,
                        Enum,
                        EscapeSequence,
                        Export,
                        Extends,
                        False,
                        Float,
                        For,
                        Func,
                        Get,
                        If,
                        In,
                        Integer,
                        Is,
                        LineContinuation,
                        Master,
                        Mastersync,
                        Match,
                        Not,
                        Null,
                        Onready,
                        Or,
                        Pass,
                        PatternOpenEnding,
                        Puppet,
                        Puppetsync,
                        RegionEnd,
                        RegionLabel,
                        Remote,
                        Remotesync,
                        Return,
                        Set,
                        Signal,
                        StaticKeyword,
                        True,
                        Var,
                        When,
                        While,
                        LBrace,
                        OrEq,
                        OrOr,
                        RBrace,
                        BitNot,
                        Comma,
                        LParenRParen,
                        NotEq,
                        DoubleQuote,
                        Hashregion,
                        Dollar,
                        Mod,
                        ModEq,
                        AndDoubleQuote,
                        AndAnd,
                        AndEq,
                        LParen,
                        RParen,
                        Mul,
                        MulMul,
                        MulMulEq,
                        MulEq,
                        Add,
                        AddEq,
                        Sub,
                        SubEq,
                        SubGt,
                        Dot,
                        DotDotDot,
                        Div,
                        DivEq,
                        Colon,
                        ColonEq,
                        Semicolon,
                        Lt,
                        LtLt,
                        LtLtEq,
                        LtEq,
                        Eq,
                        EqEq,
                        Gt,
                        GtEq,
                        GtGt,
                        GtGtEq,
                        At,
                        LBracket,
                        RBracket,
                        BitXor,
                        BitXorDoubleQuote,
                        BitXorEq
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

mod visitor {
    use crate::{
        gdscript::{
            symbols::{
                Add, AddEq, AndAnd, AndDoubleQuote, AndEq, At, BitNot, BitXor, BitXorDoubleQuote,
                BitXorEq, Colon, ColonEq, Comma, Div, DivEq, Dollar, Dot, DotDotDot, DoubleQuote,
                Eq, EqEq, Gt, GtEq, GtGt, GtGtEq, Hashregion, LBrace, LBracket, LParen,
                LParenRParen, Lt, LtEq, LtLt, LtLtEq, Mod, ModEq, Mul, MulEq, MulMul, MulMulEq,
                NotEq, OrEq, OrOr, RBrace, RBracket, RParen, Semicolon, Sub, SubEq, SubGt,
            },
            unnamed::{
                And, As, Await, Break, Class, ClassName, Const, Continue, Elif, Else, Enum, Export,
                Extends, For, Func, Get, If, In, Init, Is, Master, Mastersync, Match, Not, Onready,
                Or, Pass, Puppet, Puppetsync, Remote, Remotesync, Return, Set, Signal, Value, Var,
                When, While,
            },
            Annotation, Annotations, Arguments, Array, Assignment, Attribute, AttributeCall,
            AttributeSubscript, AugmentedAssignment, AwaitExpression, BaseCall, BinaryOperator,
            Body, BreakStatement, BreakpointStatement, Call, ClassBody, ClassDefinition,
            ClassNameStatement, Comment, ConditionalExpression, ConstStatement,
            ConstructorDefinition, ContinueStatement, DefaultParameter, Dictionary, ElifClause,
            ElseClause, EnumDefinition, Enumerator, EnumeratorList, EscapeSequence,
            ExportVariableStatement, ExpressionStatement, ExtendsStatement, False, Float,
            ForStatement, FunctionDefinition, GetBody, GetNode, Getter, Identifier, IfStatement,
            InferredType, Integer, Lambda, LineContinuation, MatchBody, MatchStatement, Name,
            NodePath, Null, OnreadyVariableStatement, Pair, Parameters, ParenthesizedExpression,
            PassStatement, PatternBinding, PatternGuard, PatternOpenEnding, PatternSection,
            RegionEnd, RegionLabel, RegionStart, RemoteKeyword, ReturnStatement, SetBody, Setter,
            SignalStatement, Source, StaticKeyword, StringName, Subscript, SubscriptArguments,
            True, Type, TypedDefaultParameter, TypedParameter, UnaryOperator, VariableStatement,
            VariadicParameter, WhileStatement,
        },
        Traverser,
    };

    #[allow(clippy::unnecessary_wraps)]
    #[allow(clippy::missing_const_for_fn)]
    #[allow(clippy::unused_self)]
    #[allow(unused_variables)]
    impl Traverser {
        pub fn visit_l_paren_r_paren(&self, node: LParenRParen) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_not_eq(&self, node: NotEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_double_quote(&self, node: DoubleQuote) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_hashregion(&self, node: Hashregion) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_dollar(&self, node: Dollar) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mod(&self, node: Mod) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mod_eq(&self, node: ModEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_and_double_quote(&self, node: AndDoubleQuote) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_and_and(&self, node: AndAnd) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_and_eq(&self, node: AndEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_l_paren(&self, node: LParen) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_r_paren(&self, node: RParen) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mul(&self, node: Mul) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mul_mul(&self, node: MulMul) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mul_mul_eq(&self, node: MulMulEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mul_eq(&self, node: MulEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_add(&self, node: Add) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_add_eq(&self, node: AddEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_sub(&self, node: Sub) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_sub_eq(&self, node: SubEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_sub_gt(&self, node: SubGt) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_dot(&self, node: Dot) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_dot_dot_dot(&self, node: DotDotDot) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_div(&self, node: Div) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_div_eq(&self, node: DivEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_colon(&self, node: Colon) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_colon_eq(&self, node: ColonEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_semicolon(&self, node: Semicolon) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_lt(&self, node: Lt) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_lt_lt(&self, node: LtLt) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_lt_lt_eq(&self, node: LtLtEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_lt_eq(&self, node: LtEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_eq(&self, node: Eq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_eq_eq(&self, node: EqEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_gt(&self, node: Gt) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_gt_eq(&self, node: GtEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_gt_gt(&self, node: GtGt) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_gt_gt_eq(&self, node: GtGtEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_at(&self, node: At) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_l_bracket(&self, node: LBracket) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_r_bracket(&self, node: RBracket) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_bit_xor(&self, node: BitXor) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_bit_xor_double_quote(&self, node: BitXorDoubleQuote) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_bit_xor_eq(&self, node: BitXorEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_annotation(&self, node: Annotation) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_annotations(&self, node: Annotations) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_arguments(&self, node: Arguments) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_array(&self, node: Array) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_assignment(&self, node: Assignment) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_attribute(&self, node: Attribute) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_attribute_call(&self, node: AttributeCall) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_attribute_subscript(&self, node: AttributeSubscript) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_augmented_assignment(&self, node: AugmentedAssignment) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_await_expression(&self, node: AwaitExpression) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_base_call(&self, node: BaseCall) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_binary_operator(&self, node: BinaryOperator) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_body(&self, node: Body) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_break_statement(&self, node: BreakStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_call(&self, node: Call) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_class_body(&self, node: ClassBody) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_class_definition(&self, node: ClassDefinition) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_class_name_statement(&self, node: ClassNameStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_conditional_expression(
            &self,
            node: ConditionalExpression,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_const_statement(&self, node: ConstStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_constructor_definition(
            &self,
            node: ConstructorDefinition,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_continue_statement(&self, node: ContinueStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_default_parameter(&self, node: DefaultParameter) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_dictionary(&self, node: Dictionary) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_elif_clause(&self, node: ElifClause) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_else_clause(&self, node: ElseClause) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_enum_definition(&self, node: EnumDefinition) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_enumerator(&self, node: Enumerator) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_enumerator_list(&self, node: EnumeratorList) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_export_variable_statement(
            &self,
            node: ExportVariableStatement,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_expression_statement(&self, node: ExpressionStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_extends_statement(&self, node: ExtendsStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_for_statement(&self, node: ForStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_function_definition(&self, node: FunctionDefinition) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_get_body(&self, node: GetBody) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_get_node(&self, node: GetNode) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_getter(&self, node: Getter) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_identifier(&self, node: Identifier) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_if_statement(&self, node: IfStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_inferred_type(&self, node: InferredType) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_lambda(&self, node: Lambda) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_match_body(&self, node: MatchBody) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_match_statement(&self, node: MatchStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_name(&self, node: Name) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_node_path(&self, node: NodePath) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_onready_variable_statement(
            &self,
            node: OnreadyVariableStatement,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pair(&self, node: Pair) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_parameters(&self, node: Parameters) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_parenthesized_expression(
            &self,
            node: ParenthesizedExpression,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pass_statement(&self, node: PassStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pattern_binding(&self, node: PatternBinding) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pattern_guard(&self, node: PatternGuard) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pattern_section(&self, node: PatternSection) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_region_start(&self, node: RegionStart) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_remote_keyword(&self, node: RemoteKeyword) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_return_statement(&self, node: ReturnStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_set_body(&self, node: SetBody) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_setter(&self, node: Setter) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_signal_statement(&self, node: SignalStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_source(&self, node: Source) -> anyhow::Result<()> {
            Ok(())
        }

        // pub fn visit_string(&self, node: String) -> anyhow::Result<()> {
        //     Ok(())
        // }

        pub fn visit_string_name(&self, node: StringName) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_subscript(&self, node: Subscript) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_subscript_arguments(&self, node: SubscriptArguments) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_type(&self, node: Type) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_typed_default_parameter(
            &self,
            node: TypedDefaultParameter,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_typed_parameter(&self, node: TypedParameter) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_unary_operator(&self, node: UnaryOperator) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_value(&self, node: Value) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_variable_statement(&self, node: VariableStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_variadic_parameter(&self, node: VariadicParameter) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_while_statement(&self, node: WhileStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_init(&self, node: Init) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_and(&self, node: And) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_as(&self, node: As) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_await(&self, node: Await) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_break(&self, node: Break) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_breakpoint_statement(&self, node: BreakpointStatement) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_class(&self, node: Class) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_class_name(&self, node: ClassName) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_comment(&self, node: Comment) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_const(&self, node: Const) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_continue(&self, node: Continue) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_elif(&self, node: Elif) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_else(&self, node: Else) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_enum(&self, node: Enum) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_escape_sequence(&self, node: EscapeSequence) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_export(&self, node: Export) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_extends(&self, node: Extends) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_false(&self, node: False) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_float(&self, node: Float) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_for(&self, node: For) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_func(&self, node: Func) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_get(&self, node: Get) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_if(&self, node: If) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_in(&self, node: In) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_integer(&self, node: Integer) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_is(&self, node: Is) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_line_continuation(&self, node: LineContinuation) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_master(&self, node: Master) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_mastersync(&self, node: Mastersync) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_match(&self, node: Match) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_not(&self, node: Not) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_null(&self, node: Null) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_onready(&self, node: Onready) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_or(&self, node: Or) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pass(&self, node: Pass) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_pattern_open_ending(&self, node: PatternOpenEnding) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_puppet(&self, node: Puppet) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_puppetsync(&self, node: Puppetsync) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_region_end(&self, node: RegionEnd) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_region_label(&self, node: RegionLabel) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_remote(&self, node: Remote) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_remotesync(&self, node: Remotesync) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_return(&self, node: Return) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_set(&self, node: Set) -> anyhow::Result<()> {
            Ok(())
        }

        // pub fn visit_setget(&self, node: Setget) -> anyhow::Result<()> {
        //     Ok(())
        // }

        pub fn visit_signal(&self, node: Signal) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_static_keyword(&self, node: StaticKeyword) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_true(&self, node: True) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_var(&self, node: Var) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_when(&self, node: When) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_while(&self, node: While) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_l_brace(&self, node: LBrace) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_or_eq(&self, node: OrEq) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_or_or(&self, node: OrOr) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_r_brace(&self, node: RBrace) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_bit_not(&self, node: BitNot) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn visit_comma(&self, node: Comma) -> anyhow::Result<()> {
            Ok(())
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let code = std::fs::read_to_string(cli.path)?;
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
