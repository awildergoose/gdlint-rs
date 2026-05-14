#![allow(clippy::possible_missing_else)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::filter_next)]
#![allow(clippy::elidable_lifetime_names)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::too_many_lines)]

#[doc = "Typed node `_attribute_expression`\n\nThis node type has subtypes:\n\n- `array` ([`Array`])\n- `base_call` ([`BaseCall`])\n- `binary_operator` ([`BinaryOperator`])\n- `call` ([`Call`])\n- `dictionary` ([`Dictionary`])\n- `false` ([`False`])\n- `float` ([`Float`])\n- `get_node` ([`GetNode`])\n- `identifier` ([`Identifier`])\n- `integer` ([`Integer`])\n- `node_path` ([`NodePath`])\n- `null` ([`Null`])\n- `parenthesized_expression` ([`ParenthesizedExpression`])\n- `string` ([`String`])\n- `subscript` ([`Subscript`])\n- `true` ([`True`])\n- `unary_operator` ([`UnaryOperator`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum AttributeExpression<'tree> {
    Array(Array<'tree>),
    BaseCall(BaseCall<'tree>),
    BinaryOperator(BinaryOperator<'tree>),
    Call(Call<'tree>),
    Dictionary(Dictionary<'tree>),
    False(False<'tree>),
    Float(Float<'tree>),
    GetNode(GetNode<'tree>),
    Identifier(Identifier<'tree>),
    Integer(Integer<'tree>),
    NodePath(NodePath<'tree>),
    Null(Null<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    String(String<'tree>),
    Subscript(Subscript<'tree>),
    True(True<'tree>),
    UnaryOperator(UnaryOperator<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeExpression<'tree> {
    #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`"]
    #[inline]
    pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Array(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`"]
    #[inline]
    pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BaseCall(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`"]
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BinaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`"]
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Call(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`"]
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Dictionary(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`"]
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::False(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`"]
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Float(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`"]
    #[inline]
    pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::GetNode(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`"]
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Integer(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`"]
    #[inline]
    pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::NodePath(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`"]
    #[inline]
    pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Null(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ParenthesizedExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`"]
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`"]
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Subscript(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`"]
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::True(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`"]
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::UnaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeExpression<'tree> {
    type WithLifetime<'a> = AttributeExpression<'a>;

    const KIND: &'static str = "_attribute_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        match node.kind() {
            "array" => Ok(unsafe {
                Self::Array(<Array<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "base_call" => Ok(unsafe {
                Self::BaseCall(
                    <BaseCall<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "binary_operator" => Ok(unsafe {
                Self::BinaryOperator(
                    <BinaryOperator<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "call" => Ok(unsafe {
                Self::Call(<Call<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "dictionary" => Ok(unsafe {
                Self::Dictionary(
                    <Dictionary<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "false" => Ok(unsafe {
                Self::False(<False<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "float" => Ok(unsafe {
                Self::Float(<Float<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "get_node" => Ok(unsafe {
                Self::GetNode(
                    <GetNode<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "identifier" => Ok(unsafe {
                Self::Identifier(
                    <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "integer" => Ok(unsafe {
                Self::Integer(
                    <Integer<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "node_path" => Ok(unsafe {
                Self::NodePath(
                    <NodePath<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "null" => Ok(unsafe {
                Self::Null(<Null<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "parenthesized_expression" => Ok(unsafe {
                Self::ParenthesizedExpression(<ParenthesizedExpression<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
            }),
            "string" => Ok(unsafe {
                Self::String(
                    <String<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "subscript" => Ok(unsafe {
                Self::Subscript(
                    <Subscript<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "true" => Ok(unsafe {
                Self::True(<True<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "unary_operator" => Ok(unsafe {
                Self::UnaryOperator(
                    <UnaryOperator<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
        }
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter::Node::raw(x),
            Self::BaseCall(x) => ::type_sitter::Node::raw(x),
            Self::BinaryOperator(x) => ::type_sitter::Node::raw(x),
            Self::Call(x) => ::type_sitter::Node::raw(x),
            Self::Dictionary(x) => ::type_sitter::Node::raw(x),
            Self::False(x) => ::type_sitter::Node::raw(x),
            Self::Float(x) => ::type_sitter::Node::raw(x),
            Self::GetNode(x) => ::type_sitter::Node::raw(x),
            Self::Identifier(x) => ::type_sitter::Node::raw(x),
            Self::Integer(x) => ::type_sitter::Node::raw(x),
            Self::NodePath(x) => ::type_sitter::Node::raw(x),
            Self::Null(x) => ::type_sitter::Node::raw(x),
            Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
            Self::String(x) => ::type_sitter::Node::raw(x),
            Self::Subscript(x) => ::type_sitter::Node::raw(x),
            Self::True(x) => ::type_sitter::Node::raw(x),
            Self::UnaryOperator(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter::Node::raw_mut(x),
            Self::BaseCall(x) => ::type_sitter::Node::raw_mut(x),
            Self::BinaryOperator(x) => ::type_sitter::Node::raw_mut(x),
            Self::Call(x) => ::type_sitter::Node::raw_mut(x),
            Self::Dictionary(x) => ::type_sitter::Node::raw_mut(x),
            Self::False(x) => ::type_sitter::Node::raw_mut(x),
            Self::Float(x) => ::type_sitter::Node::raw_mut(x),
            Self::GetNode(x) => ::type_sitter::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
            Self::Integer(x) => ::type_sitter::Node::raw_mut(x),
            Self::NodePath(x) => ::type_sitter::Node::raw_mut(x),
            Self::Null(x) => ::type_sitter::Node::raw_mut(x),
            Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::String(x) => ::type_sitter::Node::raw_mut(x),
            Self::Subscript(x) => ::type_sitter::Node::raw_mut(x),
            Self::True(x) => ::type_sitter::Node::raw_mut(x),
            Self::UnaryOperator(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => x.into_raw(),
            Self::BaseCall(x) => x.into_raw(),
            Self::BinaryOperator(x) => x.into_raw(),
            Self::Call(x) => x.into_raw(),
            Self::Dictionary(x) => x.into_raw(),
            Self::False(x) => x.into_raw(),
            Self::Float(x) => x.into_raw(),
            Self::GetNode(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::Integer(x) => x.into_raw(),
            Self::NodePath(x) => x.into_raw(),
            Self::Null(x) => x.into_raw(),
            Self::ParenthesizedExpression(x) => x.into_raw(),
            Self::String(x) => x.into_raw(),
            Self::Subscript(x) => x.into_raw(),
            Self::True(x) => x.into_raw(),
            Self::UnaryOperator(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `_compound_statement`\n\nThis node type has subtypes:\n\n- `class_definition` ([`ClassDefinition`])\n- `constructor_definition` ([`ConstructorDefinition`])\n- `enum_definition` ([`EnumDefinition`])\n- `for_statement` ([`ForStatement`])\n- `function_definition` ([`FunctionDefinition`])\n- `if_statement` ([`IfStatement`])\n- `match_statement` ([`MatchStatement`])\n- `while_statement` ([`WhileStatement`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum CompoundStatement<'tree> {
    ClassDefinition(ClassDefinition<'tree>),
    ConstructorDefinition(ConstructorDefinition<'tree>),
    EnumDefinition(EnumDefinition<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    IfStatement(IfStatement<'tree>),
    MatchStatement(MatchStatement<'tree>),
    WhileStatement(WhileStatement<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> CompoundStatement<'tree> {
    #[doc = "Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`"]
    #[inline]
    pub fn as_class_definition(self) -> ::std::option::Option<ClassDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ClassDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `constructor_definition` ([`ConstructorDefinition`]), otherwise returns `None`"]
    #[inline]
    pub fn as_constructor_definition(self) -> ::std::option::Option<ConstructorDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConstructorDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `enum_definition` ([`EnumDefinition`]), otherwise returns `None`"]
    #[inline]
    pub fn as_enum_definition(self) -> ::std::option::Option<EnumDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::EnumDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `for_statement` ([`ForStatement`]), otherwise returns `None`"]
    #[inline]
    pub fn as_for_statement(self) -> ::std::option::Option<ForStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ForStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`"]
    #[inline]
    pub fn as_function_definition(self) -> ::std::option::Option<FunctionDefinition<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::FunctionDefinition(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `if_statement` ([`IfStatement`]), otherwise returns `None`"]
    #[inline]
    pub fn as_if_statement(self) -> ::std::option::Option<IfStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::IfStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `match_statement` ([`MatchStatement`]), otherwise returns `None`"]
    #[inline]
    pub fn as_match_statement(self) -> ::std::option::Option<MatchStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::MatchStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `while_statement` ([`WhileStatement`]), otherwise returns `None`"]
    #[inline]
    pub fn as_while_statement(self) -> ::std::option::Option<WhileStatement<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::WhileStatement(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Get the field `body`.\n\nThis child has type `{body | class_body | enumerator_list | match_body}`:\n\n- [`Body`]\n- [`ClassBody`]\n- [`EnumeratorList`]\n- [`MatchBody`]\n"]
    #[inline]
    pub fn body(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::Body_ClassBody_EnumeratorList_MatchBody<'tree>>
    {
        ::type_sitter::Node::raw(self).child_by_field_name("body").map(<anon_unions::Body_ClassBody_EnumeratorList_MatchBody<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw).expect("required child not present, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for CompoundStatement<'tree> {
    type WithLifetime<'a> = CompoundStatement<'a>;

    const KIND: &'static str = "_compound_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        match node.kind() {
            "class_definition" => Ok(unsafe {
                Self::ClassDefinition(
                    <ClassDefinition<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        node,
                    ),
                )
            }),
            "constructor_definition" => {
                Ok(unsafe {
                    Self::ConstructorDefinition(<ConstructorDefinition<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                })
            }
            "enum_definition" => Ok(unsafe {
                Self::EnumDefinition(
                    <EnumDefinition<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "for_statement" => Ok(unsafe {
                Self::ForStatement(
                    <ForStatement<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "function_definition" => Ok(unsafe {
                Self::FunctionDefinition(<FunctionDefinition<'tree> as ::type_sitter::Node<
                    'tree,
                >>::from_raw_unchecked(node))
            }),
            "if_statement" => Ok(unsafe {
                Self::IfStatement(
                    <IfStatement<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "match_statement" => Ok(unsafe {
                Self::MatchStatement(
                    <MatchStatement<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "while_statement" => Ok(unsafe {
                Self::WhileStatement(
                    <WhileStatement<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
        }
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => ::type_sitter::Node::raw(x),
            Self::ConstructorDefinition(x) => ::type_sitter::Node::raw(x),
            Self::EnumDefinition(x) => ::type_sitter::Node::raw(x),
            Self::ForStatement(x) => ::type_sitter::Node::raw(x),
            Self::FunctionDefinition(x) => ::type_sitter::Node::raw(x),
            Self::IfStatement(x) => ::type_sitter::Node::raw(x),
            Self::MatchStatement(x) => ::type_sitter::Node::raw(x),
            Self::WhileStatement(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => ::type_sitter::Node::raw_mut(x),
            Self::ConstructorDefinition(x) => ::type_sitter::Node::raw_mut(x),
            Self::EnumDefinition(x) => ::type_sitter::Node::raw_mut(x),
            Self::ForStatement(x) => ::type_sitter::Node::raw_mut(x),
            Self::FunctionDefinition(x) => ::type_sitter::Node::raw_mut(x),
            Self::IfStatement(x) => ::type_sitter::Node::raw_mut(x),
            Self::MatchStatement(x) => ::type_sitter::Node::raw_mut(x),
            Self::WhileStatement(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::ClassDefinition(x) => x.into_raw(),
            Self::ConstructorDefinition(x) => x.into_raw(),
            Self::EnumDefinition(x) => x.into_raw(),
            Self::ForStatement(x) => x.into_raw(),
            Self::FunctionDefinition(x) => x.into_raw(),
            Self::IfStatement(x) => x.into_raw(),
            Self::MatchStatement(x) => x.into_raw(),
            Self::WhileStatement(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `_expression`\n\nThis node type has subtypes:\n\n- `_primary_expression` ([`PrimaryExpression`])\n- `conditional_expression` ([`ConditionalExpression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Expression<'tree> {
    PrimaryExpression(PrimaryExpression<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Expression<'tree> {
    #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PrimaryExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_conditional_expression(self) -> ::std::option::Option<ConditionalExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConditionalExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
        self.as_primary_expression()?.as_array()
    }

    #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        self.as_primary_expression()?.as_attribute()
    }

    #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
        self.as_primary_expression()?.as_await_expression()
    }

    #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
        self.as_primary_expression()?.as_base_call()
    }

    #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        self.as_primary_expression()?.as_binary_operator()
    }

    #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        self.as_primary_expression()?.as_call()
    }

    #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        self.as_primary_expression()?.as_dictionary()
    }

    #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        self.as_primary_expression()?.as_false()
    }

    #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        self.as_primary_expression()?.as_float()
    }

    #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
        self.as_primary_expression()?.as_get_node()
    }

    #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        self.as_primary_expression()?.as_identifier()
    }

    #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        self.as_primary_expression()?.as_integer()
    }

    #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
        self.as_primary_expression()?.as_node_path()
    }

    #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
        self.as_primary_expression()?.as_null()
    }

    #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        self.as_primary_expression()?.as_parenthesized_expression()
    }

    #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        self.as_primary_expression()?.as_string()
    }

    #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
        self.as_primary_expression()?.as_string_name()
    }

    #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        self.as_primary_expression()?.as_subscript()
    }

    #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        self.as_primary_expression()?.as_true()
    }

    #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        self.as_primary_expression()?.as_unary_operator()
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Expression<'tree> {
    type WithLifetime<'a> = Expression<'a>;

    const KIND: &'static str = "_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if let Ok(this) =
            <PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
        {
            return Ok(Self::PrimaryExpression(this));
        }
        if let Ok(this) =
            <ConditionalExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
        {
            return Ok(Self::ConditionalExpression(this));
        }
        Err(::type_sitter::IncorrectKind::new::<Self>(node))
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
            Self::ConditionalExpression(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::ConditionalExpression(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => x.into_raw(),
            Self::ConditionalExpression(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `_parameters`\n\nThis node type has subtypes:\n\n- `default_parameter` ([`DefaultParameter`])\n- `identifier` ([`Identifier`])\n- `typed_default_parameter` ([`TypedDefaultParameter`])\n- `typed_parameter` ([`TypedParameter`])\n- `variadic_parameter` ([`VariadicParameter`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Parameters<'tree> {
    DefaultParameter(DefaultParameter<'tree>),
    Identifier(Identifier<'tree>),
    TypedDefaultParameter(TypedDefaultParameter<'tree>),
    TypedParameter(TypedParameter<'tree>),
    VariadicParameter(VariadicParameter<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameters<'tree> {
    #[doc = "Returns the node if it is of type `default_parameter` ([`DefaultParameter`]), otherwise returns `None`"]
    #[inline]
    pub fn as_default_parameter(self) -> ::std::option::Option<DefaultParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::DefaultParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `typed_default_parameter` ([`TypedDefaultParameter`]), otherwise returns `None`"]
    #[inline]
    pub fn as_typed_default_parameter(self) -> ::std::option::Option<TypedDefaultParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypedDefaultParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `typed_parameter` ([`TypedParameter`]), otherwise returns `None`"]
    #[inline]
    pub fn as_typed_parameter(self) -> ::std::option::Option<TypedParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::TypedParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `variadic_parameter` ([`VariadicParameter`]), otherwise returns `None`"]
    #[inline]
    pub fn as_variadic_parameter(self) -> ::std::option::Option<VariadicParameter<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::VariadicParameter(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Parameters<'tree> {
    type WithLifetime<'a> = Parameters<'a>;

    const KIND: &'static str = "_parameters";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        match node.kind() {
            "default_parameter" => {
                Ok(unsafe {
                    Self::DefaultParameter(<DefaultParameter<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                })
            }
            "identifier" => Ok(unsafe {
                Self::Identifier(
                    <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "typed_default_parameter" => {
                Ok(unsafe {
                    Self::TypedDefaultParameter(<TypedDefaultParameter<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                })
            }
            "typed_parameter" => Ok(unsafe {
                Self::TypedParameter(
                    <TypedParameter<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "variadic_parameter" => {
                Ok(unsafe {
                    Self::VariadicParameter(<VariadicParameter<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                })
            }
            _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
        }
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => ::type_sitter::Node::raw(x),
            Self::Identifier(x) => ::type_sitter::Node::raw(x),
            Self::TypedDefaultParameter(x) => ::type_sitter::Node::raw(x),
            Self::TypedParameter(x) => ::type_sitter::Node::raw(x),
            Self::VariadicParameter(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => ::type_sitter::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
            Self::TypedDefaultParameter(x) => ::type_sitter::Node::raw_mut(x),
            Self::TypedParameter(x) => ::type_sitter::Node::raw_mut(x),
            Self::VariadicParameter(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::DefaultParameter(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::TypedDefaultParameter(x) => x.into_raw(),
            Self::TypedParameter(x) => x.into_raw(),
            Self::VariadicParameter(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `_pattern`\n\nThis node type has subtypes:\n\n- `_primary_expression` ([`PrimaryExpression`])\n- `conditional_expression` ([`ConditionalExpression`])\n- `pattern_binding` ([`PatternBinding`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Pattern<'tree> {
    PrimaryExpression(PrimaryExpression<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    PatternBinding(PatternBinding<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> Pattern<'tree> {
    #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PrimaryExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_conditional_expression(self) -> ::std::option::Option<ConditionalExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ConditionalExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `pattern_binding` ([`PatternBinding`]), otherwise returns `None`"]
    #[inline]
    pub fn as_pattern_binding(self) -> ::std::option::Option<PatternBinding<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::PatternBinding(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
        self.as_primary_expression()?.as_array()
    }

    #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        self.as_primary_expression()?.as_attribute()
    }

    #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
        self.as_primary_expression()?.as_await_expression()
    }

    #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
        self.as_primary_expression()?.as_base_call()
    }

    #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        self.as_primary_expression()?.as_binary_operator()
    }

    #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        self.as_primary_expression()?.as_call()
    }

    #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        self.as_primary_expression()?.as_dictionary()
    }

    #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        self.as_primary_expression()?.as_false()
    }

    #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        self.as_primary_expression()?.as_float()
    }

    #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
        self.as_primary_expression()?.as_get_node()
    }

    #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        self.as_primary_expression()?.as_identifier()
    }

    #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        self.as_primary_expression()?.as_integer()
    }

    #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
        self.as_primary_expression()?.as_node_path()
    }

    #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
        self.as_primary_expression()?.as_null()
    }

    #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        self.as_primary_expression()?.as_parenthesized_expression()
    }

    #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        self.as_primary_expression()?.as_string()
    }

    #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
        self.as_primary_expression()?.as_string_name()
    }

    #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        self.as_primary_expression()?.as_subscript()
    }

    #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        self.as_primary_expression()?.as_true()
    }

    #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        self.as_primary_expression()?.as_unary_operator()
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Pattern<'tree> {
    type WithLifetime<'a> = Pattern<'a>;

    const KIND: &'static str = "_pattern";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if let Ok(this) =
            <PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
        {
            return Ok(Self::PrimaryExpression(this));
        }
        if let Ok(this) =
            <ConditionalExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
        {
            return Ok(Self::ConditionalExpression(this));
        }
        if let Ok(this) = <PatternBinding<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
        {
            return Ok(Self::PatternBinding(this));
        }
        Err(::type_sitter::IncorrectKind::new::<Self>(node))
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
            Self::ConditionalExpression(x) => ::type_sitter::Node::raw(x),
            Self::PatternBinding(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::ConditionalExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::PatternBinding(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::PrimaryExpression(x) => x.into_raw(),
            Self::ConditionalExpression(x) => x.into_raw(),
            Self::PatternBinding(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `_primary_expression`\n\nThis node type has subtypes:\n\n- `array` ([`Array`])\n- `attribute` ([`Attribute`])\n- `await_expression` ([`AwaitExpression`])\n- `base_call` ([`BaseCall`])\n- `binary_operator` ([`BinaryOperator`])\n- `call` ([`Call`])\n- `dictionary` ([`Dictionary`])\n- `false` ([`False`])\n- `float` ([`Float`])\n- `get_node` ([`GetNode`])\n- `identifier` ([`Identifier`])\n- `integer` ([`Integer`])\n- `node_path` ([`NodePath`])\n- `null` ([`Null`])\n- `parenthesized_expression` ([`ParenthesizedExpression`])\n- `string` ([`String`])\n- `string_name` ([`StringName`])\n- `subscript` ([`Subscript`])\n- `true` ([`True`])\n- `unary_operator` ([`UnaryOperator`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum PrimaryExpression<'tree> {
    Array(Array<'tree>),
    Attribute(Attribute<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BaseCall(BaseCall<'tree>),
    BinaryOperator(BinaryOperator<'tree>),
    Call(Call<'tree>),
    Dictionary(Dictionary<'tree>),
    False(False<'tree>),
    Float(Float<'tree>),
    GetNode(GetNode<'tree>),
    Identifier(Identifier<'tree>),
    Integer(Integer<'tree>),
    NodePath(NodePath<'tree>),
    Null(Null<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    String(String<'tree>),
    StringName(StringName<'tree>),
    Subscript(Subscript<'tree>),
    True(True<'tree>),
    UnaryOperator(UnaryOperator<'tree>),
}
#[automatically_derived]
#[allow(unused)]
impl<'tree> PrimaryExpression<'tree> {
    #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`"]
    #[inline]
    pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Array(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`"]
    #[inline]
    pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Attribute(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::AwaitExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`"]
    #[inline]
    pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BaseCall(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`"]
    #[inline]
    pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::BinaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`"]
    #[inline]
    pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Call(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`"]
    #[inline]
    pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Dictionary(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`"]
    #[inline]
    pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::False(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`"]
    #[inline]
    pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Float(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`"]
    #[inline]
    pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::GetNode(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
    #[inline]
    pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Identifier(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`"]
    #[inline]
    pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Integer(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`"]
    #[inline]
    pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::NodePath(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`"]
    #[inline]
    pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Null(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`"]
    #[inline]
    pub fn as_parenthesized_expression(
        self,
    ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::ParenthesizedExpression(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`"]
    #[inline]
    pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::String(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`"]
    #[inline]
    pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::StringName(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`"]
    #[inline]
    pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::Subscript(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`"]
    #[inline]
    pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::True(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }

    #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`"]
    #[inline]
    pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
        #[allow(irrefutable_let_patterns)]
        if let Self::UnaryOperator(x) = self {
            ::std::option::Option::Some(x)
        } else {
            ::std::option::Option::None
        }
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PrimaryExpression<'tree> {
    type WithLifetime<'a> = PrimaryExpression<'a>;

    const KIND: &'static str = "_primary_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        match node.kind() {
            "array" => Ok(unsafe {
                Self::Array(<Array<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "attribute" => Ok(unsafe {
                Self::Attribute(
                    <Attribute<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "await_expression" => Ok(unsafe {
                Self::AwaitExpression(
                    <AwaitExpression<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                        node,
                    ),
                )
            }),
            "base_call" => Ok(unsafe {
                Self::BaseCall(
                    <BaseCall<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "binary_operator" => Ok(unsafe {
                Self::BinaryOperator(
                    <BinaryOperator<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "call" => Ok(unsafe {
                Self::Call(<Call<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "dictionary" => Ok(unsafe {
                Self::Dictionary(
                    <Dictionary<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "false" => Ok(unsafe {
                Self::False(<False<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "float" => Ok(unsafe {
                Self::Float(<Float<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "get_node" => Ok(unsafe {
                Self::GetNode(
                    <GetNode<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "identifier" => Ok(unsafe {
                Self::Identifier(
                    <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "integer" => Ok(unsafe {
                Self::Integer(
                    <Integer<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "node_path" => Ok(unsafe {
                Self::NodePath(
                    <NodePath<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "null" => Ok(unsafe {
                Self::Null(<Null<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "parenthesized_expression" => Ok(unsafe {
                Self::ParenthesizedExpression(<ParenthesizedExpression<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
            }),
            "string" => Ok(unsafe {
                Self::String(
                    <String<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "string_name" => Ok(unsafe {
                Self::StringName(
                    <StringName<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "subscript" => Ok(unsafe {
                Self::Subscript(
                    <Subscript<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            "true" => Ok(unsafe {
                Self::True(<True<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node))
            }),
            "unary_operator" => Ok(unsafe {
                Self::UnaryOperator(
                    <UnaryOperator<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                )
            }),
            _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
        }
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter::Node::raw(x),
            Self::Attribute(x) => ::type_sitter::Node::raw(x),
            Self::AwaitExpression(x) => ::type_sitter::Node::raw(x),
            Self::BaseCall(x) => ::type_sitter::Node::raw(x),
            Self::BinaryOperator(x) => ::type_sitter::Node::raw(x),
            Self::Call(x) => ::type_sitter::Node::raw(x),
            Self::Dictionary(x) => ::type_sitter::Node::raw(x),
            Self::False(x) => ::type_sitter::Node::raw(x),
            Self::Float(x) => ::type_sitter::Node::raw(x),
            Self::GetNode(x) => ::type_sitter::Node::raw(x),
            Self::Identifier(x) => ::type_sitter::Node::raw(x),
            Self::Integer(x) => ::type_sitter::Node::raw(x),
            Self::NodePath(x) => ::type_sitter::Node::raw(x),
            Self::Null(x) => ::type_sitter::Node::raw(x),
            Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw(x),
            Self::String(x) => ::type_sitter::Node::raw(x),
            Self::StringName(x) => ::type_sitter::Node::raw(x),
            Self::Subscript(x) => ::type_sitter::Node::raw(x),
            Self::True(x) => ::type_sitter::Node::raw(x),
            Self::UnaryOperator(x) => ::type_sitter::Node::raw(x),
        }
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => ::type_sitter::Node::raw_mut(x),
            Self::Attribute(x) => ::type_sitter::Node::raw_mut(x),
            Self::AwaitExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::BaseCall(x) => ::type_sitter::Node::raw_mut(x),
            Self::BinaryOperator(x) => ::type_sitter::Node::raw_mut(x),
            Self::Call(x) => ::type_sitter::Node::raw_mut(x),
            Self::Dictionary(x) => ::type_sitter::Node::raw_mut(x),
            Self::False(x) => ::type_sitter::Node::raw_mut(x),
            Self::Float(x) => ::type_sitter::Node::raw_mut(x),
            Self::GetNode(x) => ::type_sitter::Node::raw_mut(x),
            Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
            Self::Integer(x) => ::type_sitter::Node::raw_mut(x),
            Self::NodePath(x) => ::type_sitter::Node::raw_mut(x),
            Self::Null(x) => ::type_sitter::Node::raw_mut(x),
            Self::ParenthesizedExpression(x) => ::type_sitter::Node::raw_mut(x),
            Self::String(x) => ::type_sitter::Node::raw_mut(x),
            Self::StringName(x) => ::type_sitter::Node::raw_mut(x),
            Self::Subscript(x) => ::type_sitter::Node::raw_mut(x),
            Self::True(x) => ::type_sitter::Node::raw_mut(x),
            Self::UnaryOperator(x) => ::type_sitter::Node::raw_mut(x),
        }
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        match self {
            Self::Array(x) => x.into_raw(),
            Self::Attribute(x) => x.into_raw(),
            Self::AwaitExpression(x) => x.into_raw(),
            Self::BaseCall(x) => x.into_raw(),
            Self::BinaryOperator(x) => x.into_raw(),
            Self::Call(x) => x.into_raw(),
            Self::Dictionary(x) => x.into_raw(),
            Self::False(x) => x.into_raw(),
            Self::Float(x) => x.into_raw(),
            Self::GetNode(x) => x.into_raw(),
            Self::Identifier(x) => x.into_raw(),
            Self::Integer(x) => x.into_raw(),
            Self::NodePath(x) => x.into_raw(),
            Self::Null(x) => x.into_raw(),
            Self::ParenthesizedExpression(x) => x.into_raw(),
            Self::String(x) => x.into_raw(),
            Self::StringName(x) => x.into_raw(),
            Self::Subscript(x) => x.into_raw(),
            Self::True(x) => x.into_raw(),
            Self::UnaryOperator(x) => x.into_raw(),
        }
    }
}
#[doc = "Typed node `annotation`\n\nThis node has these fields:\n\n- `arguments`: `arguments?` ([`Arguments`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Annotation<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Annotation<'tree> {
    #[doc = "Get the optional field `arguments`.\n\nThis child has type `arguments?` ([`Arguments`])"]
    #[inline]
    pub fn arguments(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Arguments<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Annotation<'tree> {
    type WithLifetime<'a> = Annotation<'a>;

    const KIND: &'static str = "annotation";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "annotation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "annotation");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `annotations`\n\nThis node has named children of type `annotation+` ([`Annotation`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Annotations<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Annotations<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `annotation+` ([`Annotation`])"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn annotations<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, Annotation<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Annotation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Annotations<'tree> {
    type Child = Annotation<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Annotations<'tree> {
    type WithLifetime<'a> = Annotations<'a>;

    const KIND: &'static str = "annotations";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "annotations" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "annotations");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `arguments`\n\nThis node has named children of type `{_expression | lambda}*`:\n\n- [`Expression`]\n- [`Lambda`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Arguments<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Arguments<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Arguments<'tree> {
    type Child = anon_unions::Expression_Lambda<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Arguments<'tree> {
    type WithLifetime<'a> = Arguments<'a>;

    const KIND: &'static str = "arguments";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "arguments" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "arguments");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `array`\n\nThis node has named children of type `{_expression | lambda | pattern_binding | pattern_open_ending}*`:\n\n- [`Expression`]\n- [`Lambda`]\n- [`PatternBinding`]\n- [`PatternOpenEnding`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Array<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Array<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Array<'tree> {
    type Child = anon_unions::Expression_Lambda_PatternBinding_PatternOpenEnding<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Array<'tree> {
    type WithLifetime<'a> = Array<'a>;

    const KIND: &'static str = "array";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "array" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "array");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `assignment`\n\nThis node has these fields:\n\n- `left`: `_expression` ([`Expression`])\n- `right`: `{_expression | lambda}` ([`Expression`] | [`Lambda`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Assignment<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Assignment<'tree> {
    #[doc = "Get the field `left`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `right`.\n\nThis child has type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Assignment<'tree> {
    type WithLifetime<'a> = Assignment<'a>;

    const KIND: &'static str = "assignment";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "assignment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "assignment");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `attribute`\n\nThis node has named children of type `{_attribute_expression | attribute_call | attribute_subscript}+`:\n\n- [`AttributeExpression`]\n- [`AttributeCall`]\n- [`AttributeSubscript`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Attribute<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Attribute<'tree> {
    type Child = anon_unions::AttributeExpression_AttributeCall_AttributeSubscript<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Attribute<'tree> {
    type WithLifetime<'a> = Attribute<'a>;

    const KIND: &'static str = "attribute";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `attribute_call`\n\nThis node has these fields:\n\n- `arguments`: `arguments` ([`Arguments`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeCall<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeCall<'tree> {
    #[doc = "Get the field `arguments`.\n\nThis child has type `arguments` ([`Arguments`])"]
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, Arguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeCall<'tree> {
    type WithLifetime<'a> = AttributeCall<'a>;

    const KIND: &'static str = "attribute_call";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_call" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_call");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `attribute_subscript`\n\nThis node has these fields:\n\n- `arguments`: `subscript_arguments` ([`SubscriptArguments`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AttributeSubscript<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> AttributeSubscript<'tree> {
    #[doc = "Get the field `arguments`.\n\nThis child has type `subscript_arguments` ([`SubscriptArguments`])"]
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, SubscriptArguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<SubscriptArguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AttributeSubscript<'tree> {
    type WithLifetime<'a> = AttributeSubscript<'a>;

    const KIND: &'static str = "attribute_subscript";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "attribute_subscript" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "attribute_subscript");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `augmented_assignment`\n\nThis node has these fields:\n\n- `left`: `_expression` ([`Expression`])\n- `op`: `{%= | &= | **= | *= | += | -= | /= | <<= | >>= | ^= | |=}` ([`symbols::ModEq`] | [`symbols::AndEq`] | [`symbols::MulMulEq`] | [`symbols::MulEq`] | [`symbols::AddEq`] | [`symbols::SubEq`] | [`symbols::DivEq`] | [`symbols::LtLtEq`] | [`symbols::GtGtEq`] | [`symbols::BitXorEq`] | [`symbols::OrEq`])\n- `right`: `{_expression | lambda}` ([`Expression`] | [`Lambda`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AugmentedAssignment<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> AugmentedAssignment<'tree> {
    #[doc = "Get the field `left`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `op`.\n\nThis child has type `{%= | &= | **= | *= | += | -= | /= | <<= | >>= | ^= | |=}`:\n\n- [`symbols::ModEq`]\n- [`symbols::AndEq`]\n- [`symbols::MulMulEq`]\n- [`symbols::MulEq`]\n- [`symbols::AddEq`]\n- [`symbols::SubEq`]\n- [`symbols::DivEq`]\n- [`symbols::LtLtEq`]\n- [`symbols::GtGtEq`]\n- [`symbols::BitXorEq`]\n- [`symbols::OrEq`]\n"]
    #[inline]
    pub fn op(
        &self,
    ) -> ::type_sitter::NodeResult<
        'tree,
        anon_unions::ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<
            'tree,
        >,
    > {
        ::type_sitter::Node::raw(self).child_by_field_name("op").map(<anon_unions::ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw).expect("required child not present, there should at least be a MISSING node in its place")
    }

    #[doc = "Get the field `right`.\n\nThis child has type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AugmentedAssignment<'tree> {
    type WithLifetime<'a> = AugmentedAssignment<'a>;

    const KIND: &'static str = "augmented_assignment";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "augmented_assignment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "augmented_assignment");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `await_expression`\n\nThis node has a named child of type `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct AwaitExpression<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> AwaitExpression<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for AwaitExpression<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for AwaitExpression<'tree> {
    type WithLifetime<'a> = AwaitExpression<'a>;

    const KIND: &'static str = "await_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "await_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "await_expression");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `base_call`\n\nThis node has these fields:\n\n- `arguments`: `arguments` ([`Arguments`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BaseCall<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> BaseCall<'tree> {
    #[doc = "Get the field `arguments`.\n\nThis child has type `arguments` ([`Arguments`])"]
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, Arguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BaseCall<'tree> {
    type WithLifetime<'a> = BaseCall<'a>;

    const KIND: &'static str = "base_call";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "base_call" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "base_call");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `binary_operator`\n\nThis node has these fields:\n\n- `left`: `_primary_expression` ([`PrimaryExpression`])\n- `op`: `{!= | % | & | && | * | ** | + | - | / | < | << | <= | == | > | >= | >> | ^ | and | as | in | is | not | or | | | ||}+` ([`symbols::NotEq`] | [`symbols::Mod`] | [`symbols::And`] | [`symbols::AndAnd`] | [`symbols::Mul`] | [`symbols::MulMul`] | [`symbols::Add`] | [`symbols::Sub`] | [`symbols::Div`] | [`symbols::Lt`] | [`symbols::LtLt`] | [`symbols::LtEq`] | [`symbols::EqEq`] | [`symbols::Gt`] | [`symbols::GtEq`] | [`symbols::GtGt`] | [`symbols::BitXor`] | [`unnamed::And`] | [`unnamed::As`] | [`unnamed::In`] | [`unnamed::Is`] | [`unnamed::Not`] | [`unnamed::Or`] | [`symbols::Or`] | [`symbols::OrOr`])\n- `right`: `_primary_expression` ([`PrimaryExpression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BinaryOperator<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> BinaryOperator<'tree> {
    #[doc = "Get the field `left`.\n\nThis child has type `_primary_expression` ([`PrimaryExpression`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the children of field `op`.\n\nThese children have type `{!= | % | & | && | * | ** | + | - | / | < | << | <= | == | > | >= | >> | ^ | and | as | in | is | not | or | | | ||}+`:\n\n- [`symbols::NotEq`]\n- [`symbols::Mod`]\n- [`symbols::And`]\n- [`symbols::AndAnd`]\n- [`symbols::Mul`]\n- [`symbols::MulMul`]\n- [`symbols::Add`]\n- [`symbols::Sub`]\n- [`symbols::Div`]\n- [`symbols::Lt`]\n- [`symbols::LtLt`]\n- [`symbols::LtEq`]\n- [`symbols::EqEq`]\n- [`symbols::Gt`]\n- [`symbols::GtEq`]\n- [`symbols::GtGt`]\n- [`symbols::BitXor`]\n- [`unnamed::And`]\n- [`unnamed::As`]\n- [`unnamed::In`]\n- [`unnamed::Is`]\n- [`unnamed::Not`]\n- [`unnamed::Or`]\n- [`symbols::Or`]\n- [`symbols::OrOr`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn ops<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Anon19710622642415448518402396540615098358<'tree>,
        >,
    > + 'a {
        ::type_sitter::Node::raw(self).children_by_field_name("op", &mut c.0).map(<anon_unions::Anon19710622642415448518402396540615098358<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw)
    }

    #[doc = "Get the field `right`.\n\nThis child has type `_primary_expression` ([`PrimaryExpression`])"]
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, PrimaryExpression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(<PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BinaryOperator<'tree> {
    type WithLifetime<'a> = BinaryOperator<'a>;

    const KIND: &'static str = "binary_operator";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "binary_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "binary_operator");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `body`\n\nThis node has named children of type `{_compound_statement | annotation | break_statement | breakpoint_statement | class_name_statement | const_statement | continue_statement | export_variable_statement | expression_statement | extends_statement | onready_variable_statement | pass_statement | region_end | region_start | return_statement | signal_statement | variable_statement}*`:\n\n- [`CompoundStatement`]\n- [`Annotation`]\n- [`BreakStatement`]\n- [`BreakpointStatement`]\n- [`ClassNameStatement`]\n- [`ConstStatement`]\n- [`ContinueStatement`]\n- [`ExportVariableStatement`]\n- [`ExpressionStatement`]\n- [`ExtendsStatement`]\n- [`OnreadyVariableStatement`]\n- [`PassStatement`]\n- [`RegionEnd`]\n- [`RegionStart`]\n- [`ReturnStatement`]\n- [`SignalStatement`]\n- [`VariableStatement`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Body<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Body<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Body<'tree> {
    type Child = anon_unions::Anon266246878783060495398820561106992441731<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Body<'tree> {
    type WithLifetime<'a> = Body<'a>;

    const KIND: &'static str = "body";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "body" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "body");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `break_statement`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BreakStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> BreakStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BreakStatement<'tree> {
    type WithLifetime<'a> = BreakStatement<'a>;

    const KIND: &'static str = "break_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "break_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "break_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `breakpoint_statement`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct BreakpointStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> BreakpointStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for BreakpointStatement<'tree> {
    type WithLifetime<'a> = BreakpointStatement<'a>;

    const KIND: &'static str = "breakpoint_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "breakpoint_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "breakpoint_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `call`\n\nThis node has these fields:\n\n- `arguments`: `arguments` ([`Arguments`])\n\nAnd an additional named child of type `_primary_expression` ([`PrimaryExpression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Call<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Call<'tree> {
    #[doc = "Get the field `arguments`.\n\nThis child has type `arguments` ([`Arguments`])"]
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, Arguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `_primary_expression` ([`PrimaryExpression`])"]
    #[inline]
    pub fn primary_expression(&self) -> ::type_sitter::NodeResult<'tree, PrimaryExpression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Call<'tree> {
    type WithLifetime<'a> = Call<'a>;

    const KIND: &'static str = "call";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "call" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "call");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `class_body`\n\nThis node has named children of type `{class_definition | const_statement | enum_definition | extends_statement | function_definition | pass_statement | signal_statement | variable_statement}*`:\n\n- [`ClassDefinition`]\n- [`ConstStatement`]\n- [`EnumDefinition`]\n- [`ExtendsStatement`]\n- [`FunctionDefinition`]\n- [`PassStatement`]\n- [`SignalStatement`]\n- [`VariableStatement`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassBody<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassBody<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for ClassBody<'tree> {
    type Child = anon_unions::Anon194457999136656859162042825553197306441<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ClassBody<'tree> {
    type WithLifetime<'a> = ClassBody<'a>;

    const KIND: &'static str = "class_body";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "class_body" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_body");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `class_definition`\n\nThis node has these fields:\n\n- `body`: `class_body` ([`ClassBody`])\n- `extends`: `extends_statement?` ([`ExtendsStatement`])\n- `name`: `name` ([`Name`])\n\nAnd an optional additional named child of type `annotations?` ([`Annotations`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassDefinition<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassDefinition<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `class_body` ([`ClassBody`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, ClassBody<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<ClassBody<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `extends`.\n\nThis child has type `extends_statement?` ([`ExtendsStatement`])"]
    #[inline]
    pub fn extends(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, ExtendsStatement<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("extends")
            .map(<ExtendsStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child, if it has one.\n\nThis child has type `annotations?` ([`Annotations`])"]
    #[inline]
    pub fn annotations(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Annotations<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Annotations<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ClassDefinition<'tree> {
    type WithLifetime<'a> = ClassDefinition<'a>;

    const KIND: &'static str = "class_definition";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "class_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_definition");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `class_name_statement`\n\nThis node has these fields:\n\n- `extends`: `extends_statement?` ([`ExtendsStatement`])\n- `icon_path`: `string?` ([`String`])\n- `name`: `name` ([`Name`])\n\nAnd an optional additional named child of type `annotations?` ([`Annotations`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ClassNameStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ClassNameStatement<'tree> {
    #[doc = "Get the optional field `extends`.\n\nThis child has type `extends_statement?` ([`ExtendsStatement`])"]
    #[inline]
    pub fn extends(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, ExtendsStatement<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("extends")
            .map(<ExtendsStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `icon_path`.\n\nThis child has type `string?` ([`String`])"]
    #[inline]
    pub fn icon_path(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, String<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("icon_path")
            .map(<String<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child, if it has one.\n\nThis child has type `annotations?` ([`Annotations`])"]
    #[inline]
    pub fn annotations(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Annotations<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Annotations<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ClassNameStatement<'tree> {
    type WithLifetime<'a> = ClassNameStatement<'a>;

    const KIND: &'static str = "class_name_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "class_name_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "class_name_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `comment`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Comment<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Comment<'tree> {
    type WithLifetime<'a> = Comment<'a>;

    const KIND: &'static str = "comment";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "comment");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `conditional_expression`\n\nThis node has these fields:\n\n- `condition`: `_expression` ([`Expression`])\n- `left`: `_expression` ([`Expression`])\n- `right`: `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConditionalExpression<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ConditionalExpression<'tree> {
    #[doc = "Get the field `condition`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn condition(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `left`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `right`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ConditionalExpression<'tree> {
    type WithLifetime<'a> = ConditionalExpression<'a>;

    const KIND: &'static str = "conditional_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "conditional_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "conditional_expression");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `const_statement`\n\nThis node has these fields:\n\n- `name`: `name` ([`Name`])\n- `type`: `{inferred_type | type}?` ([`InferredType`] | [`Type`])\n- `value`: `{_expression | lambda}` ([`Expression`] | [`Lambda`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConstStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ConstStatement<'tree> {
    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `type`.\n\nThis child has type `{inferred_type | type}?`:\n\n- [`InferredType`]\n- [`Type`]\n"]
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::InferredType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(
                <anon_unions::InferredType_Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the field `value`.\n\nThis child has type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ConstStatement<'tree> {
    type WithLifetime<'a> = ConstStatement<'a>;

    const KIND: &'static str = "const_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "const_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "const_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `constructor_definition`\n\nThis node has these fields:\n\n- `arguments`: `arguments?` ([`Arguments`])\n- `body`: `body` ([`Body`])\n- `parameters`: `parameters` ([`Parameters_`])\n- `return_type`: `type?` ([`Type`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ConstructorDefinition<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ConstructorDefinition<'tree> {
    #[doc = "Get the optional field `arguments`.\n\nThis child has type `arguments?` ([`Arguments`])"]
    #[inline]
    pub fn arguments(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Arguments<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `parameters`.\n\nThis child has type `parameters` ([`Parameters_`])"]
    #[inline]
    pub fn parameters(&self) -> ::type_sitter::NodeResult<'tree, Parameters_<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<Parameters_<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `return_type`.\n\nThis child has type `type?` ([`Type`])"]
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("return_type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ConstructorDefinition<'tree> {
    type WithLifetime<'a> = ConstructorDefinition<'a>;

    const KIND: &'static str = "constructor_definition";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "constructor_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "constructor_definition");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `continue_statement`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ContinueStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ContinueStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ContinueStatement<'tree> {
    type WithLifetime<'a> = ContinueStatement<'a>;

    const KIND: &'static str = "continue_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "continue_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "continue_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `default_parameter`\n\nThis node has these fields:\n\n- `value`: `{_expression | lambda}` ([`Expression`] | [`Lambda`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct DefaultParameter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> DefaultParameter<'tree> {
    #[doc = "Get the field `value`.\n\nThis child has type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for DefaultParameter<'tree> {
    type WithLifetime<'a> = DefaultParameter<'a>;

    const KIND: &'static str = "default_parameter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "default_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "default_parameter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `dictionary`\n\nThis node has named children of type `{_primary_expression | pair | pattern_open_ending}*`:\n\n- [`PrimaryExpression`]\n- [`Pair`]\n- [`PatternOpenEnding`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Dictionary<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Dictionary<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Dictionary<'tree> {
    type Child = anon_unions::PrimaryExpression_Pair_PatternOpenEnding<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Dictionary<'tree> {
    type WithLifetime<'a> = Dictionary<'a>;

    const KIND: &'static str = "dictionary";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "dictionary" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "dictionary");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `elif_clause`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n- `condition`: `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ElifClause<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ElifClause<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `condition`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn condition(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ElifClause<'tree> {
    type WithLifetime<'a> = ElifClause<'a>;

    const KIND: &'static str = "elif_clause";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "elif_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "elif_clause");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `else_clause`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ElseClause<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ElseClause<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ElseClause<'tree> {
    type WithLifetime<'a> = ElseClause<'a>;

    const KIND: &'static str = "else_clause";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "else_clause" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "else_clause");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `enum_definition`\n\nThis node has these fields:\n\n- `body`: `enumerator_list` ([`EnumeratorList`])\n- `name`: `name?` ([`Name`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EnumDefinition<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> EnumDefinition<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `enumerator_list` ([`EnumeratorList`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, EnumeratorList<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<EnumeratorList<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `name`.\n\nThis child has type `name?` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Name<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EnumDefinition<'tree> {
    type WithLifetime<'a> = EnumDefinition<'a>;

    const KIND: &'static str = "enum_definition";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "enum_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "enum_definition");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `enumerator`\n\nThis node has these fields:\n\n- `left`: `identifier` ([`Identifier`])\n- `right`: `{attribute | binary_operator | call | identifier | integer | parenthesized_expression | subscript | unary_operator}?` ([`Attribute`] | [`BinaryOperator`] | [`Call`] | [`Identifier`] | [`Integer`] | [`ParenthesizedExpression`] | [`Subscript`] | [`UnaryOperator`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Enumerator<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Enumerator<'tree> {
    #[doc = "Get the field `left`.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `right`.\n\nThis child has type `{attribute | binary_operator | call | identifier | integer | parenthesized_expression | subscript | unary_operator}?`:\n\n- [`Attribute`]\n- [`BinaryOperator`]\n- [`Call`]\n- [`Identifier`]\n- [`Integer`]\n- [`ParenthesizedExpression`]\n- [`Subscript`]\n- [`UnaryOperator`]\n"]
    #[inline]
    pub fn right(&self) ->  ::std::option::Option< ::type_sitter::NodeResult<'tree,anon_unions::Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<'tree> > >{
        ::type_sitter::Node::raw(self).child_by_field_name("right").map(<anon_unions::Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Enumerator<'tree> {
    type WithLifetime<'a> = Enumerator<'a>;

    const KIND: &'static str = "enumerator";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "enumerator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "enumerator");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `enumerator_list`\n\nThis node has named children of type `enumerator+` ([`Enumerator`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EnumeratorList<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> EnumeratorList<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `enumerator+` ([`Enumerator`])"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn enumerators<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, Enumerator<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Enumerator<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for EnumeratorList<'tree> {
    type Child = Enumerator<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EnumeratorList<'tree> {
    type WithLifetime<'a> = EnumeratorList<'a>;

    const KIND: &'static str = "enumerator_list";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "enumerator_list" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "enumerator_list");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `escape_sequence`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> EscapeSequence<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for EscapeSequence<'tree> {
    type WithLifetime<'a> = EscapeSequence<'a>;

    const KIND: &'static str = "escape_sequence";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "escape_sequence");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `export_variable_statement`\n\nThis node has these fields:\n\n- `arguments`: `arguments?` ([`Arguments`])\n- `name`: `name` ([`Name`])\n- `setget`: `setget?` ([`Setget`])\n- `static`: `static_keyword?` ([`StaticKeyword`])\n- `type`: `{inferred_type | type}?` ([`InferredType`] | [`Type`])\n- `value`: `{_expression | lambda}?` ([`Expression`] | [`Lambda`])\n\nAnd additional named children of type `{annotations | remote_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExportVariableStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ExportVariableStatement<'tree> {
    #[doc = "Get the optional field `arguments`.\n\nThis child has type `arguments?` ([`Arguments`])"]
    #[inline]
    pub fn arguments(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Arguments<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `setget`.\n\nThis child has type `setget?` ([`Setget`])"]
    #[inline]
    pub fn setget(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Setget<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("setget")
            .map(<Setget<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `static`.\n\nThis child has type `static_keyword?` ([`StaticKeyword`])"]
    #[inline]
    pub fn r#static(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, StaticKeyword<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("static")
            .map(<StaticKeyword<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `type`.\n\nThis child has type `{inferred_type | type}?`:\n\n- [`InferredType`]\n- [`Type`]\n"]
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::InferredType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(
                <anon_unions::InferredType_Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the optional field `value`.\n\nThis child has type `{_expression | lambda}?`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the node's non-field not-extra named children.\n\nThese children have type `{annotations | remote_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n"]
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Annotations_RemoteKeyword<'tree>>,
    > + 'a {
        {
            let me =  * ::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self).named_children(&mut c.0).enumerate().filter(move|(i,n)| !n.is_extra()&&me.field_name_for_named_child(*i as _).is_none()).map(|(_,n)|n)
        }.map(<anon_unions::Annotations_RemoteKeyword<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExportVariableStatement<'tree> {
    type WithLifetime<'a> = ExportVariableStatement<'a>;

    const KIND: &'static str = "export_variable_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "export_variable_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "export_variable_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `expression_statement`\n\nThis node has a named child of type `{_expression | assignment | augmented_assignment}`:\n\n- [`Expression`]\n- [`Assignment`]\n- [`AugmentedAssignment`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExpressionStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ExpressionStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ExpressionStatement<'tree> {
    type Child = anon_unions::Expression_Assignment_AugmentedAssignment<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExpressionStatement<'tree> {
    type WithLifetime<'a> = ExpressionStatement<'a>;

    const KIND: &'static str = "expression_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "expression_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `extends_statement`\n\nThis node has a named child of type `{string | type}`:\n\n- [`String`]\n- [`Type`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ExtendsStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ExtendsStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ExtendsStatement<'tree> {
    type Child = anon_unions::String_Type<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ExtendsStatement<'tree> {
    type WithLifetime<'a> = ExtendsStatement<'a>;

    const KIND: &'static str = "extends_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "extends_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "extends_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `false`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct False<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> False<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for False<'tree> {
    type WithLifetime<'a> = False<'a>;

    const KIND: &'static str = "false";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "false");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `float`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Float<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Float<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Float<'tree> {
    type WithLifetime<'a> = Float<'a>;

    const KIND: &'static str = "float";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "float" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "float");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `for_statement`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n- `left`: `identifier` ([`Identifier`])\n- `right`: `_expression` ([`Expression`])\n- `type`: `type?` ([`Type`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ForStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ForStatement<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `left`.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn left(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("left")
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `right`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn right(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("right")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `type`.\n\nThis child has type `type?` ([`Type`])"]
    #[inline]
    pub fn r#type(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ForStatement<'tree> {
    type WithLifetime<'a> = ForStatement<'a>;

    const KIND: &'static str = "for_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "for_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "for_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `function_definition`\n\nThis node has these fields:\n\n- `body`: `body?` ([`Body`])\n- `name`: `name?` ([`Name`])\n- `parameters`: `parameters` ([`Parameters_`])\n- `return_type`: `type?` ([`Type`])\n\nAnd additional named children of type `{annotations | remote_keyword | static_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n- [`StaticKeyword`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct FunctionDefinition<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> FunctionDefinition<'tree> {
    #[doc = "Get the optional field `body`.\n\nThis child has type `body?` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Body<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `name`.\n\nThis child has type `name?` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Name<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `parameters`.\n\nThis child has type `parameters` ([`Parameters_`])"]
    #[inline]
    pub fn parameters(&self) -> ::type_sitter::NodeResult<'tree, Parameters_<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<Parameters_<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `return_type`.\n\nThis child has type `type?` ([`Type`])"]
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("return_type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the node's non-field not-extra named children.\n\nThese children have type `{annotations | remote_keyword | static_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n- [`StaticKeyword`]\n"]
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<
            'tree,
            anon_unions::Annotations_RemoteKeyword_StaticKeyword<'tree>,
        >,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
        .map(
            <anon_unions::Annotations_RemoteKeyword_StaticKeyword<'tree> as ::type_sitter::Node<
                'tree,
            >>::try_from_raw,
        )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for FunctionDefinition<'tree> {
    type WithLifetime<'a> = FunctionDefinition<'a>;

    const KIND: &'static str = "function_definition";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "function_definition" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "function_definition");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `get_body`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GetBody<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> GetBody<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for GetBody<'tree> {
    type WithLifetime<'a> = GetBody<'a>;

    const KIND: &'static str = "get_body";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "get_body" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "get_body");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `get_node`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct GetNode<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> GetNode<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for GetNode<'tree> {
    type WithLifetime<'a> = GetNode<'a>;

    const KIND: &'static str = "get_node";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "get_node" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "get_node");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `getter`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Getter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Getter<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Getter<'tree> {
    type WithLifetime<'a> = Getter<'a>;

    const KIND: &'static str = "getter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "getter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "getter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `identifier`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Identifier<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Identifier<'tree> {
    type WithLifetime<'a> = Identifier<'a>;

    const KIND: &'static str = "identifier";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "identifier");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `if_statement`\n\nThis node has these fields:\n\n- `alternative`: `{elif_clause | else_clause}*` ([`ElifClause`] | [`ElseClause`])\n- `body`: `body` ([`Body`])\n- `condition`: `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct IfStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> IfStatement<'tree> {
    #[doc = "Get the children of field `alternative`.\n\nThese children have type `{elif_clause | else_clause}*`:\n\n- [`ElifClause`]\n- [`ElseClause`]\n"]
    #[inline]
    pub fn alternatives<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::ElifClause_ElseClause<'tree>>,
    > + 'a {
        ::type_sitter::Node::raw(self).children_by_field_name("alternative", &mut c.0).map(<anon_unions::ElifClause_ElseClause<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw)
    }

    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `condition`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn condition(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for IfStatement<'tree> {
    type WithLifetime<'a> = IfStatement<'a>;

    const KIND: &'static str = "if_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "if_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "if_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `inferred_type`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct InferredType<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> InferredType<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for InferredType<'tree> {
    type WithLifetime<'a> = InferredType<'a>;

    const KIND: &'static str = "inferred_type";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "inferred_type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "inferred_type");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `integer`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Integer<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Integer<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Integer<'tree> {
    type WithLifetime<'a> = Integer<'a>;

    const KIND: &'static str = "integer";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "integer" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "integer");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `lambda`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n- `name`: `name?` ([`Name`])\n- `parameters`: `parameters` ([`Parameters_`])\n- `return_type`: `type?` ([`Type`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Lambda<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Lambda<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `name`.\n\nThis child has type `name?` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Name<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the field `parameters`.\n\nThis child has type `parameters` ([`Parameters_`])"]
    #[inline]
    pub fn parameters(&self) -> ::type_sitter::NodeResult<'tree, Parameters_<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<Parameters_<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `return_type`.\n\nThis child has type `type?` ([`Type`])"]
    #[inline]
    pub fn return_type(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Type<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("return_type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Lambda<'tree> {
    type WithLifetime<'a> = Lambda<'a>;

    const KIND: &'static str = "lambda";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "lambda" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "lambda");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `line_continuation`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct LineContinuation<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> LineContinuation<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for LineContinuation<'tree> {
    type WithLifetime<'a> = LineContinuation<'a>;

    const KIND: &'static str = "line_continuation";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "line_continuation" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "line_continuation");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `match_body`\n\nThis node has named children of type `{annotation | pattern_section}+`:\n\n- [`Annotation`]\n- [`PatternSection`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MatchBody<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> MatchBody<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for MatchBody<'tree> {
    type Child = anon_unions::Annotation_PatternSection<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for MatchBody<'tree> {
    type WithLifetime<'a> = MatchBody<'a>;

    const KIND: &'static str = "match_body";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "match_body" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "match_body");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `match_statement`\n\nThis node has these fields:\n\n- `body`: `match_body` ([`MatchBody`])\n- `value`: `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct MatchStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> MatchStatement<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `match_body` ([`MatchBody`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, MatchBody<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<MatchBody<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `value`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for MatchStatement<'tree> {
    type WithLifetime<'a> = MatchStatement<'a>;

    const KIND: &'static str = "match_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "match_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "match_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `name`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Name<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Name<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Name<'tree> {
    type WithLifetime<'a> = Name<'a>;

    const KIND: &'static str = "name";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "name");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `node_path`\n\nThis node has named children of type `escape_sequence*` ([`EscapeSequence`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct NodePath<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> NodePath<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `escape_sequence*` ([`EscapeSequence`])"]
    #[inline]
    pub fn escape_sequences<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, EscapeSequence<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<EscapeSequence<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for NodePath<'tree> {
    type Child = EscapeSequence<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for NodePath<'tree> {
    type WithLifetime<'a> = NodePath<'a>;

    const KIND: &'static str = "node_path";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "node_path" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "node_path");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `null`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Null<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Null<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Null<'tree> {
    type WithLifetime<'a> = Null<'a>;

    const KIND: &'static str = "null";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "null" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "null");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `onready_variable_statement`\n\nThis node has these fields:\n\n- `name`: `name` ([`Name`])\n- `setget`: `setget?` ([`Setget`])\n- `static`: `static_keyword?` ([`StaticKeyword`])\n- `type`: `{inferred_type | type}?` ([`InferredType`] | [`Type`])\n- `value`: `{_expression | lambda}?` ([`Expression`] | [`Lambda`])\n\nAnd an optional additional named child of type `annotations?` ([`Annotations`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct OnreadyVariableStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> OnreadyVariableStatement<'tree> {
    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `setget`.\n\nThis child has type `setget?` ([`Setget`])"]
    #[inline]
    pub fn setget(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Setget<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("setget")
            .map(<Setget<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `static`.\n\nThis child has type `static_keyword?` ([`StaticKeyword`])"]
    #[inline]
    pub fn r#static(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, StaticKeyword<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("static")
            .map(<StaticKeyword<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `type`.\n\nThis child has type `{inferred_type | type}?`:\n\n- [`InferredType`]\n- [`Type`]\n"]
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::InferredType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(
                <anon_unions::InferredType_Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the optional field `value`.\n\nThis child has type `{_expression | lambda}?`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the node's only non-field not-extra named child, if it has one.\n\nThis child has type `annotations?` ([`Annotations`])"]
    #[inline]
    pub fn annotations(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Annotations<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Annotations<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for OnreadyVariableStatement<'tree> {
    type WithLifetime<'a> = OnreadyVariableStatement<'a>;

    const KIND: &'static str = "onready_variable_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "onready_variable_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "onready_variable_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pair`\n\nThis node has these fields:\n\n- `left`: `{_expression | identifier | lambda}` ([`Expression`] | [`Identifier`] | [`Lambda`])\n- `value`: `{_expression | lambda | pattern_binding}` ([`Expression`] | [`Lambda`] | [`PatternBinding`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Pair<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Pair<'tree> {
    #[doc = "Get the field `left`.\n\nThis child has type `{_expression | identifier | lambda}`:\n\n- [`Expression`]\n- [`Identifier`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn left(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Identifier_Lambda<'tree>> {
        ::type_sitter::Node::raw(self).child_by_field_name("left").map(<anon_unions::Expression_Identifier_Lambda<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw).expect("required child not present, there should at least be a MISSING node in its place")
    }

    #[doc = "Get the field `value`.\n\nThis child has type `{_expression | lambda | pattern_binding}`:\n\n- [`Expression`]\n- [`Lambda`]\n- [`PatternBinding`]\n"]
    #[inline]
    pub fn value(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda_PatternBinding<'tree>>
    {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda_PatternBinding<'tree> as ::type_sitter::Node<
                    'tree,
                >>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Pair<'tree> {
    type WithLifetime<'a> = Pair<'a>;

    const KIND: &'static str = "pair";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pair" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pair");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `parameters`\n\nThis node has named children of type `_parameters*` ([`Parameters`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Parameters_<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Parameters_<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `_parameters*` ([`Parameters`])"]
    #[inline]
    pub fn parameterss<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, Parameters<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<Parameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Parameters_<'tree> {
    type Child = Parameters<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Parameters_<'tree> {
    type WithLifetime<'a> = Parameters_<'a>;

    const KIND: &'static str = "parameters";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "parameters" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parameters");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `parenthesized_expression`\n\nThis node has a named child of type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ParenthesizedExpression<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for ParenthesizedExpression<'tree> {
    type Child = anon_unions::Expression_Lambda<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ParenthesizedExpression<'tree> {
    type WithLifetime<'a> = ParenthesizedExpression<'a>;

    const KIND: &'static str = "parenthesized_expression";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "parenthesized_expression");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pass_statement`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PassStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> PassStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PassStatement<'tree> {
    type WithLifetime<'a> = PassStatement<'a>;

    const KIND: &'static str = "pass_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pass_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pass_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pattern_binding`\n\nThis node has a named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PatternBinding<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> PatternBinding<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for PatternBinding<'tree> {
    type Child = Identifier<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PatternBinding<'tree> {
    type WithLifetime<'a> = PatternBinding<'a>;

    const KIND: &'static str = "pattern_binding";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pattern_binding" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pattern_binding");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pattern_guard`\n\nThis node has a named child of type `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PatternGuard<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> PatternGuard<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn expression(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for PatternGuard<'tree> {
    type Child = Expression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PatternGuard<'tree> {
    type WithLifetime<'a> = PatternGuard<'a>;

    const KIND: &'static str = "pattern_guard";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pattern_guard" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pattern_guard");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pattern_open_ending`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PatternOpenEnding<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> PatternOpenEnding<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PatternOpenEnding<'tree> {
    type WithLifetime<'a> = PatternOpenEnding<'a>;

    const KIND: &'static str = "pattern_open_ending";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pattern_open_ending" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pattern_open_ending");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `pattern_section`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n\nAnd additional named children of type `{_pattern | pattern_guard}+`:\n\n- [`Pattern`]\n- [`PatternGuard`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct PatternSection<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> PatternSection<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's non-field not-extra named children.\n\nThese children have type `{_pattern | pattern_guard}+`:\n\n- [`Pattern`]\n- [`PatternGuard`]\n"]
    #[doc = "\n\nThis is guaranteed to return at least one child."]
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Pattern_PatternGuard<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
        .map(<anon_unions::Pattern_PatternGuard<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for PatternSection<'tree> {
    type WithLifetime<'a> = PatternSection<'a>;

    const KIND: &'static str = "pattern_section";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "pattern_section" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "pattern_section");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `region_end`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RegionEnd<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> RegionEnd<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RegionEnd<'tree> {
    type WithLifetime<'a> = RegionEnd<'a>;

    const KIND: &'static str = "region_end";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "region_end" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "region_end");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `region_label`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RegionLabel<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> RegionLabel<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RegionLabel<'tree> {
    type WithLifetime<'a> = RegionLabel<'a>;

    const KIND: &'static str = "region_label";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "region_label" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "region_label");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `region_start`\n\nThis node has an optional named child of type `region_label?` ([`RegionLabel`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RegionStart<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> RegionStart<'tree> {
    #[doc = "Get the node's only not-extra named child, if it has one.\n\nThis child has type `region_label?` ([`RegionLabel`])"]
    #[inline]
    pub fn region_label(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, RegionLabel<'tree>>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<RegionLabel<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasOptionalChild<'tree> for RegionStart<'tree> {
    type Child = RegionLabel<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RegionStart<'tree> {
    type WithLifetime<'a> = RegionStart<'a>;

    const KIND: &'static str = "region_start";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "region_start" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "region_start");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `remote_keyword`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct RemoteKeyword<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> RemoteKeyword<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for RemoteKeyword<'tree> {
    type WithLifetime<'a> = RemoteKeyword<'a>;

    const KIND: &'static str = "remote_keyword";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "remote_keyword" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "remote_keyword");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `return_statement`\n\nThis node has an optional named child of type `{_expression | lambda}?`:\n\n- [`Expression`]\n- [`Lambda`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct ReturnStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> ReturnStatement<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasOptionalChild<'tree> for ReturnStatement<'tree> {
    type Child = anon_unions::Expression_Lambda<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for ReturnStatement<'tree> {
    type WithLifetime<'a> = ReturnStatement<'a>;

    const KIND: &'static str = "return_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "return_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "return_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `set_body`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n\nAnd an additional named child of type `parameters` ([`Parameters_`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SetBody<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> SetBody<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `parameters` ([`Parameters_`])"]
    #[inline]
    pub fn parameters_(&self) -> ::type_sitter::NodeResult<'tree, Parameters_<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Parameters_<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SetBody<'tree> {
    type WithLifetime<'a> = SetBody<'a>;

    const KIND: &'static str = "set_body";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "set_body" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "set_body");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `setget`\n\nThis node has these fields:\n\n- `get`: `{get_body | getter}?` ([`GetBody`] | [`Getter`])\n- `set`: `{set_body | setter}?` ([`SetBody`] | [`Setter`])\n\nAnd additional named children of type `{getter | setter}*`:\n\n- [`Getter`]\n- [`Setter`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Setget<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Setget<'tree> {
    #[doc = "Get the optional field `get`.\n\nThis child has type `{get_body | getter}?`:\n\n- [`GetBody`]\n- [`Getter`]\n"]
    #[inline]
    pub fn get(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, anon_unions::GetBody_Getter<'tree>>>
    {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("get")
            .map(<anon_unions::GetBody_Getter<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `set`.\n\nThis child has type `{set_body | setter}?`:\n\n- [`SetBody`]\n- [`Setter`]\n"]
    #[inline]
    pub fn set(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, anon_unions::SetBody_Setter<'tree>>>
    {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("set")
            .map(<anon_unions::SetBody_Setter<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the node's non-field not-extra named children.\n\nThese children have type `{getter | setter}*`:\n\n- [`Getter`]\n- [`Setter`]\n"]
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Getter_Setter<'tree>>,
    > + 'a {
        {
            let me = *::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .enumerate()
                .filter(move |(i, n)| {
                    !n.is_extra() && me.field_name_for_named_child(*i as _).is_none()
                })
                .map(|(_, n)| n)
        }
        .map(<anon_unions::Getter_Setter<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Setget<'tree> {
    type WithLifetime<'a> = Setget<'a>;

    const KIND: &'static str = "setget";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "setget" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "setget");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `setter`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Setter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Setter<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Setter<'tree> {
    type WithLifetime<'a> = Setter<'a>;

    const KIND: &'static str = "setter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "setter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "setter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `signal_statement`\n\nThis node has these fields:\n\n- `name`: `name` ([`Name`])\n- `parameters`: `parameters?` ([`Parameters_`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SignalStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> SignalStatement<'tree> {
    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `parameters`.\n\nThis child has type `parameters?` ([`Parameters_`])"]
    #[inline]
    pub fn parameters(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Parameters_<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("parameters")
            .map(<Parameters_<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SignalStatement<'tree> {
    type WithLifetime<'a> = SignalStatement<'a>;

    const KIND: &'static str = "signal_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "signal_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "signal_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `source`\n\nThis node has named children of type `{_compound_statement | annotation | break_statement | breakpoint_statement | class_name_statement | const_statement | continue_statement | export_variable_statement | expression_statement | extends_statement | onready_variable_statement | pass_statement | region_end | region_start | return_statement | signal_statement | variable_statement}*`:\n\n- [`CompoundStatement`]\n- [`Annotation`]\n- [`BreakStatement`]\n- [`BreakpointStatement`]\n- [`ClassNameStatement`]\n- [`ConstStatement`]\n- [`ContinueStatement`]\n- [`ExportVariableStatement`]\n- [`ExpressionStatement`]\n- [`ExtendsStatement`]\n- [`OnreadyVariableStatement`]\n- [`PassStatement`]\n- [`RegionEnd`]\n- [`RegionStart`]\n- [`ReturnStatement`]\n- [`SignalStatement`]\n- [`VariableStatement`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Source<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Source<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for Source<'tree> {
    type Child = anon_unions::Anon266246878783060495398820561106992441731<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Source<'tree> {
    type WithLifetime<'a> = Source<'a>;

    const KIND: &'static str = "source";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "source" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "source");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `static_keyword`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StaticKeyword<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> StaticKeyword<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for StaticKeyword<'tree> {
    type WithLifetime<'a> = StaticKeyword<'a>;

    const KIND: &'static str = "static_keyword";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "static_keyword" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "static_keyword");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `string`\n\nThis node has named children of type `escape_sequence*` ([`EscapeSequence`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> String<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `escape_sequence*` ([`EscapeSequence`])"]
    #[inline]
    pub fn escape_sequences<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, EscapeSequence<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<EscapeSequence<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for String<'tree> {
    type Child = EscapeSequence<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for String<'tree> {
    type WithLifetime<'a> = String<'a>;

    const KIND: &'static str = "string";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `string_name`\n\nThis node has named children of type `escape_sequence*` ([`EscapeSequence`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct StringName<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> StringName<'tree> {
    #[doc = "Get the node's not-extra named children.\n\nThese children have type `escape_sequence*` ([`EscapeSequence`])"]
    #[inline]
    pub fn escape_sequences<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, EscapeSequence<'tree>>> + 'a
    {
        ::type_sitter::Node::raw(self)
            .named_children(&mut c.0)
            .filter(|n| !n.is_extra())
            .map(<EscapeSequence<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for StringName<'tree> {
    type Child = EscapeSequence<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for StringName<'tree> {
    type WithLifetime<'a> = StringName<'a>;

    const KIND: &'static str = "string_name";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "string_name" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "string_name");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `subscript`\n\nThis node has these fields:\n\n- `arguments`: `subscript_arguments` ([`SubscriptArguments`])\n\nAnd an additional named child of type `_primary_expression` ([`PrimaryExpression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Subscript<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Subscript<'tree> {
    #[doc = "Get the field `arguments`.\n\nThis child has type `subscript_arguments` ([`SubscriptArguments`])"]
    #[inline]
    pub fn arguments(&self) -> ::type_sitter::NodeResult<'tree, SubscriptArguments<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("arguments")
            .map(<SubscriptArguments<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `_primary_expression` ([`PrimaryExpression`])"]
    #[inline]
    pub fn primary_expression(&self) -> ::type_sitter::NodeResult<'tree, PrimaryExpression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Subscript<'tree> {
    type WithLifetime<'a> = Subscript<'a>;

    const KIND: &'static str = "subscript";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "subscript" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "subscript");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `subscript_arguments`\n\nThis node has named children of type `{_expression | lambda}+`:\n\n- [`Expression`]\n- [`Lambda`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct SubscriptArguments<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> SubscriptArguments<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChildren<'tree> for SubscriptArguments<'tree> {
    type Child = anon_unions::Expression_Lambda<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for SubscriptArguments<'tree> {
    type WithLifetime<'a> = SubscriptArguments<'a>;

    const KIND: &'static str = "subscript_arguments";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "subscript_arguments" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "subscript_arguments");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `true`\n\nThis node has no named children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct True<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> True<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for True<'tree> {
    type WithLifetime<'a> = True<'a>;

    const KIND: &'static str = "true";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "true");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `type`\n\nThis node has a named child of type `{attribute | identifier | subscript}`:\n\n- [`Attribute`]\n- [`Identifier`]\n- [`Subscript`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct Type<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> Type<'tree> {}

#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for Type<'tree> {
    type Child = anon_unions::Attribute_Identifier_Subscript<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for Type<'tree> {
    type WithLifetime<'a> = Type<'a>;

    const KIND: &'static str = "type";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "type" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "type");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `typed_default_parameter`\n\nThis node has these fields:\n\n- `type`: `{inferred_type | type}` ([`InferredType`] | [`Type`])\n- `value`: `{_expression | lambda}` ([`Expression`] | [`Lambda`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypedDefaultParameter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> TypedDefaultParameter<'tree> {
    #[doc = "Get the field `type`.\n\nThis child has type `{inferred_type | type}`:\n\n- [`InferredType`]\n- [`Type`]\n"]
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::type_sitter::NodeResult<'tree, anon_unions::InferredType_Type<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(
                <anon_unions::InferredType_Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `value`.\n\nThis child has type `{_expression | lambda}`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(&self) -> ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypedDefaultParameter<'tree> {
    type WithLifetime<'a> = TypedDefaultParameter<'a>;

    const KIND: &'static str = "typed_default_parameter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "typed_default_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "typed_default_parameter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `typed_parameter`\n\nThis node has these fields:\n\n- `type`: `type` ([`Type`])\n\nAnd an additional named child of type `identifier` ([`Identifier`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct TypedParameter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> TypedParameter<'tree> {
    #[doc = "Get the field `type`.\n\nThis child has type `type` ([`Type`])"]
    #[inline]
    pub fn r#type(&self) -> ::type_sitter::NodeResult<'tree, Type<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(<Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the node's only non-field not-extra named child.\n\nThis child has type `identifier` ([`Identifier`])"]
    #[inline]
    pub fn identifier(&self) -> ::type_sitter::NodeResult<'tree, Identifier<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .filter(|i| {
                ::type_sitter::Node::raw(self)
                    .field_name_for_named_child((*i).try_into().unwrap())
                    .is_none()
            })
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for TypedParameter<'tree> {
    type WithLifetime<'a> = TypedParameter<'a>;

    const KIND: &'static str = "typed_parameter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "typed_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "typed_parameter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `unary_operator`\n\nThis node has a named child of type `_primary_expression` ([`PrimaryExpression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct UnaryOperator<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> UnaryOperator<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `_primary_expression` ([`PrimaryExpression`])"]
    #[inline]
    pub fn primary_expression(&self) -> ::type_sitter::NodeResult<'tree, PrimaryExpression<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for UnaryOperator<'tree> {
    type Child = PrimaryExpression<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for UnaryOperator<'tree> {
    type WithLifetime<'a> = UnaryOperator<'a>;

    const KIND: &'static str = "unary_operator";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "unary_operator" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "unary_operator");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `variable_statement`\n\nThis node has these fields:\n\n- `name`: `name` ([`Name`])\n- `setget`: `setget?` ([`Setget`])\n- `static`: `static_keyword?` ([`StaticKeyword`])\n- `type`: `{inferred_type | type}?` ([`InferredType`] | [`Type`])\n- `value`: `{_expression | lambda}?` ([`Expression`] | [`Lambda`])\n\nAnd additional named children of type `{annotations | remote_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VariableStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> VariableStatement<'tree> {
    #[doc = "Get the field `name`.\n\nThis child has type `name` ([`Name`])"]
    #[inline]
    pub fn name(&self) -> ::type_sitter::NodeResult<'tree, Name<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("name")
            .map(<Name<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the optional field `setget`.\n\nThis child has type `setget?` ([`Setget`])"]
    #[inline]
    pub fn setget(&self) -> ::std::option::Option<::type_sitter::NodeResult<'tree, Setget<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("setget")
            .map(<Setget<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `static`.\n\nThis child has type `static_keyword?` ([`StaticKeyword`])"]
    #[inline]
    pub fn r#static(
        &self,
    ) -> ::std::option::Option<::type_sitter::NodeResult<'tree, StaticKeyword<'tree>>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("static")
            .map(<StaticKeyword<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
    }

    #[doc = "Get the optional field `type`.\n\nThis child has type `{inferred_type | type}?`:\n\n- [`InferredType`]\n- [`Type`]\n"]
    #[inline]
    pub fn r#type(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::InferredType_Type<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("type")
            .map(
                <anon_unions::InferredType_Type<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the optional field `value`.\n\nThis child has type `{_expression | lambda}?`:\n\n- [`Expression`]\n- [`Lambda`]\n"]
    #[inline]
    pub fn value(
        &self,
    ) -> ::std::option::Option<
        ::type_sitter::NodeResult<'tree, anon_unions::Expression_Lambda<'tree>>,
    > {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("value")
            .map(
                <anon_unions::Expression_Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw,
            )
    }

    #[doc = "Get the node's non-field not-extra named children.\n\nThese children have type `{annotations | remote_keyword}*`:\n\n- [`Annotations`]\n- [`RemoteKeyword`]\n"]
    #[inline]
    pub fn others<'a>(
        &self,
        c: &'a mut ::type_sitter::TreeCursor<'tree>,
    ) -> impl ::std::iter::Iterator<
        Item = ::type_sitter::NodeResult<'tree, anon_unions::Annotations_RemoteKeyword<'tree>>,
    > + 'a {
        {
            let me =  * ::type_sitter::Node::raw(self);
            ::type_sitter::Node::raw(self).named_children(&mut c.0).enumerate().filter(move|(i,n)| !n.is_extra()&&me.field_name_for_named_child(*i as _).is_none()).map(|(_,n)|n)
        }.map(<anon_unions::Annotations_RemoteKeyword<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw)
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VariableStatement<'tree> {
    type WithLifetime<'a> = VariableStatement<'a>;

    const KIND: &'static str = "variable_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variable_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variable_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `variadic_parameter`\n\nThis node has a named child of type `_parameters` ([`Parameters`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct VariadicParameter<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> VariadicParameter<'tree> {
    #[doc = "Get the node's only not-extra named child.\n\nThis child has type `_parameters` ([`Parameters`])"]
    #[inline]
    pub fn parameters(&self) -> ::type_sitter::NodeResult<'tree, Parameters<'tree>> {
        (0..::type_sitter::Node::raw(self).named_child_count())
            .map(|i| {
                ::type_sitter::Node::raw(self)
                    .named_child(i.try_into().unwrap())
                    .unwrap()
            })
            .filter(|n| !n.is_extra())
            .next()
            .map(<Parameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::HasChild<'tree> for VariadicParameter<'tree> {
    type Child = Parameters<'tree>;
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for VariadicParameter<'tree> {
    type WithLifetime<'a> = VariadicParameter<'a>;

    const KIND: &'static str = "variadic_parameter";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "variadic_parameter" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "variadic_parameter");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
#[doc = "Typed node `while_statement`\n\nThis node has these fields:\n\n- `body`: `body` ([`Body`])\n- `condition`: `_expression` ([`Expression`])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct WhileStatement<'tree>(::type_sitter::raw::Node<'tree>);

#[automatically_derived]
#[allow(unused)]
impl<'tree> WhileStatement<'tree> {
    #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
    #[inline]
    pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("body")
            .map(<Body<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }

    #[doc = "Get the field `condition`.\n\nThis child has type `_expression` ([`Expression`])"]
    #[inline]
    pub fn condition(&self) -> ::type_sitter::NodeResult<'tree, Expression<'tree>> {
        ::type_sitter::Node::raw(self)
            .child_by_field_name("condition")
            .map(<Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
            .expect(
                "required child not present, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> ::type_sitter::Node<'tree> for WhileStatement<'tree> {
    type WithLifetime<'a> = WhileStatement<'a>;

    const KIND: &'static str = "while_statement";

    #[inline]
    fn try_from_raw(
        node: ::type_sitter::raw::Node<'tree>,
    ) -> ::type_sitter::NodeResult<'tree, Self> {
        if node.kind() == "while_statement" {
            Ok(Self(node))
        } else {
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }
    }

    #[inline]
    unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
        debug_assert_eq!(node.kind(), "while_statement");
        Self(node)
    }

    #[inline]
    fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
        &self.0
    }

    #[inline]
    fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
        &mut self.0
    }

    #[inline]
    fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
        self.0
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `_init`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Init<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Init<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Init<'tree> {
        type WithLifetime<'a> = Init<'a>;

        const KIND: &'static str = "_init";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "_init" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "_init");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `and`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;

        const KIND: &'static str = "and";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "and" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "and");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `as`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct As<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> As<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for As<'tree> {
        type WithLifetime<'a> = As<'a>;

        const KIND: &'static str = "as";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "as" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "as");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `await`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Await<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Await<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Await<'tree> {
        type WithLifetime<'a> = Await<'a>;

        const KIND: &'static str = "await";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "await" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "await");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `break`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Break<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Break<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Break<'tree> {
        type WithLifetime<'a> = Break<'a>;

        const KIND: &'static str = "break";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "break" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "break");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `class`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Class<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Class<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Class<'tree> {
        type WithLifetime<'a> = Class<'a>;

        const KIND: &'static str = "class";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "class" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "class");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `class_name`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ClassName<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ClassName<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ClassName<'tree> {
        type WithLifetime<'a> = ClassName<'a>;

        const KIND: &'static str = "class_name";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "class_name" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "class_name");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `const`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Const<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Const<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Const<'tree> {
        type WithLifetime<'a> = Const<'a>;

        const KIND: &'static str = "const";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "const" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "const");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `continue`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Continue<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Continue<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Continue<'tree> {
        type WithLifetime<'a> = Continue<'a>;

        const KIND: &'static str = "continue";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "continue" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "continue");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `elif`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Elif<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Elif<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Elif<'tree> {
        type WithLifetime<'a> = Elif<'a>;

        const KIND: &'static str = "elif";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "elif" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "elif");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `else`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Else<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Else<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Else<'tree> {
        type WithLifetime<'a> = Else<'a>;

        const KIND: &'static str = "else";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "else" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "else");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `enum`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Enum<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Enum<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Enum<'tree> {
        type WithLifetime<'a> = Enum<'a>;

        const KIND: &'static str = "enum";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "enum" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "enum");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `export`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Export<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Export<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Export<'tree> {
        type WithLifetime<'a> = Export<'a>;

        const KIND: &'static str = "export";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "export" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "export");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `extends`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Extends<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Extends<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Extends<'tree> {
        type WithLifetime<'a> = Extends<'a>;

        const KIND: &'static str = "extends";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "extends" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "extends");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `for`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct For<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> For<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for For<'tree> {
        type WithLifetime<'a> = For<'a>;

        const KIND: &'static str = "for";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "for" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "for");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `func`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Func<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Func<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Func<'tree> {
        type WithLifetime<'a> = Func<'a>;

        const KIND: &'static str = "func";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "func" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "func");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `get`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Get<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Get<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Get<'tree> {
        type WithLifetime<'a> = Get<'a>;

        const KIND: &'static str = "get";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "get" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "get");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `if`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct If<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> If<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for If<'tree> {
        type WithLifetime<'a> = If<'a>;

        const KIND: &'static str = "if";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "if" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "if");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `in`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct In<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> In<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for In<'tree> {
        type WithLifetime<'a> = In<'a>;

        const KIND: &'static str = "in";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "in" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "in");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `is`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Is<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Is<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Is<'tree> {
        type WithLifetime<'a> = Is<'a>;

        const KIND: &'static str = "is";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "is" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "is");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `master`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Master<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Master<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Master<'tree> {
        type WithLifetime<'a> = Master<'a>;

        const KIND: &'static str = "master";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "master" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "master");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `mastersync`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mastersync<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mastersync<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mastersync<'tree> {
        type WithLifetime<'a> = Mastersync<'a>;

        const KIND: &'static str = "mastersync";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "mastersync" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "mastersync");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `match`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Match<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Match<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Match<'tree> {
        type WithLifetime<'a> = Match<'a>;

        const KIND: &'static str = "match";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "match" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "match");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `not`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Not<'tree> {
        type WithLifetime<'a> = Not<'a>;

        const KIND: &'static str = "not";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "not" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "not");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `onready`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Onready<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Onready<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Onready<'tree> {
        type WithLifetime<'a> = Onready<'a>;

        const KIND: &'static str = "onready";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "onready" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "onready");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `or`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;

        const KIND: &'static str = "or";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "or" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "or");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `pass`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Pass<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Pass<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Pass<'tree> {
        type WithLifetime<'a> = Pass<'a>;

        const KIND: &'static str = "pass";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "pass" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "pass");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `puppet`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Puppet<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Puppet<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Puppet<'tree> {
        type WithLifetime<'a> = Puppet<'a>;

        const KIND: &'static str = "puppet";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "puppet" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "puppet");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `puppetsync`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Puppetsync<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Puppetsync<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Puppetsync<'tree> {
        type WithLifetime<'a> = Puppetsync<'a>;

        const KIND: &'static str = "puppetsync";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "puppetsync" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "puppetsync");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `remote`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Remote<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Remote<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Remote<'tree> {
        type WithLifetime<'a> = Remote<'a>;

        const KIND: &'static str = "remote";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "remote" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "remote");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `remotesync`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Remotesync<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Remotesync<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Remotesync<'tree> {
        type WithLifetime<'a> = Remotesync<'a>;

        const KIND: &'static str = "remotesync";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "remotesync" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "remotesync");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `return`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Return<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Return<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Return<'tree> {
        type WithLifetime<'a> = Return<'a>;

        const KIND: &'static str = "return";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "return" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "return");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `set`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Set<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Set<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Set<'tree> {
        type WithLifetime<'a> = Set<'a>;

        const KIND: &'static str = "set";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "set" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "set");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `setget`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Setget<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Setget<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Setget<'tree> {
        type WithLifetime<'a> = Setget<'a>;

        const KIND: &'static str = "setget";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "setget" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "setget");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `signal`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Signal<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Signal<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Signal<'tree> {
        type WithLifetime<'a> = Signal<'a>;

        const KIND: &'static str = "signal";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "signal" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "signal");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `value`\n\nThis node has named children of type `escape_sequence*` ([`EscapeSequence`])\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Value<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Value<'tree> {
        #[doc = "Get the node's not-extra named children.\n\nThese children have type `escape_sequence*` ([`EscapeSequence`])"]
        #[inline]
        pub fn escape_sequences<'a>(
            &self,
            c: &'a mut ::type_sitter::TreeCursor<'tree>,
        ) -> impl ::std::iter::Iterator<
            Item = ::type_sitter::NodeResult<'tree, EscapeSequence<'tree>>,
        > + 'a {
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .filter(|n| !n.is_extra())
                .map(<EscapeSequence<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::HasChildren<'tree> for Value<'tree> {
        type Child = EscapeSequence<'tree>;
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Value<'tree> {
        type WithLifetime<'a> = Value<'a>;

        const KIND: &'static str = "value";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "value" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "value");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `var`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Var<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Var<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Var<'tree> {
        type WithLifetime<'a> = Var<'a>;

        const KIND: &'static str = "var";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "var" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "var");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `when`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct When<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> When<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for When<'tree> {
        type WithLifetime<'a> = When<'a>;

        const KIND: &'static str = "when";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "when" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "when");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `while`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct While<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> While<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for While<'tree> {
        type WithLifetime<'a> = While<'a>;

        const KIND: &'static str = "while";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "while" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "while");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `!`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Not<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Not<'tree> {
        type WithLifetime<'a> = Not<'a>;

        const KIND: &'static str = "!";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `!=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct NotEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> NotEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for NotEq<'tree> {
        type WithLifetime<'a> = NotEq<'a>;

        const KIND: &'static str = "!=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "!=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `\"`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DoubleQuote<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DoubleQuote<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DoubleQuote<'tree> {
        type WithLifetime<'a> = DoubleQuote<'a>;

        const KIND: &'static str = "\"";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "\"" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "\"");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `#region`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Hashregion<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Hashregion<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Hashregion<'tree> {
        type WithLifetime<'a> = Hashregion<'a>;

        const KIND: &'static str = "#region";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "#region" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "#region");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `$`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dollar<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dollar<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Dollar<'tree> {
        type WithLifetime<'a> = Dollar<'a>;

        const KIND: &'static str = "$";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "$" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "$");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `%`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mod<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mod<'tree> {
        type WithLifetime<'a> = Mod<'a>;

        const KIND: &'static str = "%";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "%");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `%=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ModEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ModEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ModEq<'tree> {
        type WithLifetime<'a> = ModEq<'a>;

        const KIND: &'static str = "%=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "%=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "%=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `&`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> And<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for And<'tree> {
        type WithLifetime<'a> = And<'a>;

        const KIND: &'static str = "&";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `&\"`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AndDoubleQuote<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AndDoubleQuote<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AndDoubleQuote<'tree> {
        type WithLifetime<'a> = AndDoubleQuote<'a>;

        const KIND: &'static str = "&\"";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&\"" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&\"");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `&&`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AndAnd<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AndAnd<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AndAnd<'tree> {
        type WithLifetime<'a> = AndAnd<'a>;

        const KIND: &'static str = "&&";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&&" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&&");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `&=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AndEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AndEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AndEq<'tree> {
        type WithLifetime<'a> = AndEq<'a>;

        const KIND: &'static str = "&=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "&=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "&=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `(`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParen<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParen<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LParen<'tree> {
        type WithLifetime<'a> = LParen<'a>;

        const KIND: &'static str = "(";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "(");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `()`\n\nThis node has named children of type `_parameters*` ([`Parameters`])\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LParenRParen<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LParenRParen<'tree> {
        #[doc = "Get the node's not-extra named children.\n\nThese children have type `_parameters*` ([`Parameters`])"]
        #[inline]
        pub fn parameterss<'a>(
            &self,
            c: &'a mut ::type_sitter::TreeCursor<'tree>,
        ) -> impl ::std::iter::Iterator<Item = ::type_sitter::NodeResult<'tree, Parameters<'tree>>> + 'a
        {
            ::type_sitter::Node::raw(self)
                .named_children(&mut c.0)
                .filter(|n| !n.is_extra())
                .map(<Parameters<'tree> as ::type_sitter::Node<'tree>>::try_from_raw)
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::HasChildren<'tree> for LParenRParen<'tree> {
        type Child = Parameters<'tree>;
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LParenRParen<'tree> {
        type WithLifetime<'a> = LParenRParen<'a>;

        const KIND: &'static str = "()";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "()" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "()");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `)`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RParen<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RParen<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RParen<'tree> {
        type WithLifetime<'a> = RParen<'a>;

        const KIND: &'static str = ")";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ")");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `*`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Mul<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Mul<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Mul<'tree> {
        type WithLifetime<'a> = Mul<'a>;

        const KIND: &'static str = "*";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `**`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulMul<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulMul<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for MulMul<'tree> {
        type WithLifetime<'a> = MulMul<'a>;

        const KIND: &'static str = "**";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "**" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "**");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `**=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulMulEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulMulEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for MulMulEq<'tree> {
        type WithLifetime<'a> = MulMulEq<'a>;

        const KIND: &'static str = "**=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "**=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "**=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `*=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct MulEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> MulEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for MulEq<'tree> {
        type WithLifetime<'a> = MulEq<'a>;

        const KIND: &'static str = "*=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "*=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "*=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `+`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Add<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Add<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Add<'tree> {
        type WithLifetime<'a> = Add<'a>;

        const KIND: &'static str = "+";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `+=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct AddEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AddEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for AddEq<'tree> {
        type WithLifetime<'a> = AddEq<'a>;

        const KIND: &'static str = "+=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "+=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "+=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `,`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Comma<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Comma<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Comma<'tree> {
        type WithLifetime<'a> = Comma<'a>;

        const KIND: &'static str = ",";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ",");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `-`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Sub<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Sub<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Sub<'tree> {
        type WithLifetime<'a> = Sub<'a>;

        const KIND: &'static str = "-";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `-=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubEq<'tree> {
        type WithLifetime<'a> = SubEq<'a>;

        const KIND: &'static str = "-=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "-=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "-=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `->`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct SubGt<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SubGt<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SubGt<'tree> {
        type WithLifetime<'a> = SubGt<'a>;

        const KIND: &'static str = "->";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "->");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `.`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Dot<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Dot<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Dot<'tree> {
        type WithLifetime<'a> = Dot<'a>;

        const KIND: &'static str = ".";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ".");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `...`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DotDotDot<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DotDotDot<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DotDotDot<'tree> {
        type WithLifetime<'a> = DotDotDot<'a>;

        const KIND: &'static str = "...";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "..." {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "...");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `/`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Div<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Div<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Div<'tree> {
        type WithLifetime<'a> = Div<'a>;

        const KIND: &'static str = "/";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `/=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct DivEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> DivEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for DivEq<'tree> {
        type WithLifetime<'a> = DivEq<'a>;

        const KIND: &'static str = "/=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "/=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "/=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `:`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Colon<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Colon<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Colon<'tree> {
        type WithLifetime<'a> = Colon<'a>;

        const KIND: &'static str = ":";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `:=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct ColonEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ColonEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ColonEq<'tree> {
        type WithLifetime<'a> = ColonEq<'a>;

        const KIND: &'static str = ":=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ":=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ":=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `;`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Semicolon<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Semicolon<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Semicolon<'tree> {
        type WithLifetime<'a> = Semicolon<'a>;

        const KIND: &'static str = ";";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ";" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ";");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `<`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Lt<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Lt<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Lt<'tree> {
        type WithLifetime<'a> = Lt<'a>;

        const KIND: &'static str = "<";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `<<`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtLt<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtLt<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtLt<'tree> {
        type WithLifetime<'a> = LtLt<'a>;

        const KIND: &'static str = "<<";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<<" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<<");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `<<=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtLtEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtLtEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtLtEq<'tree> {
        type WithLifetime<'a> = LtLtEq<'a>;

        const KIND: &'static str = "<<=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<<=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `<=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LtEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LtEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LtEq<'tree> {
        type WithLifetime<'a> = LtEq<'a>;

        const KIND: &'static str = "<=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "<=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Eq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Eq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Eq<'tree> {
        type WithLifetime<'a> = Eq<'a>;

        const KIND: &'static str = "=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `==`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct EqEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> EqEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for EqEq<'tree> {
        type WithLifetime<'a> = EqEq<'a>;

        const KIND: &'static str = "==";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "==");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `>`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Gt<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Gt<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Gt<'tree> {
        type WithLifetime<'a> = Gt<'a>;

        const KIND: &'static str = ">";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `>=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtEq<'tree> {
        type WithLifetime<'a> = GtEq<'a>;

        const KIND: &'static str = ">=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `>>`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtGt<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtGt<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtGt<'tree> {
        type WithLifetime<'a> = GtGt<'a>;

        const KIND: &'static str = ">>";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">>" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">>");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `>>=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct GtGtEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GtGtEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GtGtEq<'tree> {
        type WithLifetime<'a> = GtGtEq<'a>;

        const KIND: &'static str = ">>=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == ">>=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), ">>=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `@`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct At<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> At<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for At<'tree> {
        type WithLifetime<'a> = At<'a>;

        const KIND: &'static str = "@";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "@" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "@");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `[`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBracket<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBracket<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBracket<'tree> {
        type WithLifetime<'a> = LBracket<'a>;

        const KIND: &'static str = "[";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "[");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `]`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBracket<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBracket<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBracket<'tree> {
        type WithLifetime<'a> = RBracket<'a>;

        const KIND: &'static str = "]";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "]");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `^`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXor<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXor<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitXor<'tree> {
        type WithLifetime<'a> = BitXor<'a>;

        const KIND: &'static str = "^";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `^\"`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXorDoubleQuote<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXorDoubleQuote<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitXorDoubleQuote<'tree> {
        type WithLifetime<'a> = BitXorDoubleQuote<'a>;

        const KIND: &'static str = "^\"";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "^\"" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^\"");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `^=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitXorEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitXorEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitXorEq<'tree> {
        type WithLifetime<'a> = BitXorEq<'a>;

        const KIND: &'static str = "^=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "^=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "^=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `{`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct LBrace<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> LBrace<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for LBrace<'tree> {
        type WithLifetime<'a> = LBrace<'a>;

        const KIND: &'static str = "{";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "{");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `|`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Or<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Or<'tree> {
        type WithLifetime<'a> = Or<'a>;

        const KIND: &'static str = "|";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `|=`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct OrEq<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> OrEq<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for OrEq<'tree> {
        type WithLifetime<'a> = OrEq<'a>;

        const KIND: &'static str = "|=";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "|=" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "|=");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `||`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct OrOr<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> OrOr<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for OrOr<'tree> {
        type WithLifetime<'a> = OrOr<'a>;

        const KIND: &'static str = "||";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "||" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "||");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `}`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct RBrace<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> RBrace<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for RBrace<'tree> {
        type WithLifetime<'a> = RBrace<'a>;

        const KIND: &'static str = "}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "}");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
    #[doc = "Typed node `~`\n\nThis node has no named children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    #[allow(non_camel_case_types)]
    pub struct BitNot<'tree>(::type_sitter::raw::Node<'tree>);

    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> BitNot<'tree> {}

    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for BitNot<'tree> {
        type WithLifetime<'a> = BitNot<'a>;

        const KIND: &'static str = "~";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(::type_sitter::IncorrectKind::new::<Self>(node))
            }
        }

        #[inline]
        unsafe fn from_raw_unchecked(node: ::type_sitter::raw::Node<'tree>) -> Self {
            debug_assert_eq!(node.kind(), "~");
            Self(node)
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            &self.0
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            &mut self.0
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            self.0
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "One of `{annotation | pattern_section}`:\n- [`Annotation`]\n- [`PatternSection`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Annotation_PatternSection<'tree> {
        Annotation(Annotation<'tree>),
        PatternSection(PatternSection<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Annotation_PatternSection<'tree> {
        #[doc = "Returns the node if it is of type `annotation` ([`Annotation`]), otherwise returns `None`"]
        #[inline]
        pub fn as_annotation(self) -> ::std::option::Option<Annotation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Annotation(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_section` ([`PatternSection`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_section(self) -> ::std::option::Option<PatternSection<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternSection(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Annotation_PatternSection<'tree> {
        type WithLifetime<'a> = Annotation_PatternSection<'a>;

        const KIND: &'static str = "{annotation | pattern_section}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "annotation" => Ok(unsafe {
                    Self::Annotation(
                        <Annotation<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "pattern_section" => {
                    Ok(unsafe {
                        Self::PatternSection(<PatternSection<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotation(x) => ::type_sitter::Node::raw(x),
                Self::PatternSection(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotation(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternSection(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotation(x) => x.into_raw(),
                Self::PatternSection(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{annotations | remote_keyword}`:\n- [`Annotations`]\n- [`RemoteKeyword`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Annotations_RemoteKeyword<'tree> {
        Annotations(Annotations<'tree>),
        RemoteKeyword(RemoteKeyword<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Annotations_RemoteKeyword<'tree> {
        #[doc = "Returns the node if it is of type `annotations` ([`Annotations`]), otherwise returns `None`"]
        #[inline]
        pub fn as_annotations(self) -> ::std::option::Option<Annotations<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Annotations(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `remote_keyword` ([`RemoteKeyword`]), otherwise returns `None`"]
        #[inline]
        pub fn as_remote_keyword(self) -> ::std::option::Option<RemoteKeyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RemoteKeyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Annotations_RemoteKeyword<'tree> {
        type WithLifetime<'a> = Annotations_RemoteKeyword<'a>;

        const KIND: &'static str = "{annotations | remote_keyword}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "annotations" => Ok(unsafe {
                    Self::Annotations(
                        <Annotations<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "remote_keyword" => {
                    Ok(unsafe {
                        Self::RemoteKeyword(<RemoteKeyword<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => ::type_sitter::Node::raw(x),
                Self::RemoteKeyword(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => ::type_sitter::Node::raw_mut(x),
                Self::RemoteKeyword(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => x.into_raw(),
                Self::RemoteKeyword(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{annotations | remote_keyword | static_keyword}`:\n- [`Annotations`]\n- [`RemoteKeyword`]\n- [`StaticKeyword`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Annotations_RemoteKeyword_StaticKeyword<'tree> {
        Annotations(Annotations<'tree>),
        RemoteKeyword(RemoteKeyword<'tree>),
        StaticKeyword(StaticKeyword<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Annotations_RemoteKeyword_StaticKeyword<'tree> {
        #[doc = "Returns the node if it is of type `annotations` ([`Annotations`]), otherwise returns `None`"]
        #[inline]
        pub fn as_annotations(self) -> ::std::option::Option<Annotations<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Annotations(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `remote_keyword` ([`RemoteKeyword`]), otherwise returns `None`"]
        #[inline]
        pub fn as_remote_keyword(self) -> ::std::option::Option<RemoteKeyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RemoteKeyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `static_keyword` ([`StaticKeyword`]), otherwise returns `None`"]
        #[inline]
        pub fn as_static_keyword(self) -> ::std::option::Option<StaticKeyword<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::StaticKeyword(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Annotations_RemoteKeyword_StaticKeyword<'tree> {
        type WithLifetime<'a> = Annotations_RemoteKeyword_StaticKeyword<'a>;

        const KIND: &'static str = "{annotations | remote_keyword | static_keyword}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "annotations" => Ok(unsafe {
                    Self::Annotations(
                        <Annotations<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "remote_keyword" => {
                    Ok(unsafe {
                        Self::RemoteKeyword(<RemoteKeyword<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "static_keyword" => {
                    Ok(unsafe {
                        Self::StaticKeyword(<StaticKeyword<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => ::type_sitter::Node::raw(x),
                Self::RemoteKeyword(x) => ::type_sitter::Node::raw(x),
                Self::StaticKeyword(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => ::type_sitter::Node::raw_mut(x),
                Self::RemoteKeyword(x) => ::type_sitter::Node::raw_mut(x),
                Self::StaticKeyword(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Annotations(x) => x.into_raw(),
                Self::RemoteKeyword(x) => x.into_raw(),
                Self::StaticKeyword(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{class_definition | const_statement | enum_definition | extends_statement | function_definition | pass_statement | signal_statement | variable_statement}`:\n- [`ClassDefinition`]\n- [`ConstStatement`]\n- [`EnumDefinition`]\n- [`ExtendsStatement`]\n- [`FunctionDefinition`]\n- [`PassStatement`]\n- [`SignalStatement`]\n- [`VariableStatement`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon194457999136656859162042825553197306441<'tree> {
        ClassDefinition(ClassDefinition<'tree>),
        ConstStatement(ConstStatement<'tree>),
        EnumDefinition(EnumDefinition<'tree>),
        ExtendsStatement(ExtendsStatement<'tree>),
        FunctionDefinition(FunctionDefinition<'tree>),
        PassStatement(PassStatement<'tree>),
        SignalStatement(SignalStatement<'tree>),
        VariableStatement(VariableStatement<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon194457999136656859162042825553197306441<'tree> {
        #[doc = "Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`"]
        #[inline]
        pub fn as_class_definition(self) -> ::std::option::Option<ClassDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `const_statement` ([`ConstStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_const_statement(self) -> ::std::option::Option<ConstStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConstStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `enum_definition` ([`EnumDefinition`]), otherwise returns `None`"]
        #[inline]
        pub fn as_enum_definition(self) -> ::std::option::Option<EnumDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EnumDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `extends_statement` ([`ExtendsStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_extends_statement(self) -> ::std::option::Option<ExtendsStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExtendsStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`"]
        #[inline]
        pub fn as_function_definition(self) -> ::std::option::Option<FunctionDefinition<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::FunctionDefinition(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pass_statement` ([`PassStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pass_statement(self) -> ::std::option::Option<PassStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PassStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `signal_statement` ([`SignalStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_signal_statement(self) -> ::std::option::Option<SignalStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SignalStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `variable_statement` ([`VariableStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_variable_statement(self) -> ::std::option::Option<VariableStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Anon194457999136656859162042825553197306441<'tree> {
        type WithLifetime<'a> = Anon194457999136656859162042825553197306441<'a>;

        const KIND: &'static str = "{class_definition | const_statement | enum_definition | extends_statement | function_definition | pass_statement | signal_statement | variable_statement}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "class_definition" => {
                    Ok(unsafe {
                        Self::ClassDefinition(<ClassDefinition<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "const_statement" => {
                    Ok(unsafe {
                        Self::ConstStatement(<ConstStatement<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "enum_definition" => {
                    Ok(unsafe {
                        Self::EnumDefinition(<EnumDefinition<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "extends_statement" => Ok(unsafe {
                    Self::ExtendsStatement(<ExtendsStatement<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                "function_definition" => {
                    Ok(unsafe {
                        Self::FunctionDefinition(<FunctionDefinition<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "pass_statement" => {
                    Ok(unsafe {
                        Self::PassStatement(<PassStatement<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "signal_statement" => {
                    Ok(unsafe {
                        Self::SignalStatement(<SignalStatement<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "variable_statement" => Ok(unsafe {
                    Self::VariableStatement(<VariableStatement<'tree> as ::type_sitter::Node<
                        'tree,
                    >>::from_raw_unchecked(node))
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => ::type_sitter::Node::raw(x),
                Self::ConstStatement(x) => ::type_sitter::Node::raw(x),
                Self::EnumDefinition(x) => ::type_sitter::Node::raw(x),
                Self::ExtendsStatement(x) => ::type_sitter::Node::raw(x),
                Self::FunctionDefinition(x) => ::type_sitter::Node::raw(x),
                Self::PassStatement(x) => ::type_sitter::Node::raw(x),
                Self::SignalStatement(x) => ::type_sitter::Node::raw(x),
                Self::VariableStatement(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::ConstStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::EnumDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::ExtendsStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::FunctionDefinition(x) => ::type_sitter::Node::raw_mut(x),
                Self::PassStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::SignalStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableStatement(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ClassDefinition(x) => x.into_raw(),
                Self::ConstStatement(x) => x.into_raw(),
                Self::EnumDefinition(x) => x.into_raw(),
                Self::ExtendsStatement(x) => x.into_raw(),
                Self::FunctionDefinition(x) => x.into_raw(),
                Self::PassStatement(x) => x.into_raw(),
                Self::SignalStatement(x) => x.into_raw(),
                Self::VariableStatement(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{!= | % | & | && | * | ** | + | - | / | < | << | <= | == | > | >= | >> | ^ | and | as | in | is | not | or | | | ||}`:\n- [`symbols::NotEq`]\n- [`symbols::Mod`]\n- [`symbols::And`]\n- [`symbols::AndAnd`]\n- [`symbols::Mul`]\n- [`symbols::MulMul`]\n- [`symbols::Add`]\n- [`symbols::Sub`]\n- [`symbols::Div`]\n- [`symbols::Lt`]\n- [`symbols::LtLt`]\n- [`symbols::LtEq`]\n- [`symbols::EqEq`]\n- [`symbols::Gt`]\n- [`symbols::GtEq`]\n- [`symbols::GtGt`]\n- [`symbols::BitXor`]\n- [`unnamed::And`]\n- [`unnamed::As`]\n- [`unnamed::In`]\n- [`unnamed::Is`]\n- [`unnamed::Not`]\n- [`unnamed::Or`]\n- [`symbols::Or`]\n- [`symbols::OrOr`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon19710622642415448518402396540615098358<'tree> {
        NotEq(symbols::NotEq<'tree>),
        Mod(symbols::Mod<'tree>),
        And(symbols::And<'tree>),
        AndAnd(symbols::AndAnd<'tree>),
        Mul(symbols::Mul<'tree>),
        MulMul(symbols::MulMul<'tree>),
        Add(symbols::Add<'tree>),
        Sub(symbols::Sub<'tree>),
        Div(symbols::Div<'tree>),
        Lt(symbols::Lt<'tree>),
        LtLt(symbols::LtLt<'tree>),
        LtEq(symbols::LtEq<'tree>),
        EqEq(symbols::EqEq<'tree>),
        Gt(symbols::Gt<'tree>),
        GtEq(symbols::GtEq<'tree>),
        GtGt(symbols::GtGt<'tree>),
        BitXor(symbols::BitXor<'tree>),
        And_(unnamed::And<'tree>),
        As(unnamed::As<'tree>),
        In(unnamed::In<'tree>),
        Is(unnamed::Is<'tree>),
        Not(unnamed::Not<'tree>),
        Or(unnamed::Or<'tree>),
        Or_(symbols::Or<'tree>),
        OrOr(symbols::OrOr<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon19710622642415448518402396540615098358<'tree> {
        #[doc = "Returns the node if it is of type `!=` ([`symbols::NotEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_not_eq(self) -> ::std::option::Option<symbols::NotEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::NotEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `%` ([`symbols::Mod`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mod(self) -> ::std::option::Option<symbols::Mod<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mod(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `&` ([`symbols::And`]), otherwise returns `None`"]
        #[inline]
        pub fn as_and(self) -> ::std::option::Option<symbols::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `&&` ([`symbols::AndAnd`]), otherwise returns `None`"]
        #[inline]
        pub fn as_and_and(self) -> ::std::option::Option<symbols::AndAnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AndAnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `*` ([`symbols::Mul`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mul(self) -> ::std::option::Option<symbols::Mul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Mul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `**` ([`symbols::MulMul`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mul_mul(self) -> ::std::option::Option<symbols::MulMul<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulMul(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `+` ([`symbols::Add`]), otherwise returns `None`"]
        #[inline]
        pub fn as_add(self) -> ::std::option::Option<symbols::Add<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Add(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `-` ([`symbols::Sub`]), otherwise returns `None`"]
        #[inline]
        pub fn as_sub(self) -> ::std::option::Option<symbols::Sub<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Sub(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `/` ([`symbols::Div`]), otherwise returns `None`"]
        #[inline]
        pub fn as_div(self) -> ::std::option::Option<symbols::Div<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Div(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `<` ([`symbols::Lt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lt(self) -> ::std::option::Option<symbols::Lt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `<<` ([`symbols::LtLt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lt_lt(self) -> ::std::option::Option<symbols::LtLt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtLt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `<=` ([`symbols::LtEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lt_eq(self) -> ::std::option::Option<symbols::LtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `==` ([`symbols::EqEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_eq_eq(self) -> ::std::option::Option<symbols::EqEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EqEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `>` ([`symbols::Gt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gt(self) -> ::std::option::Option<symbols::Gt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Gt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `>=` ([`symbols::GtEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gt_eq(self) -> ::std::option::Option<symbols::GtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `>>` ([`symbols::GtGt`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gt_gt(self) -> ::std::option::Option<symbols::GtGt<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtGt(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `^` ([`symbols::BitXor`]), otherwise returns `None`"]
        #[inline]
        pub fn as_bit_xor(self) -> ::std::option::Option<symbols::BitXor<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitXor(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `and` ([`unnamed::And`]), otherwise returns `None`"]
        #[inline]
        pub fn as_and_(self) -> ::std::option::Option<unnamed::And<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::And_(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `as` ([`unnamed::As`]), otherwise returns `None`"]
        #[inline]
        pub fn as_as(self) -> ::std::option::Option<unnamed::As<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::As(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `in` ([`unnamed::In`]), otherwise returns `None`"]
        #[inline]
        pub fn as_in(self) -> ::std::option::Option<unnamed::In<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::In(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `is` ([`unnamed::Is`]), otherwise returns `None`"]
        #[inline]
        pub fn as_is(self) -> ::std::option::Option<unnamed::Is<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Is(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `not` ([`unnamed::Not`]), otherwise returns `None`"]
        #[inline]
        pub fn as_not_(self) -> ::std::option::Option<unnamed::Not<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Not(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `or` ([`unnamed::Or`]), otherwise returns `None`"]
        #[inline]
        pub fn as_or(self) -> ::std::option::Option<unnamed::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `|` ([`symbols::Or`]), otherwise returns `None`"]
        #[inline]
        pub fn as_or_(self) -> ::std::option::Option<symbols::Or<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Or_(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `||` ([`symbols::OrOr`]), otherwise returns `None`"]
        #[inline]
        pub fn as_or_or(self) -> ::std::option::Option<symbols::OrOr<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrOr(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Anon19710622642415448518402396540615098358<'tree> {
        type WithLifetime<'a> = Anon19710622642415448518402396540615098358<'a>;

        const KIND: &'static str = "{!= | % | & | && | * | ** | + | - | / | < | << | <= | == | > | >= | >> | ^ | and | as | in | is | not | or | | | ||}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "!=" => {
                    Ok(unsafe {
                        Self::NotEq(<symbols::NotEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "%" => Ok(unsafe {
                    Self::Mod(
                        <symbols::Mod<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "&" => Ok(unsafe {
                    Self::And(
                        <symbols::And<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "&&" => {
                    Ok(unsafe {
                        Self::AndAnd(<symbols::AndAnd<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "*" => Ok(unsafe {
                    Self::Mul(
                        <symbols::Mul<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "**" => {
                    Ok(unsafe {
                        Self::MulMul(<symbols::MulMul<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "+" => Ok(unsafe {
                    Self::Add(
                        <symbols::Add<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "-" => Ok(unsafe {
                    Self::Sub(
                        <symbols::Sub<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "/" => Ok(unsafe {
                    Self::Div(
                        <symbols::Div<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "<" => Ok(unsafe {
                    Self::Lt(
                        <symbols::Lt<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "<<" => {
                    Ok(unsafe {
                        Self::LtLt(<symbols::LtLt<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "<=" => {
                    Ok(unsafe {
                        Self::LtEq(<symbols::LtEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "==" => {
                    Ok(unsafe {
                        Self::EqEq(<symbols::EqEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                ">" => Ok(unsafe {
                    Self::Gt(
                        <symbols::Gt<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                ">=" => {
                    Ok(unsafe {
                        Self::GtEq(<symbols::GtEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                ">>" => {
                    Ok(unsafe {
                        Self::GtGt(<symbols::GtGt<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "^" => {
                    Ok(unsafe {
                        Self::BitXor(<symbols::BitXor<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "and" => Ok(unsafe {
                    Self::And_(
                        <unnamed::And<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "as" => Ok(unsafe {
                    Self::As(
                        <unnamed::As<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "in" => Ok(unsafe {
                    Self::In(
                        <unnamed::In<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "is" => Ok(unsafe {
                    Self::Is(
                        <unnamed::Is<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "not" => Ok(unsafe {
                    Self::Not(
                        <unnamed::Not<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "or" => Ok(unsafe {
                    Self::Or(
                        <unnamed::Or<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "|" => Ok(unsafe {
                    Self::Or_(
                        <symbols::Or<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "||" => {
                    Ok(unsafe {
                        Self::OrOr(<symbols::OrOr<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw(x),
                Self::Mod(x) => ::type_sitter::Node::raw(x),
                Self::And(x) => ::type_sitter::Node::raw(x),
                Self::AndAnd(x) => ::type_sitter::Node::raw(x),
                Self::Mul(x) => ::type_sitter::Node::raw(x),
                Self::MulMul(x) => ::type_sitter::Node::raw(x),
                Self::Add(x) => ::type_sitter::Node::raw(x),
                Self::Sub(x) => ::type_sitter::Node::raw(x),
                Self::Div(x) => ::type_sitter::Node::raw(x),
                Self::Lt(x) => ::type_sitter::Node::raw(x),
                Self::LtLt(x) => ::type_sitter::Node::raw(x),
                Self::LtEq(x) => ::type_sitter::Node::raw(x),
                Self::EqEq(x) => ::type_sitter::Node::raw(x),
                Self::Gt(x) => ::type_sitter::Node::raw(x),
                Self::GtEq(x) => ::type_sitter::Node::raw(x),
                Self::GtGt(x) => ::type_sitter::Node::raw(x),
                Self::BitXor(x) => ::type_sitter::Node::raw(x),
                Self::And_(x) => ::type_sitter::Node::raw(x),
                Self::As(x) => ::type_sitter::Node::raw(x),
                Self::In(x) => ::type_sitter::Node::raw(x),
                Self::Is(x) => ::type_sitter::Node::raw(x),
                Self::Not(x) => ::type_sitter::Node::raw(x),
                Self::Or(x) => ::type_sitter::Node::raw(x),
                Self::Or_(x) => ::type_sitter::Node::raw(x),
                Self::OrOr(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mod(x) => ::type_sitter::Node::raw_mut(x),
                Self::And(x) => ::type_sitter::Node::raw_mut(x),
                Self::AndAnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::Mul(x) => ::type_sitter::Node::raw_mut(x),
                Self::MulMul(x) => ::type_sitter::Node::raw_mut(x),
                Self::Add(x) => ::type_sitter::Node::raw_mut(x),
                Self::Sub(x) => ::type_sitter::Node::raw_mut(x),
                Self::Div(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtLt(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::EqEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::Gt(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtGt(x) => ::type_sitter::Node::raw_mut(x),
                Self::BitXor(x) => ::type_sitter::Node::raw_mut(x),
                Self::And_(x) => ::type_sitter::Node::raw_mut(x),
                Self::As(x) => ::type_sitter::Node::raw_mut(x),
                Self::In(x) => ::type_sitter::Node::raw_mut(x),
                Self::Is(x) => ::type_sitter::Node::raw_mut(x),
                Self::Not(x) => ::type_sitter::Node::raw_mut(x),
                Self::Or(x) => ::type_sitter::Node::raw_mut(x),
                Self::Or_(x) => ::type_sitter::Node::raw_mut(x),
                Self::OrOr(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::NotEq(x) => x.into_raw(),
                Self::Mod(x) => x.into_raw(),
                Self::And(x) => x.into_raw(),
                Self::AndAnd(x) => x.into_raw(),
                Self::Mul(x) => x.into_raw(),
                Self::MulMul(x) => x.into_raw(),
                Self::Add(x) => x.into_raw(),
                Self::Sub(x) => x.into_raw(),
                Self::Div(x) => x.into_raw(),
                Self::Lt(x) => x.into_raw(),
                Self::LtLt(x) => x.into_raw(),
                Self::LtEq(x) => x.into_raw(),
                Self::EqEq(x) => x.into_raw(),
                Self::Gt(x) => x.into_raw(),
                Self::GtEq(x) => x.into_raw(),
                Self::GtGt(x) => x.into_raw(),
                Self::BitXor(x) => x.into_raw(),
                Self::And_(x) => x.into_raw(),
                Self::As(x) => x.into_raw(),
                Self::In(x) => x.into_raw(),
                Self::Is(x) => x.into_raw(),
                Self::Not(x) => x.into_raw(),
                Self::Or(x) => x.into_raw(),
                Self::Or_(x) => x.into_raw(),
                Self::OrOr(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_compound_statement | annotation | break_statement | breakpoint_statement | class_name_statement | const_statement | continue_statement | export_variable_statement | expression_statement | extends_statement | onready_variable_statement | pass_statement | region_end | region_start | return_statement | signal_statement | variable_statement}`:\n- [`CompoundStatement`]\n- [`Annotation`]\n- [`BreakStatement`]\n- [`BreakpointStatement`]\n- [`ClassNameStatement`]\n- [`ConstStatement`]\n- [`ContinueStatement`]\n- [`ExportVariableStatement`]\n- [`ExpressionStatement`]\n- [`ExtendsStatement`]\n- [`OnreadyVariableStatement`]\n- [`PassStatement`]\n- [`RegionEnd`]\n- [`RegionStart`]\n- [`ReturnStatement`]\n- [`SignalStatement`]\n- [`VariableStatement`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Anon266246878783060495398820561106992441731<'tree> {
        CompoundStatement(CompoundStatement<'tree>),
        Annotation(Annotation<'tree>),
        BreakStatement(BreakStatement<'tree>),
        BreakpointStatement(BreakpointStatement<'tree>),
        ClassNameStatement(ClassNameStatement<'tree>),
        ConstStatement(ConstStatement<'tree>),
        ContinueStatement(ContinueStatement<'tree>),
        ExportVariableStatement(ExportVariableStatement<'tree>),
        ExpressionStatement(ExpressionStatement<'tree>),
        ExtendsStatement(ExtendsStatement<'tree>),
        OnreadyVariableStatement(OnreadyVariableStatement<'tree>),
        PassStatement(PassStatement<'tree>),
        RegionEnd(RegionEnd<'tree>),
        RegionStart(RegionStart<'tree>),
        ReturnStatement(ReturnStatement<'tree>),
        SignalStatement(SignalStatement<'tree>),
        VariableStatement(VariableStatement<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Anon266246878783060495398820561106992441731<'tree> {
        #[doc = "Returns the node if it is of type `_compound_statement` ([`CompoundStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_compound_statement(self) -> ::std::option::Option<CompoundStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::CompoundStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `annotation` ([`Annotation`]), otherwise returns `None`"]
        #[inline]
        pub fn as_annotation(self) -> ::std::option::Option<Annotation<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Annotation(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `break_statement` ([`BreakStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_break_statement(self) -> ::std::option::Option<BreakStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BreakStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `breakpoint_statement` ([`BreakpointStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_breakpoint_statement(self) -> ::std::option::Option<BreakpointStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BreakpointStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `class_name_statement` ([`ClassNameStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_class_name_statement(self) -> ::std::option::Option<ClassNameStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassNameStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `const_statement` ([`ConstStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_const_statement(self) -> ::std::option::Option<ConstStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ConstStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `continue_statement` ([`ContinueStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_continue_statement(self) -> ::std::option::Option<ContinueStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ContinueStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `export_variable_statement` ([`ExportVariableStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_export_variable_statement(
            self,
        ) -> ::std::option::Option<ExportVariableStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExportVariableStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `expression_statement` ([`ExpressionStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression_statement(self) -> ::std::option::Option<ExpressionStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExpressionStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `extends_statement` ([`ExtendsStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_extends_statement(self) -> ::std::option::Option<ExtendsStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ExtendsStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `onready_variable_statement` ([`OnreadyVariableStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_onready_variable_statement(
            self,
        ) -> ::std::option::Option<OnreadyVariableStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OnreadyVariableStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pass_statement` ([`PassStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pass_statement(self) -> ::std::option::Option<PassStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PassStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `region_end` ([`RegionEnd`]), otherwise returns `None`"]
        #[inline]
        pub fn as_region_end(self) -> ::std::option::Option<RegionEnd<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RegionEnd(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `region_start` ([`RegionStart`]), otherwise returns `None`"]
        #[inline]
        pub fn as_region_start(self) -> ::std::option::Option<RegionStart<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::RegionStart(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `return_statement` ([`ReturnStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_return_statement(self) -> ::std::option::Option<ReturnStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ReturnStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `signal_statement` ([`SignalStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_signal_statement(self) -> ::std::option::Option<SignalStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SignalStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `variable_statement` ([`VariableStatement`]), otherwise returns `None`"]
        #[inline]
        pub fn as_variable_statement(self) -> ::std::option::Option<VariableStatement<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::VariableStatement(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `class_definition` ([`ClassDefinition`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_class_definition(self) -> ::std::option::Option<ClassDefinition<'tree>> {
            self.as_compound_statement()?.as_class_definition()
        }

        #[doc = "Returns the node if it is of type `constructor_definition` ([`ConstructorDefinition`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_constructor_definition(
            self,
        ) -> ::std::option::Option<ConstructorDefinition<'tree>> {
            self.as_compound_statement()?.as_constructor_definition()
        }

        #[doc = "Returns the node if it is of type `enum_definition` ([`EnumDefinition`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_enum_definition(self) -> ::std::option::Option<EnumDefinition<'tree>> {
            self.as_compound_statement()?.as_enum_definition()
        }

        #[doc = "Returns the node if it is of type `for_statement` ([`ForStatement`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_for_statement(self) -> ::std::option::Option<ForStatement<'tree>> {
            self.as_compound_statement()?.as_for_statement()
        }

        #[doc = "Returns the node if it is of type `function_definition` ([`FunctionDefinition`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_function_definition(self) -> ::std::option::Option<FunctionDefinition<'tree>> {
            self.as_compound_statement()?.as_function_definition()
        }

        #[doc = "Returns the node if it is of type `if_statement` ([`IfStatement`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_if_statement(self) -> ::std::option::Option<IfStatement<'tree>> {
            self.as_compound_statement()?.as_if_statement()
        }

        #[doc = "Returns the node if it is of type `match_statement` ([`MatchStatement`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_match_statement(self) -> ::std::option::Option<MatchStatement<'tree>> {
            self.as_compound_statement()?.as_match_statement()
        }

        #[doc = "Returns the node if it is of type `while_statement` ([`WhileStatement`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_compound_statement` ([`CompoundStatement < 'tree >`], from [`as_compound_statement`](Self::as_compound_statement))"]
        #[inline]
        pub fn as_while_statement(self) -> ::std::option::Option<WhileStatement<'tree>> {
            self.as_compound_statement()?.as_while_statement()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Anon266246878783060495398820561106992441731<'tree> {
        type WithLifetime<'a> = Anon266246878783060495398820561106992441731<'a>;

        const KIND: &'static str = "{_compound_statement | annotation | break_statement | breakpoint_statement | class_name_statement | const_statement | continue_statement | export_variable_statement | expression_statement | extends_statement | onready_variable_statement | pass_statement | region_end | region_start | return_statement | signal_statement | variable_statement}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) =
                <CompoundStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::CompoundStatement(this));
            }
            if let Ok(this) = <Annotation<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Annotation(this));
            }
            if let Ok(this) =
                <BreakStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::BreakStatement(this));
            }
            if let Ok(this) =
                <BreakpointStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::BreakpointStatement(this));
            }
            if let Ok(this) =
                <ClassNameStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ClassNameStatement(this));
            }
            if let Ok(this) =
                <ConstStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ConstStatement(this));
            }
            if let Ok(this) =
                <ContinueStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ContinueStatement(this));
            }
            if let Ok(this) =
                <ExportVariableStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ExportVariableStatement(this));
            }
            if let Ok(this) =
                <ExpressionStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ExpressionStatement(this));
            }
            if let Ok(this) =
                <ExtendsStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ExtendsStatement(this));
            }
            if let Ok(this) =
                <OnreadyVariableStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::OnreadyVariableStatement(this));
            }
            if let Ok(this) =
                <PassStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PassStatement(this));
            }
            if let Ok(this) = <RegionEnd<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::RegionEnd(this));
            }
            if let Ok(this) = <RegionStart<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::RegionStart(this));
            }
            if let Ok(this) =
                <ReturnStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::ReturnStatement(this));
            }
            if let Ok(this) =
                <SignalStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::SignalStatement(this));
            }
            if let Ok(this) =
                <VariableStatement<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::VariableStatement(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => ::type_sitter::Node::raw(x),
                Self::Annotation(x) => ::type_sitter::Node::raw(x),
                Self::BreakStatement(x) => ::type_sitter::Node::raw(x),
                Self::BreakpointStatement(x) => ::type_sitter::Node::raw(x),
                Self::ClassNameStatement(x) => ::type_sitter::Node::raw(x),
                Self::ConstStatement(x) => ::type_sitter::Node::raw(x),
                Self::ContinueStatement(x) => ::type_sitter::Node::raw(x),
                Self::ExportVariableStatement(x) => ::type_sitter::Node::raw(x),
                Self::ExpressionStatement(x) => ::type_sitter::Node::raw(x),
                Self::ExtendsStatement(x) => ::type_sitter::Node::raw(x),
                Self::OnreadyVariableStatement(x) => ::type_sitter::Node::raw(x),
                Self::PassStatement(x) => ::type_sitter::Node::raw(x),
                Self::RegionEnd(x) => ::type_sitter::Node::raw(x),
                Self::RegionStart(x) => ::type_sitter::Node::raw(x),
                Self::ReturnStatement(x) => ::type_sitter::Node::raw(x),
                Self::SignalStatement(x) => ::type_sitter::Node::raw(x),
                Self::VariableStatement(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::Annotation(x) => ::type_sitter::Node::raw_mut(x),
                Self::BreakStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::BreakpointStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ClassNameStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ConstStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ContinueStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ExportVariableStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ExpressionStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::ExtendsStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::OnreadyVariableStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::PassStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::RegionEnd(x) => ::type_sitter::Node::raw_mut(x),
                Self::RegionStart(x) => ::type_sitter::Node::raw_mut(x),
                Self::ReturnStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::SignalStatement(x) => ::type_sitter::Node::raw_mut(x),
                Self::VariableStatement(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::CompoundStatement(x) => x.into_raw(),
                Self::Annotation(x) => x.into_raw(),
                Self::BreakStatement(x) => x.into_raw(),
                Self::BreakpointStatement(x) => x.into_raw(),
                Self::ClassNameStatement(x) => x.into_raw(),
                Self::ConstStatement(x) => x.into_raw(),
                Self::ContinueStatement(x) => x.into_raw(),
                Self::ExportVariableStatement(x) => x.into_raw(),
                Self::ExpressionStatement(x) => x.into_raw(),
                Self::ExtendsStatement(x) => x.into_raw(),
                Self::OnreadyVariableStatement(x) => x.into_raw(),
                Self::PassStatement(x) => x.into_raw(),
                Self::RegionEnd(x) => x.into_raw(),
                Self::RegionStart(x) => x.into_raw(),
                Self::ReturnStatement(x) => x.into_raw(),
                Self::SignalStatement(x) => x.into_raw(),
                Self::VariableStatement(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_attribute_expression | attribute_call | attribute_subscript}`:\n- [`AttributeExpression`]\n- [`AttributeCall`]\n- [`AttributeSubscript`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AttributeExpression_AttributeCall_AttributeSubscript<'tree> {
        AttributeExpression(AttributeExpression<'tree>),
        AttributeCall(AttributeCall<'tree>),
        AttributeSubscript(AttributeSubscript<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> AttributeExpression_AttributeCall_AttributeSubscript<'tree> {
        #[doc = "Returns the node if it is of type `_attribute_expression` ([`AttributeExpression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute_expression(self) -> ::std::option::Option<AttributeExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `attribute_call` ([`AttributeCall`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute_call(self) -> ::std::option::Option<AttributeCall<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeCall(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `attribute_subscript` ([`AttributeSubscript`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute_subscript(self) -> ::std::option::Option<AttributeSubscript<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AttributeSubscript(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_attribute_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_attribute_expression()?.as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_attribute_expression()?.as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_attribute_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_attribute_expression()?.as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_attribute_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_attribute_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_attribute_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_attribute_expression()?.as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_attribute_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_attribute_expression()?.as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_attribute_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_attribute_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_attribute_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_attribute_expression()?.as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_attribute_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_attribute_expression` ([`AttributeExpression < 'tree >`], from [`as_attribute_expression`](Self::as_attribute_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_attribute_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for AttributeExpression_AttributeCall_AttributeSubscript<'tree>
    {
        type WithLifetime<'a> = AttributeExpression_AttributeCall_AttributeSubscript<'a>;

        const KIND: &'static str = "{_attribute_expression | attribute_call | attribute_subscript}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) =
                <AttributeExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::AttributeExpression(this));
            }
            if let Ok(this) =
                <AttributeCall<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::AttributeCall(this));
            }
            if let Ok(this) =
                <AttributeSubscript<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::AttributeSubscript(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeExpression(x) => ::type_sitter::Node::raw(x),
                Self::AttributeCall(x) => ::type_sitter::Node::raw(x),
                Self::AttributeSubscript(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::AttributeCall(x) => ::type_sitter::Node::raw_mut(x),
                Self::AttributeSubscript(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::AttributeExpression(x) => x.into_raw(),
                Self::AttributeCall(x) => x.into_raw(),
                Self::AttributeSubscript(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{attribute | binary_operator | call | identifier | integer | parenthesized_expression | subscript | unary_operator}`:\n- [`Attribute`]\n- [`BinaryOperator`]\n- [`Call`]\n- [`Identifier`]\n- [`Integer`]\n- [`ParenthesizedExpression`]\n- [`Subscript`]\n- [`UnaryOperator`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<
        'tree,
    > {
        Attribute(Attribute<'tree>),
        BinaryOperator(BinaryOperator<'tree>),
        Call(Call<'tree>),
        Identifier(Identifier<'tree>),
        Integer(Integer<'tree>),
        ParenthesizedExpression(ParenthesizedExpression<'tree>),
        Subscript(Subscript<'tree>),
        UnaryOperator(UnaryOperator<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl <'tree>Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<'tree>{
        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute(self) ->  ::std::option::Option<Attribute<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`"]
        #[inline]
        pub fn as_binary_operator(self) ->  ::std::option::Option<BinaryOperator<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::BinaryOperator(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`"]
        #[inline]
        pub fn as_call(self) ->  ::std::option::Option<Call<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::Call(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) ->  ::std::option::Option<Identifier<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`"]
        #[inline]
        pub fn as_integer(self) ->  ::std::option::Option<Integer<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::Integer(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_parenthesized_expression(self) ->  ::std::option::Option<ParenthesizedExpression<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::ParenthesizedExpression(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`"]
        #[inline]
        pub fn as_subscript(self) ->  ::std::option::Option<Subscript<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::Subscript(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }
        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`"]
        #[inline]
        pub fn as_unary_operator(self) ->  ::std::option::Option<UnaryOperator<'tree> >{
            #[allow(irrefutable_let_patterns)]
            if let Self::UnaryOperator(x) = self {
                ::std::option::Option::Some(x)
            }else {
                ::std::option::Option::None
            }
        }

        }
    #[automatically_derived]
    impl <'tree> ::type_sitter::Node<'tree>for Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<'tree>{
        type WithLifetime<'a>  = Attribute_BinaryOperator_Call_Identifier_Integer_ParenthesizedExpression_Subscript_UnaryOperator<'a> ;
        const KIND: &'static str = "{attribute | binary_operator | call | identifier | integer | parenthesized_expression | subscript | unary_operator}";
        #[inline]
        fn try_from_raw(node: ::type_sitter::raw::Node<'tree>) ->  ::type_sitter::NodeResult<'tree,Self>{
            match node.kind(){
                "attribute" => Ok(unsafe {
                    Self::Attribute(<Attribute<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "binary_operator" => Ok(unsafe {
                    Self::BinaryOperator(<BinaryOperator<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "call" => Ok(unsafe {
                    Self::Call(<Call<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "identifier" => Ok(unsafe {
                    Self::Identifier(<Identifier<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "integer" => Ok(unsafe {
                    Self::Integer(<Integer<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "parenthesized_expression" => Ok(unsafe {
                    Self::ParenthesizedExpression(<ParenthesizedExpression<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "subscript" => Ok(unsafe {
                    Self::Subscript(<Subscript<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "unary_operator" => Ok(unsafe {
                    Self::UnaryOperator(<UnaryOperator<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                _ => Err(::type_sitter::IncorrectKind::new:: <Self>(node))

                }
        }
        #[inline]
        fn raw(&self) ->  & ::type_sitter::raw::Node<'tree>{
            match self {
                Self::Attribute(x) =>  ::type_sitter::Node::raw(x),
                Self::BinaryOperator(x) =>  ::type_sitter::Node::raw(x),
                Self::Call(x) =>  ::type_sitter::Node::raw(x),
                Self::Identifier(x) =>  ::type_sitter::Node::raw(x),
                Self::Integer(x) =>  ::type_sitter::Node::raw(x),
                Self::ParenthesizedExpression(x) =>  ::type_sitter::Node::raw(x),
                Self::Subscript(x) =>  ::type_sitter::Node::raw(x),
                Self::UnaryOperator(x) =>  ::type_sitter::Node::raw(x),

                }
        }
        #[inline]
        fn raw_mut(&mut self) ->  &mut ::type_sitter::raw::Node<'tree>{
            match self {
                Self::Attribute(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::BinaryOperator(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::Call(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::Integer(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::ParenthesizedExpression(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::Subscript(x) =>  ::type_sitter::Node::raw_mut(x),
                Self::UnaryOperator(x) =>  ::type_sitter::Node::raw_mut(x),

                }
        }
        #[inline]
        fn into_raw(self) ->  ::type_sitter::raw::Node<'tree>{
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::BinaryOperator(x) => x.into_raw(),
                Self::Call(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Integer(x) => x.into_raw(),
                Self::ParenthesizedExpression(x) => x.into_raw(),
                Self::Subscript(x) => x.into_raw(),
                Self::UnaryOperator(x) => x.into_raw(),

                }
        }

        }
    #[doc = "One of `{attribute | identifier | subscript}`:\n- [`Attribute`]\n- [`Identifier`]\n- [`Subscript`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute_Identifier_Subscript<'tree> {
        Attribute(Attribute<'tree>),
        Identifier(Identifier<'tree>),
        Subscript(Subscript<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Attribute_Identifier_Subscript<'tree> {
        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Attribute(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Subscript(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Attribute_Identifier_Subscript<'tree> {
        type WithLifetime<'a> = Attribute_Identifier_Subscript<'a>;

        const KIND: &'static str = "{attribute | identifier | subscript}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "attribute" => Ok(unsafe {
                    Self::Attribute(
                        <Attribute<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "identifier" => Ok(unsafe {
                    Self::Identifier(
                        <Identifier<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "subscript" => Ok(unsafe {
                    Self::Subscript(
                        <Subscript<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::Subscript(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Subscript(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Attribute(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Subscript(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{body | class_body | enumerator_list | match_body}`:\n- [`Body`]\n- [`ClassBody`]\n- [`EnumeratorList`]\n- [`MatchBody`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Body_ClassBody_EnumeratorList_MatchBody<'tree> {
        Body(Body<'tree>),
        ClassBody(ClassBody<'tree>),
        EnumeratorList(EnumeratorList<'tree>),
        MatchBody(MatchBody<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Body_ClassBody_EnumeratorList_MatchBody<'tree> {
        #[doc = "Returns the node if it is of type `body` ([`Body`]), otherwise returns `None`"]
        #[inline]
        pub fn as_body(self) -> ::std::option::Option<Body<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Body(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `class_body` ([`ClassBody`]), otherwise returns `None`"]
        #[inline]
        pub fn as_class_body(self) -> ::std::option::Option<ClassBody<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ClassBody(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `enumerator_list` ([`EnumeratorList`]), otherwise returns `None`"]
        #[inline]
        pub fn as_enumerator_list(self) -> ::std::option::Option<EnumeratorList<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::EnumeratorList(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `match_body` ([`MatchBody`]), otherwise returns `None`"]
        #[inline]
        pub fn as_match_body(self) -> ::std::option::Option<MatchBody<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MatchBody(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Body_ClassBody_EnumeratorList_MatchBody<'tree> {
        type WithLifetime<'a> = Body_ClassBody_EnumeratorList_MatchBody<'a>;

        const KIND: &'static str = "{body | class_body | enumerator_list | match_body}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "body" => Ok(unsafe {
                    Self::Body(
                        <Body<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "class_body" => Ok(unsafe {
                    Self::ClassBody(
                        <ClassBody<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "enumerator_list" => {
                    Ok(unsafe {
                        Self::EnumeratorList(<EnumeratorList<'tree> as ::type_sitter::Node<
                            'tree,
                        >>::from_raw_unchecked(node))
                    })
                }
                "match_body" => Ok(unsafe {
                    Self::MatchBody(
                        <MatchBody<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Body(x) => ::type_sitter::Node::raw(x),
                Self::ClassBody(x) => ::type_sitter::Node::raw(x),
                Self::EnumeratorList(x) => ::type_sitter::Node::raw(x),
                Self::MatchBody(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Body(x) => ::type_sitter::Node::raw_mut(x),
                Self::ClassBody(x) => ::type_sitter::Node::raw_mut(x),
                Self::EnumeratorList(x) => ::type_sitter::Node::raw_mut(x),
                Self::MatchBody(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Body(x) => x.into_raw(),
                Self::ClassBody(x) => x.into_raw(),
                Self::EnumeratorList(x) => x.into_raw(),
                Self::MatchBody(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{elif_clause | else_clause}`:\n- [`ElifClause`]\n- [`ElseClause`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ElifClause_ElseClause<'tree> {
        ElifClause(ElifClause<'tree>),
        ElseClause(ElseClause<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ElifClause_ElseClause<'tree> {
        #[doc = "Returns the node if it is of type `elif_clause` ([`ElifClause`]), otherwise returns `None`"]
        #[inline]
        pub fn as_elif_clause(self) -> ::std::option::Option<ElifClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ElifClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `else_clause` ([`ElseClause`]), otherwise returns `None`"]
        #[inline]
        pub fn as_else_clause(self) -> ::std::option::Option<ElseClause<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ElseClause(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Get the field `body`.\n\nThis child has type `body` ([`Body`])"]
        #[inline]
        pub fn body(&self) -> ::type_sitter::NodeResult<'tree, Body<'tree>> {
            ::type_sitter::Node::raw(self).child_by_field_name("body").map(<Body<'tree>as ::type_sitter::Node<'tree>> ::try_from_raw).expect("required child not present, there should at least be a MISSING node in its place")
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for ElifClause_ElseClause<'tree> {
        type WithLifetime<'a> = ElifClause_ElseClause<'a>;

        const KIND: &'static str = "{elif_clause | else_clause}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "elif_clause" => Ok(unsafe {
                    Self::ElifClause(
                        <ElifClause<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "else_clause" => Ok(unsafe {
                    Self::ElseClause(
                        <ElseClause<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ElifClause(x) => ::type_sitter::Node::raw(x),
                Self::ElseClause(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ElifClause(x) => ::type_sitter::Node::raw_mut(x),
                Self::ElseClause(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ElifClause(x) => x.into_raw(),
                Self::ElseClause(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_expression | assignment | augmented_assignment}`:\n- [`Expression`]\n- [`Assignment`]\n- [`AugmentedAssignment`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Assignment_AugmentedAssignment<'tree> {
        Expression(Expression<'tree>),
        Assignment(Assignment<'tree>),
        AugmentedAssignment(AugmentedAssignment<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Assignment_AugmentedAssignment<'tree> {
        #[doc = "Returns the node if it is of type `_expression` ([`Expression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `assignment` ([`Assignment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_assignment(self) -> ::std::option::Option<Assignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Assignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `augmented_assignment` ([`AugmentedAssignment`]), otherwise returns `None`"]
        #[inline]
        pub fn as_augmented_assignment(self) -> ::std::option::Option<AugmentedAssignment<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AugmentedAssignment(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Expression_Assignment_AugmentedAssignment<'tree> {
        type WithLifetime<'a> = Expression_Assignment_AugmentedAssignment<'a>;

        const KIND: &'static str = "{_expression | assignment | augmented_assignment}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Assignment<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Assignment(this));
            }
            if let Ok(this) =
                <AugmentedAssignment<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::AugmentedAssignment(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Assignment(x) => ::type_sitter::Node::raw(x),
                Self::AugmentedAssignment(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Assignment(x) => ::type_sitter::Node::raw_mut(x),
                Self::AugmentedAssignment(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Assignment(x) => x.into_raw(),
                Self::AugmentedAssignment(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_expression | identifier | lambda}`:\n- [`Expression`]\n- [`Identifier`]\n- [`Lambda`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Identifier_Lambda<'tree> {
        Expression(Expression<'tree>),
        Identifier(Identifier<'tree>),
        Lambda(Lambda<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Identifier_Lambda<'tree> {
        #[doc = "Returns the node if it is of type `_expression` ([`Expression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Identifier(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lambda(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Expression_Identifier_Lambda<'tree> {
        type WithLifetime<'a> = Expression_Identifier_Lambda<'a>;

        const KIND: &'static str = "{_expression | identifier | lambda}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Identifier<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Identifier(this));
            }
            if let Ok(this) = <Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Lambda(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Identifier(x) => ::type_sitter::Node::raw(x),
                Self::Lambda(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Identifier(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lambda(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Identifier(x) => x.into_raw(),
                Self::Lambda(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_expression | lambda}`:\n- [`Expression`]\n- [`Lambda`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Lambda<'tree> {
        Expression(Expression<'tree>),
        Lambda(Lambda<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Lambda<'tree> {
        #[doc = "Returns the node if it is of type `_expression` ([`Expression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lambda(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Expression_Lambda<'tree> {
        type WithLifetime<'a> = Expression_Lambda<'a>;

        const KIND: &'static str = "{_expression | lambda}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Lambda(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Lambda(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lambda(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Lambda(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_expression | lambda | pattern_binding}`:\n- [`Expression`]\n- [`Lambda`]\n- [`PatternBinding`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Lambda_PatternBinding<'tree> {
        Expression(Expression<'tree>),
        Lambda(Lambda<'tree>),
        PatternBinding(PatternBinding<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Lambda_PatternBinding<'tree> {
        #[doc = "Returns the node if it is of type `_expression` ([`Expression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lambda(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_binding` ([`PatternBinding`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_binding(self) -> ::std::option::Option<PatternBinding<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternBinding(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Expression_Lambda_PatternBinding<'tree> {
        type WithLifetime<'a> = Expression_Lambda_PatternBinding<'a>;

        const KIND: &'static str = "{_expression | lambda | pattern_binding}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Lambda(this));
            }
            if let Ok(this) =
                <PatternBinding<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PatternBinding(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Lambda(x) => ::type_sitter::Node::raw(x),
                Self::PatternBinding(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lambda(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternBinding(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Lambda(x) => x.into_raw(),
                Self::PatternBinding(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_expression | lambda | pattern_binding | pattern_open_ending}`:\n- [`Expression`]\n- [`Lambda`]\n- [`PatternBinding`]\n- [`PatternOpenEnding`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Expression_Lambda_PatternBinding_PatternOpenEnding<'tree> {
        Expression(Expression<'tree>),
        Lambda(Lambda<'tree>),
        PatternBinding(PatternBinding<'tree>),
        PatternOpenEnding(PatternOpenEnding<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Expression_Lambda_PatternBinding_PatternOpenEnding<'tree> {
        #[doc = "Returns the node if it is of type `_expression` ([`Expression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_expression(self) -> ::std::option::Option<Expression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Expression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `lambda` ([`Lambda`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lambda(self) -> ::std::option::Option<Lambda<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Lambda(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_binding` ([`PatternBinding`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_binding(self) -> ::std::option::Option<PatternBinding<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternBinding(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_open_ending` ([`PatternOpenEnding`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_open_ending(self) -> ::std::option::Option<PatternOpenEnding<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternOpenEnding(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_expression()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_expression()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_expression()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_expression` ([`Expression < 'tree >`], from [`as_expression`](Self::as_expression))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Expression < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_expression()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for Expression_Lambda_PatternBinding_PatternOpenEnding<'tree>
    {
        type WithLifetime<'a> = Expression_Lambda_PatternBinding_PatternOpenEnding<'a>;

        const KIND: &'static str = "{_expression | lambda | pattern_binding | pattern_open_ending}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Expression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::Expression(this));
            }
            if let Ok(this) = <Lambda<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Lambda(this));
            }
            if let Ok(this) =
                <PatternBinding<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PatternBinding(this));
            }
            if let Ok(this) =
                <PatternOpenEnding<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PatternOpenEnding(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw(x),
                Self::Lambda(x) => ::type_sitter::Node::raw(x),
                Self::PatternBinding(x) => ::type_sitter::Node::raw(x),
                Self::PatternOpenEnding(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Lambda(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternBinding(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternOpenEnding(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Expression(x) => x.into_raw(),
                Self::Lambda(x) => x.into_raw(),
                Self::PatternBinding(x) => x.into_raw(),
                Self::PatternOpenEnding(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{get_body | getter}`:\n- [`GetBody`]\n- [`Getter`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum GetBody_Getter<'tree> {
        GetBody(GetBody<'tree>),
        Getter(Getter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> GetBody_Getter<'tree> {
        #[doc = "Returns the node if it is of type `get_body` ([`GetBody`]), otherwise returns `None`"]
        #[inline]
        pub fn as_get_body(self) -> ::std::option::Option<GetBody<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GetBody(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `getter` ([`Getter`]), otherwise returns `None`"]
        #[inline]
        pub fn as_getter(self) -> ::std::option::Option<Getter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Getter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for GetBody_Getter<'tree> {
        type WithLifetime<'a> = GetBody_Getter<'a>;

        const KIND: &'static str = "{get_body | getter}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "get_body" => Ok(unsafe {
                    Self::GetBody(
                        <GetBody<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "getter" => Ok(unsafe {
                    Self::Getter(
                        <Getter<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::GetBody(x) => ::type_sitter::Node::raw(x),
                Self::Getter(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::GetBody(x) => ::type_sitter::Node::raw_mut(x),
                Self::Getter(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::GetBody(x) => x.into_raw(),
                Self::Getter(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{getter | setter}`:\n- [`Getter`]\n- [`Setter`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Getter_Setter<'tree> {
        Getter(Getter<'tree>),
        Setter(Setter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Getter_Setter<'tree> {
        #[doc = "Returns the node if it is of type `getter` ([`Getter`]), otherwise returns `None`"]
        #[inline]
        pub fn as_getter(self) -> ::std::option::Option<Getter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Getter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `setter` ([`Setter`]), otherwise returns `None`"]
        #[inline]
        pub fn as_setter(self) -> ::std::option::Option<Setter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Setter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Getter_Setter<'tree> {
        type WithLifetime<'a> = Getter_Setter<'a>;

        const KIND: &'static str = "{getter | setter}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "getter" => Ok(unsafe {
                    Self::Getter(
                        <Getter<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "setter" => Ok(unsafe {
                    Self::Setter(
                        <Setter<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Getter(x) => ::type_sitter::Node::raw(x),
                Self::Setter(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Getter(x) => ::type_sitter::Node::raw_mut(x),
                Self::Setter(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Getter(x) => x.into_raw(),
                Self::Setter(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{inferred_type | type}`:\n- [`InferredType`]\n- [`Type`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum InferredType_Type<'tree> {
        InferredType(InferredType<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> InferredType_Type<'tree> {
        #[doc = "Returns the node if it is of type `inferred_type` ([`InferredType`]), otherwise returns `None`"]
        #[inline]
        pub fn as_inferred_type(self) -> ::std::option::Option<InferredType<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::InferredType(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `type` ([`Type`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type(self) -> ::std::option::Option<Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for InferredType_Type<'tree> {
        type WithLifetime<'a> = InferredType_Type<'a>;

        const KIND: &'static str = "{inferred_type | type}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "inferred_type" => Ok(unsafe {
                    Self::InferredType(
                        <InferredType<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(
                            node,
                        ),
                    )
                }),
                "type" => Ok(unsafe {
                    Self::Type(
                        <Type<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::InferredType(x) => ::type_sitter::Node::raw(x),
                Self::Type(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::InferredType(x) => ::type_sitter::Node::raw_mut(x),
                Self::Type(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::InferredType(x) => x.into_raw(),
                Self::Type(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{%= | &= | **= | *= | += | -= | /= | <<= | >>= | ^= | |=}`:\n- [`symbols::ModEq`]\n- [`symbols::AndEq`]\n- [`symbols::MulMulEq`]\n- [`symbols::MulEq`]\n- [`symbols::AddEq`]\n- [`symbols::SubEq`]\n- [`symbols::DivEq`]\n- [`symbols::LtLtEq`]\n- [`symbols::GtGtEq`]\n- [`symbols::BitXorEq`]\n- [`symbols::OrEq`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree> {
        ModEq(symbols::ModEq<'tree>),
        AndEq(symbols::AndEq<'tree>),
        MulMulEq(symbols::MulMulEq<'tree>),
        MulEq(symbols::MulEq<'tree>),
        AddEq(symbols::AddEq<'tree>),
        SubEq(symbols::SubEq<'tree>),
        DivEq(symbols::DivEq<'tree>),
        LtLtEq(symbols::LtLtEq<'tree>),
        GtGtEq(symbols::GtGtEq<'tree>),
        BitXorEq(symbols::BitXorEq<'tree>),
        OrEq(symbols::OrEq<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree> {
        #[doc = "Returns the node if it is of type `%=` ([`symbols::ModEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mod_eq(self) -> ::std::option::Option<symbols::ModEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::ModEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `&=` ([`symbols::AndEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_and_eq(self) -> ::std::option::Option<symbols::AndEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AndEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `**=` ([`symbols::MulMulEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mul_mul_eq(self) -> ::std::option::Option<symbols::MulMulEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulMulEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `*=` ([`symbols::MulEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_mul_eq(self) -> ::std::option::Option<symbols::MulEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::MulEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `+=` ([`symbols::AddEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_add_eq(self) -> ::std::option::Option<symbols::AddEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::AddEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `-=` ([`symbols::SubEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_sub_eq(self) -> ::std::option::Option<symbols::SubEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SubEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `/=` ([`symbols::DivEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_div_eq(self) -> ::std::option::Option<symbols::DivEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::DivEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `<<=` ([`symbols::LtLtEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_lt_lt_eq(self) -> ::std::option::Option<symbols::LtLtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::LtLtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `>>=` ([`symbols::GtGtEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_gt_gt_eq(self) -> ::std::option::Option<symbols::GtGtEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::GtGtEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `^=` ([`symbols::BitXorEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_bit_xor_eq(self) -> ::std::option::Option<symbols::BitXorEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::BitXorEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `|=` ([`symbols::OrEq`]), otherwise returns `None`"]
        #[inline]
        pub fn as_or_eq(self) -> ::std::option::Option<symbols::OrEq<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::OrEq(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree>
        for ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'tree>
    {
        type WithLifetime<'a> =
            ModEq_AndEq_MulMulEq_MulEq_AddEq_SubEq_DivEq_LtLtEq_GtGtEq_BitXorEq_OrEq<'a>;

        const KIND: &'static str = "{%= | &= | **= | *= | += | -= | /= | <<= | >>= | ^= | |=}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "%=" => {
                    Ok(unsafe {
                        Self::ModEq(<symbols::ModEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "&=" => {
                    Ok(unsafe {
                        Self::AndEq(<symbols::AndEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "**=" => Ok(unsafe {
                    Self::MulMulEq(<symbols::MulMulEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "*=" => {
                    Ok(unsafe {
                        Self::MulEq(<symbols::MulEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "+=" => {
                    Ok(unsafe {
                        Self::AddEq(<symbols::AddEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "-=" => {
                    Ok(unsafe {
                        Self::SubEq(<symbols::SubEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "/=" => {
                    Ok(unsafe {
                        Self::DivEq(<symbols::DivEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "<<=" => {
                    Ok(unsafe {
                        Self::LtLtEq(<symbols::LtLtEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                ">>=" => {
                    Ok(unsafe {
                        Self::GtGtEq(<symbols::GtGtEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                "^=" => Ok(unsafe {
                    Self::BitXorEq(<symbols::BitXorEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                }),
                "|=" => {
                    Ok(unsafe {
                        Self::OrEq(<symbols::OrEq<'tree>as ::type_sitter::Node<'tree>> ::from_raw_unchecked(node))
                    })
                }
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::ModEq(x) => ::type_sitter::Node::raw(x),
                Self::AndEq(x) => ::type_sitter::Node::raw(x),
                Self::MulMulEq(x) => ::type_sitter::Node::raw(x),
                Self::MulEq(x) => ::type_sitter::Node::raw(x),
                Self::AddEq(x) => ::type_sitter::Node::raw(x),
                Self::SubEq(x) => ::type_sitter::Node::raw(x),
                Self::DivEq(x) => ::type_sitter::Node::raw(x),
                Self::LtLtEq(x) => ::type_sitter::Node::raw(x),
                Self::GtGtEq(x) => ::type_sitter::Node::raw(x),
                Self::BitXorEq(x) => ::type_sitter::Node::raw(x),
                Self::OrEq(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ModEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::AndEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::MulMulEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::MulEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::AddEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::SubEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::DivEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::LtLtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::GtGtEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::BitXorEq(x) => ::type_sitter::Node::raw_mut(x),
                Self::OrEq(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::ModEq(x) => x.into_raw(),
                Self::AndEq(x) => x.into_raw(),
                Self::MulMulEq(x) => x.into_raw(),
                Self::MulEq(x) => x.into_raw(),
                Self::AddEq(x) => x.into_raw(),
                Self::SubEq(x) => x.into_raw(),
                Self::DivEq(x) => x.into_raw(),
                Self::LtLtEq(x) => x.into_raw(),
                Self::GtGtEq(x) => x.into_raw(),
                Self::BitXorEq(x) => x.into_raw(),
                Self::OrEq(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_pattern | pattern_guard}`:\n- [`Pattern`]\n- [`PatternGuard`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Pattern_PatternGuard<'tree> {
        Pattern(Pattern<'tree>),
        PatternGuard(PatternGuard<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> Pattern_PatternGuard<'tree> {
        #[doc = "Returns the node if it is of type `_pattern` ([`Pattern`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern(self) -> ::std::option::Option<Pattern<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pattern(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_guard` ([`PatternGuard`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_guard(self) -> ::std::option::Option<PatternGuard<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternGuard(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            self.as_pattern()?.as_primary_expression()
        }

        #[doc = "Returns the node if it is of type `conditional_expression` ([`ConditionalExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))"]
        #[inline]
        pub fn as_conditional_expression(
            self,
        ) -> ::std::option::Option<ConditionalExpression<'tree>> {
            self.as_pattern()?.as_conditional_expression()
        }

        #[doc = "Returns the node if it is of type `pattern_binding` ([`PatternBinding`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))"]
        #[inline]
        pub fn as_pattern_binding(self) -> ::std::option::Option<PatternBinding<'tree>> {
            self.as_pattern()?.as_pattern_binding()
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_pattern()?
                .as_primary_expression()?
                .as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_pattern()?
                .as_primary_expression()?
                .as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_pattern()?
                .as_primary_expression()?
                .as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_pattern()?.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_pattern` ([`Pattern < 'tree >`], from [`as_pattern`](Self::as_pattern))\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Pattern < 'tree >::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_pattern()?
                .as_primary_expression()?
                .as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for Pattern_PatternGuard<'tree> {
        type WithLifetime<'a> = Pattern_PatternGuard<'a>;

        const KIND: &'static str = "{_pattern | pattern_guard}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) = <Pattern<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Pattern(this));
            }
            if let Ok(this) =
                <PatternGuard<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PatternGuard(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::Pattern(x) => ::type_sitter::Node::raw(x),
                Self::PatternGuard(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Pattern(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternGuard(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::Pattern(x) => x.into_raw(),
                Self::PatternGuard(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{_primary_expression | pair | pattern_open_ending}`:\n- [`PrimaryExpression`]\n- [`Pair`]\n- [`PatternOpenEnding`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PrimaryExpression_Pair_PatternOpenEnding<'tree> {
        PrimaryExpression(PrimaryExpression<'tree>),
        Pair(Pair<'tree>),
        PatternOpenEnding(PatternOpenEnding<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> PrimaryExpression_Pair_PatternOpenEnding<'tree> {
        #[doc = "Returns the node if it is of type `_primary_expression` ([`PrimaryExpression`]), otherwise returns `None`"]
        #[inline]
        pub fn as_primary_expression(self) -> ::std::option::Option<PrimaryExpression<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PrimaryExpression(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pair` ([`Pair`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pair(self) -> ::std::option::Option<Pair<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Pair(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `pattern_open_ending` ([`PatternOpenEnding`]), otherwise returns `None`"]
        #[inline]
        pub fn as_pattern_open_ending(self) -> ::std::option::Option<PatternOpenEnding<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::PatternOpenEnding(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `array` ([`Array`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_array(self) -> ::std::option::Option<Array<'tree>> {
            self.as_primary_expression()?.as_array()
        }

        #[doc = "Returns the node if it is of type `attribute` ([`Attribute`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_attribute(self) -> ::std::option::Option<Attribute<'tree>> {
            self.as_primary_expression()?.as_attribute()
        }

        #[doc = "Returns the node if it is of type `await_expression` ([`AwaitExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_await_expression(self) -> ::std::option::Option<AwaitExpression<'tree>> {
            self.as_primary_expression()?.as_await_expression()
        }

        #[doc = "Returns the node if it is of type `base_call` ([`BaseCall`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_base_call(self) -> ::std::option::Option<BaseCall<'tree>> {
            self.as_primary_expression()?.as_base_call()
        }

        #[doc = "Returns the node if it is of type `binary_operator` ([`BinaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_binary_operator(self) -> ::std::option::Option<BinaryOperator<'tree>> {
            self.as_primary_expression()?.as_binary_operator()
        }

        #[doc = "Returns the node if it is of type `call` ([`Call`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_call(self) -> ::std::option::Option<Call<'tree>> {
            self.as_primary_expression()?.as_call()
        }

        #[doc = "Returns the node if it is of type `dictionary` ([`Dictionary`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_dictionary(self) -> ::std::option::Option<Dictionary<'tree>> {
            self.as_primary_expression()?.as_dictionary()
        }

        #[doc = "Returns the node if it is of type `false` ([`False`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_false(self) -> ::std::option::Option<False<'tree>> {
            self.as_primary_expression()?.as_false()
        }

        #[doc = "Returns the node if it is of type `float` ([`Float`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_float(self) -> ::std::option::Option<Float<'tree>> {
            self.as_primary_expression()?.as_float()
        }

        #[doc = "Returns the node if it is of type `get_node` ([`GetNode`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_get_node(self) -> ::std::option::Option<GetNode<'tree>> {
            self.as_primary_expression()?.as_get_node()
        }

        #[doc = "Returns the node if it is of type `identifier` ([`Identifier`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_identifier(self) -> ::std::option::Option<Identifier<'tree>> {
            self.as_primary_expression()?.as_identifier()
        }

        #[doc = "Returns the node if it is of type `integer` ([`Integer`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_integer(self) -> ::std::option::Option<Integer<'tree>> {
            self.as_primary_expression()?.as_integer()
        }

        #[doc = "Returns the node if it is of type `node_path` ([`NodePath`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_node_path(self) -> ::std::option::Option<NodePath<'tree>> {
            self.as_primary_expression()?.as_node_path()
        }

        #[doc = "Returns the node if it is of type `null` ([`Null`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_null(self) -> ::std::option::Option<Null<'tree>> {
            self.as_primary_expression()?.as_null()
        }

        #[doc = "Returns the node if it is of type `parenthesized_expression` ([`ParenthesizedExpression`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_parenthesized_expression(
            self,
        ) -> ::std::option::Option<ParenthesizedExpression<'tree>> {
            self.as_primary_expression()?.as_parenthesized_expression()
        }

        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            self.as_primary_expression()?.as_string()
        }

        #[doc = "Returns the node if it is of type `string_name` ([`StringName`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_string_name(self) -> ::std::option::Option<StringName<'tree>> {
            self.as_primary_expression()?.as_string_name()
        }

        #[doc = "Returns the node if it is of type `subscript` ([`Subscript`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_subscript(self) -> ::std::option::Option<Subscript<'tree>> {
            self.as_primary_expression()?.as_subscript()
        }

        #[doc = "Returns the node if it is of type `true` ([`True`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_true(self) -> ::std::option::Option<True<'tree>> {
            self.as_primary_expression()?.as_true()
        }

        #[doc = "Returns the node if it is of type `unary_operator` ([`UnaryOperator`]), otherwise returns `None`.\n\nFollows the following chain:\n- `_primary_expression` ([`PrimaryExpression < 'tree >`], from [`as_primary_expression`](Self::as_primary_expression))"]
        #[inline]
        pub fn as_unary_operator(self) -> ::std::option::Option<UnaryOperator<'tree>> {
            self.as_primary_expression()?.as_unary_operator()
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for PrimaryExpression_Pair_PatternOpenEnding<'tree> {
        type WithLifetime<'a> = PrimaryExpression_Pair_PatternOpenEnding<'a>;

        const KIND: &'static str = "{_primary_expression | pair | pattern_open_ending}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            if let Ok(this) =
                <PrimaryExpression<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PrimaryExpression(this));
            }
            if let Ok(this) = <Pair<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::Pair(this));
            }
            if let Ok(this) =
                <PatternOpenEnding<'tree> as ::type_sitter::Node<'tree>>::try_from_raw(node)
            {
                return Ok(Self::PatternOpenEnding(this));
            }
            Err(::type_sitter::IncorrectKind::new::<Self>(node))
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw(x),
                Self::Pair(x) => ::type_sitter::Node::raw(x),
                Self::PatternOpenEnding(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PrimaryExpression(x) => ::type_sitter::Node::raw_mut(x),
                Self::Pair(x) => ::type_sitter::Node::raw_mut(x),
                Self::PatternOpenEnding(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::PrimaryExpression(x) => x.into_raw(),
                Self::Pair(x) => x.into_raw(),
                Self::PatternOpenEnding(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{set_body | setter}`:\n- [`SetBody`]\n- [`Setter`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum SetBody_Setter<'tree> {
        SetBody(SetBody<'tree>),
        Setter(Setter<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> SetBody_Setter<'tree> {
        #[doc = "Returns the node if it is of type `set_body` ([`SetBody`]), otherwise returns `None`"]
        #[inline]
        pub fn as_set_body(self) -> ::std::option::Option<SetBody<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::SetBody(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `setter` ([`Setter`]), otherwise returns `None`"]
        #[inline]
        pub fn as_setter(self) -> ::std::option::Option<Setter<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Setter(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for SetBody_Setter<'tree> {
        type WithLifetime<'a> = SetBody_Setter<'a>;

        const KIND: &'static str = "{set_body | setter}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "set_body" => Ok(unsafe {
                    Self::SetBody(
                        <SetBody<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "setter" => Ok(unsafe {
                    Self::Setter(
                        <Setter<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::SetBody(x) => ::type_sitter::Node::raw(x),
                Self::Setter(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::SetBody(x) => ::type_sitter::Node::raw_mut(x),
                Self::Setter(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::SetBody(x) => x.into_raw(),
                Self::Setter(x) => x.into_raw(),
            }
        }
    }
    #[doc = "One of `{string | type}`:\n- [`String`]\n- [`Type`]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum String_Type<'tree> {
        String(String<'tree>),
        Type(Type<'tree>),
    }
    #[automatically_derived]
    #[allow(unused)]
    impl<'tree> String_Type<'tree> {
        #[doc = "Returns the node if it is of type `string` ([`String`]), otherwise returns `None`"]
        #[inline]
        pub fn as_string(self) -> ::std::option::Option<String<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::String(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }

        #[doc = "Returns the node if it is of type `type` ([`Type`]), otherwise returns `None`"]
        #[inline]
        pub fn as_type(self) -> ::std::option::Option<Type<'tree>> {
            #[allow(irrefutable_let_patterns)]
            if let Self::Type(x) = self {
                ::std::option::Option::Some(x)
            } else {
                ::std::option::Option::None
            }
        }
    }
    #[automatically_derived]
    impl<'tree> ::type_sitter::Node<'tree> for String_Type<'tree> {
        type WithLifetime<'a> = String_Type<'a>;

        const KIND: &'static str = "{string | type}";

        #[inline]
        fn try_from_raw(
            node: ::type_sitter::raw::Node<'tree>,
        ) -> ::type_sitter::NodeResult<'tree, Self> {
            match node.kind() {
                "string" => Ok(unsafe {
                    Self::String(
                        <String<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                "type" => Ok(unsafe {
                    Self::Type(
                        <Type<'tree> as ::type_sitter::Node<'tree>>::from_raw_unchecked(node),
                    )
                }),
                _ => Err(::type_sitter::IncorrectKind::new::<Self>(node)),
            }
        }

        #[inline]
        fn raw(&self) -> &::type_sitter::raw::Node<'tree> {
            match self {
                Self::String(x) => ::type_sitter::Node::raw(x),
                Self::Type(x) => ::type_sitter::Node::raw(x),
            }
        }

        #[inline]
        fn raw_mut(&mut self) -> &mut ::type_sitter::raw::Node<'tree> {
            match self {
                Self::String(x) => ::type_sitter::Node::raw_mut(x),
                Self::Type(x) => ::type_sitter::Node::raw_mut(x),
            }
        }

        #[inline]
        fn into_raw(self) -> ::type_sitter::raw::Node<'tree> {
            match self {
                Self::String(x) => x.into_raw(),
                Self::Type(x) => x.into_raw(),
            }
        }
    }
}
