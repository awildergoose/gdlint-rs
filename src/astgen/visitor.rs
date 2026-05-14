use type_sitter::Node;

use crate::{
    astgen::{
        ast::GDNodeKind,
        gdscript::{
            symbols::{
                Add, AddEq, And as AndSymbol, AndAnd, AndDoubleQuote, AndEq, At, BitNot, BitXor,
                BitXorDoubleQuote, BitXorEq, Colon, ColonEq, Comma, Div, DivEq, Dollar, Dot,
                DotDotDot, DoubleQuote, Eq, EqEq, Gt, GtEq, GtGt, GtGtEq, Hashregion, LBrace,
                LBracket, LParen, LParenRParen, Lt, LtEq, LtLt, LtLtEq, Mod, ModEq, Mul, MulEq,
                MulMul, MulMulEq, Not as NotSymbol, NotEq, Or as OrSymbol, OrEq, OrOr, RBrace,
                RBracket, RParen, Semicolon, Sub, SubEq, SubGt,
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
            NodePath, Null, OnreadyVariableStatement, Pair, Parameters, Parameters_,
            ParenthesizedExpression, PassStatement, PatternBinding, PatternGuard,
            PatternOpenEnding, PatternSection, RegionEnd, RegionLabel, RegionStart, RemoteKeyword,
            ReturnStatement, SetBody, Setget, Setter, SignalStatement, Source, StaticKeyword,
            String as GDString, StringName, Subscript, SubscriptArguments, True, Type,
            TypedDefaultParameter, TypedParameter, UnaryOperator, VariableStatement,
            VariadicParameter, WhileStatement,
        },
    },
    string_of_node, string_of_node_res, wrap_err, Traverser,
};

#[allow(clippy::unnecessary_wraps)]
#[allow(clippy::missing_const_for_fn)]
#[allow(clippy::unused_self)]
#[allow(unused_variables)]
impl Traverser {
    pub fn visit_g_d_string(&mut self, node: GDString) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_l_paren_r_paren(&mut self, node: LParenRParen) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_not_eq(&mut self, node: NotEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_double_quote(&mut self, node: DoubleQuote) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_hashregion(&mut self, node: Hashregion) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_dollar(&mut self, node: Dollar) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mod(&mut self, node: Mod) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mod_eq(&mut self, node: ModEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_and_double_quote(&mut self, node: AndDoubleQuote) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_and_and(&mut self, node: AndAnd) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_and_eq(&mut self, node: AndEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_l_paren(&mut self, node: LParen) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_r_paren(&mut self, node: RParen) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mul(&mut self, node: Mul) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mul_mul(&mut self, node: MulMul) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mul_mul_eq(&mut self, node: MulMulEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mul_eq(&mut self, node: MulEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_add(&mut self, node: Add) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_add_eq(&mut self, node: AddEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_sub(&mut self, node: Sub) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_sub_eq(&mut self, node: SubEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_sub_gt(&mut self, node: SubGt) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_dot(&mut self, node: Dot) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_dot_dot_dot(&mut self, node: DotDotDot) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_div(&mut self, node: Div) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_div_eq(&mut self, node: DivEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_colon(&mut self, node: Colon) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_colon_eq(&mut self, node: ColonEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_semicolon(&mut self, node: Semicolon) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_lt(&mut self, node: Lt) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_lt_lt(&mut self, node: LtLt) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_lt_lt_eq(&mut self, node: LtLtEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_lt_eq(&mut self, node: LtEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_eq(&mut self, node: Eq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_eq_eq(&mut self, node: EqEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_gt(&mut self, node: Gt) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_gt_eq(&mut self, node: GtEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_gt_gt(&mut self, node: GtGt) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_gt_gt_eq(&mut self, node: GtGtEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_at(&mut self, node: At) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_l_bracket(&mut self, node: LBracket) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_r_bracket(&mut self, node: RBracket) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_bit_xor(&mut self, node: BitXor) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_bit_xor_double_quote(&mut self, node: BitXorDoubleQuote) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_bit_xor_eq(&mut self, node: BitXorEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_annotation(&mut self, node: Annotation) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_annotations(&mut self, node: Annotations) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_arguments(&mut self, node: Arguments) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_array(&mut self, node: Array) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_assignment(&mut self, node: Assignment) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_attribute(&mut self, node: Attribute) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_attribute_call(&mut self, node: AttributeCall) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_attribute_subscript(&mut self, node: AttributeSubscript) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_augmented_assignment(&mut self, node: AugmentedAssignment) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_await_expression(&mut self, node: AwaitExpression) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_base_call(&mut self, node: BaseCall) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_binary_operator(&mut self, node: BinaryOperator) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_body(&mut self, node: Body) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_break_statement(&mut self, node: BreakStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_call(&mut self, node: Call) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_class_body(&mut self, node: ClassBody) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_class_definition(&mut self, node: ClassDefinition) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_class_name_statement(&mut self, node: ClassNameStatement) -> anyhow::Result<()> {
        if let GDNodeKind::Document { class_name, .. } = &mut self.root.kind {
            *class_name = Some(string_of_node_res!(self, node.name()));
        } else {
            anyhow::bail!("root is not of kind Document");
        }

        Ok(())
    }

    pub fn visit_conditional_expression(
        &mut self,
        node: ConditionalExpression,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_const_statement(&mut self, node: ConstStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_constructor_definition(
        &mut self,
        node: ConstructorDefinition,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_continue_statement(&mut self, node: ContinueStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_default_parameter(&mut self, node: DefaultParameter) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_dictionary(&mut self, node: Dictionary) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_elif_clause(&mut self, node: ElifClause) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_else_clause(&mut self, node: ElseClause) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_enum_definition(&mut self, node: EnumDefinition) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_enumerator(&mut self, node: Enumerator) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_enumerator_list(&mut self, node: EnumeratorList) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_export_variable_statement(
        &mut self,
        node: ExportVariableStatement,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_expression_statement(&mut self, node: ExpressionStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_extends_statement(&mut self, node: ExtendsStatement) -> anyhow::Result<()> {
        if let GDNodeKind::Document { extends, .. } = &mut self.root.kind {
            *extends = Some(
                string_of_node!(self, node)
                    .split(' ')
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(" "),
            );
        } else {
            anyhow::bail!("root is not of kind Document");
        }

        Ok(())
    }

    pub fn visit_for_statement(&mut self, node: ForStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_function_definition(&mut self, node: FunctionDefinition) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_get_body(&mut self, node: GetBody) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_get_node(&mut self, node: GetNode) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_getter(&mut self, node: Getter) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_identifier(&mut self, node: Identifier) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_if_statement(&mut self, node: IfStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_inferred_type(&mut self, node: InferredType) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_lambda(&mut self, node: Lambda) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_match_body(&mut self, node: MatchBody) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_match_statement(&mut self, node: MatchStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_name(&mut self, node: Name) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_node_path(&mut self, node: NodePath) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_onready_variable_statement(
        &mut self,
        node: OnreadyVariableStatement,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pair(&mut self, node: Pair) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_parameters(&mut self, node: Parameters) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_parameters_(&mut self, node: Parameters_) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_parenthesized_expression(
        &mut self,
        node: ParenthesizedExpression,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pass_statement(&mut self, node: PassStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pattern_binding(&mut self, node: PatternBinding) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pattern_guard(&mut self, node: PatternGuard) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pattern_section(&mut self, node: PatternSection) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_region_start(&mut self, node: RegionStart) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_remote_keyword(&mut self, node: RemoteKeyword) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_return_statement(&mut self, node: ReturnStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_set_body(&mut self, node: SetBody) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_setter(&mut self, node: Setter) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_signal_statement(&mut self, node: SignalStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_source(&mut self, node: Source) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_string_name(&mut self, node: StringName) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_subscript(&mut self, node: Subscript) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_subscript_arguments(&mut self, node: SubscriptArguments) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_type(&mut self, node: Type) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_typed_default_parameter(
        &mut self,
        node: TypedDefaultParameter,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_typed_parameter(&mut self, node: TypedParameter) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_unary_operator(&mut self, node: UnaryOperator) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_value(&mut self, node: Value) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_variable_statement(&mut self, node: VariableStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_variadic_parameter(&mut self, node: VariadicParameter) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_while_statement(&mut self, node: WhileStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_init(&mut self, node: Init) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_and(&mut self, node: And) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_and_symbol(&mut self, node: AndSymbol) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_as(&mut self, node: As) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_await(&mut self, node: Await) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_break(&mut self, node: Break) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_breakpoint_statement(&mut self, node: BreakpointStatement) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_class(&mut self, node: Class) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_class_name(&mut self, node: ClassName) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_comment(&mut self, node: Comment) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_const(&mut self, node: Const) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_continue(&mut self, node: Continue) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_elif(&mut self, node: Elif) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_else(&mut self, node: Else) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_enum(&mut self, node: Enum) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_escape_sequence(&mut self, node: EscapeSequence) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_export(&mut self, node: Export) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_extends(&mut self, node: Extends) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_false(&mut self, node: False) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_float(&mut self, node: Float) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_for(&mut self, node: For) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_func(&mut self, node: Func) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_get(&mut self, node: Get) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_if(&mut self, node: If) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_in(&mut self, node: In) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_integer(&mut self, node: Integer) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_is(&mut self, node: Is) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_line_continuation(&mut self, node: LineContinuation) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_master(&mut self, node: Master) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_mastersync(&mut self, node: Mastersync) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_match(&mut self, node: Match) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_not(&mut self, node: Not) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_not_symbol(&mut self, node: NotSymbol) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_null(&mut self, node: Null) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_onready(&mut self, node: Onready) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_or(&mut self, node: Or) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_or_symbol(&mut self, node: OrSymbol) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pass(&mut self, node: Pass) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_pattern_open_ending(&mut self, node: PatternOpenEnding) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_puppet(&mut self, node: Puppet) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_puppetsync(&mut self, node: Puppetsync) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_region_end(&mut self, node: RegionEnd) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_region_label(&mut self, node: RegionLabel) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_remote(&mut self, node: Remote) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_remotesync(&mut self, node: Remotesync) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_return(&mut self, node: Return) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_set(&mut self, node: Set) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_setget(&mut self, node: Setget) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_signal(&mut self, node: Signal) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_static_keyword(&mut self, node: StaticKeyword) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_true(&mut self, node: True) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_var(&mut self, node: Var) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_when(&mut self, node: When) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_while(&mut self, node: While) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_l_brace(&mut self, node: LBrace) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_or_eq(&mut self, node: OrEq) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_or_or(&mut self, node: OrOr) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_r_brace(&mut self, node: RBrace) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_bit_not(&mut self, node: BitNot) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn visit_comma(&mut self, node: Comma) -> anyhow::Result<()> {
        Ok(())
    }
}
