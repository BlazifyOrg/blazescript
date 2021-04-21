use crate::core::parser::nodes::Node;
use crate::core::parser::parser_result::ParseResult;
use crate::core::token::Token;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: usize,
    pub current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.clone()[0].clone();
        Parser {
            tokens,
            token_index: 0,
            current_token,
        }
    }

    pub fn advance(&mut self) -> Token {
        self.token_index += 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens.clone()[self.clone().token_index].clone();
        };
        self.current_token.clone()
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.expr();
        self.advance();
        if res.error.is_none() && self.current_token.r#type != Tokens::EOF {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Operators, Variables, Functions, etc but found none",
            ));
        }
        res
    }

    pub fn expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String(String::from("val")))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String(String::from("var")))
        {
            let var_type: String;
            match self.current_token.value.clone() {
                DynType::String(value) => var_type = value,
                _ => panic!(),
            };
            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax Error",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected Identifier",
                ));
            }

            let var_name = self.current_token.clone();
            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::Equals {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '='",
                ));
            }

            res.register_advancement();
            self.advance();

            let expr = res.register(self.expr()).unwrap();
            res.register_advancement();
            self.advance();

            let reassignable = if var_type == String::from("var") {
                true
            } else {
                false
            };
            return res.success(Node::VarAssignNode {
                name: var_name.clone(),
                value: Box::new(expr),
                reassignable,
                pos_start: var_name.pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.comp_expr());
        if res.error.is_some() {
            return res;
        }

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("and".to_string()))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("or".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();
            let right = res.register(self.comp_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'var', int, float, identifier, '+', '-' or '('",
            ));
        }

        res.success(left.unwrap())
    }

    pub fn comp_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("not".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let node = res.register(self.comp_expr());
            if res.error.is_some() {
                return res;
            }

            return res.success(Node::UnaryNode {
                node: Box::new(node.clone().unwrap()),
                op_token: op_token.clone(),
                pos_start: op_token.pos_start,
                pos_end: self.current_token.clone().pos_start,
            });
        }

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.arith_expr());
        if res.error.is_some() {
            return res;
        }

        while [
            Tokens::DoubleEquals,
            Tokens::NotEquals,
            Tokens::LessThan,
            Tokens::LessThanEquals,
            Tokens::GreaterThan,
            Tokens::GreaterThanEquals,
        ]
        .contains(&self.current_token.r#type)
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.arith_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected",
            ));
        }
        res.success(left.unwrap())
    }

    pub fn arith_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.term());
        if res.error.is_some() {
            return res;
        }

        while [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.term());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    pub fn term(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.factor());
        if res.error.is_some() {
            return res;
        }

        while [Tokens::Multiply, Tokens::Divide].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    pub fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            res.register_advancement();
            self.advance();
            let factor = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }
            return res.success(Node::UnaryNode {
                op_token: token.clone(),
                node: Box::new(factor.clone().unwrap()),
                pos_start: token.pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }
        self.power()
    }

    pub fn power(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.call());
        if res.error.is_some() {
            return res;
        }

        while self.current_token.r#type == Tokens::Power {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    pub fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let atom = res.register(self.atom());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.r#type == Tokens::LeftParenthesis {
            let mut arg_nodes: Vec<Node> = vec![];
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                arg_nodes.push(expr.unwrap());

                while self.current_token.r#type == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token.r#type != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
            }
            return res.success(Node::CallNode {
                node_to_call: Box::new(atom.clone().unwrap()),
                args: arg_nodes,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(atom.unwrap())
    }

    pub fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Int, Tokens::Float].contains(&token.r#type) {
            res.register_advancement();
            self.advance();
            return res.success(Node::NumberNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Boolean {
            res.register_advancement();
            self.advance();
            return res.success(Node::BooleanNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::String {
            res.register_advancement();
            self.advance();
            return res.success(Node::StringNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Char {
            res.register_advancement();
            self.advance();
            return res.success(Node::CharNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Identifier {
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::Equals {
                res.register_advancement();
                self.advance();

                let new_value = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                res.register_advancement();
                self.advance();

                return res.success(Node::VarReassignNode {
                    name: token.clone(),
                    value: Box::new(new_value.clone().unwrap()),
                    pos_start: token.clone().pos_start,
                    pos_end: self.current_token.clone().pos_end,
                });
            }

            return res.success(Node::VarAccessNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            if self.current_token.clone().r#type != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.clone().pos_start,
                    self.current_token.clone().pos_end,
                    "Expected ')'",
                ));
            }

            self.advance();
            return res.success(expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            let if_expr = res.register(self.if_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(if_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            let while_expr = res.register(self.while_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(while_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            let for_expr = res.register(self.for_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(for_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            let fun_def = res.register(self.fun_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(fun_def.unwrap());
        }

        res.failure(Error::new(
            "Invalid Syntax",
            token.pos_start,
            token.pos_end,
            "A Int, Float, String, Char, Keyword, Identifier, '+', '-', '(', etc was Expected",
        ))
    }

    pub fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'if'",
            ));
        }

        res.register_advancement();
        self.advance();

        let pos_start = self.current_token.clone().pos_start;
        let mut cases: Vec<(Node, Node)> = vec![];
        let mut else_case: Option<Node> = None;

        let first_condition = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let first_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }
        cases.push((first_condition.unwrap(), first_expr.unwrap()));

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }
        self.advance();
        res.register_advancement();

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("else".to_string()))
        {
            res.register_advancement();
            self.advance();

            if self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("if".to_string()))
            {
                res.register_advancement();
                self.advance();

                let condition = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::LeftCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '{'",
                    ));
                }

                res.register_advancement();
                self.advance();

                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                cases.push((condition.unwrap(), expr.unwrap()));

                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::RightCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '}'",
                    ));
                }
                res.register_advancement();
                self.advance();
            } else {
                let else_expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }
                else_case = Some(else_expr.unwrap());
                res.register_advancement();
                self.advance();
                break;
            }
        }
        res.success(Node::IfNode {
            cases,
            else_case: Box::new(else_case.clone()),
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    pub fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'while'",
            ));
        }

        res.register_advancement();
        self.advance();

        let condition_node = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body_node = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.success(Node::WhileNode {
            condition_node: Box::new(condition_node.clone().unwrap()),
            body_node: Box::new(body_node.clone().unwrap()),
            pos_start: pos_start.clone(),
            pos_end: self.current_token.clone().pos_end,
        })
    }

    pub fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'for'",
            ));
        }

        res.register_advancement();
        self.advance();
        let start = self.current_token.clone().pos_start;

        if self.current_token.r#type != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Identifier",
            ));
        }

        let var_name = self.current_token.clone();
        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Equals {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '='",
            ));
        }

        res.register_advancement();
        self.advance();

        let init_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("to".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'to'",
            ));
        }

        res.register_advancement();
        self.advance();

        let end_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        let mut step: Option<Node> = None;
        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("step".to_string()))
        {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            step = Some(expr.unwrap());
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        self.advance();

        res.success(Node::ForNode {
            var_name_token: var_name,
            start_value: Box::new(init_expr.clone().unwrap()),
            end_value: Box::new(end_expr.clone().unwrap()),
            body_node: Box::new(body.clone().unwrap()),
            step_value_node: Box::new(step.clone()),
            pos_start: start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    pub fn fun_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'fun'",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut fun_name: Option<Token> = None;
        if self.current_token.r#type == Tokens::Identifier {
            fun_name = Some(self.current_token.clone());

            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::LeftParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '('",
                ));
            }
        } else if self.current_token.r#type != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '(' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut args_name_tokens: Vec<Token> = vec![];
        if self.current_token.r#type == Tokens::Identifier {
            let name = self.current_token.clone();
            args_name_tokens.push(name);

            res.register_advancement();
            self.advance();

            while self.current_token.r#type == Tokens::Comma {
                res.register_advancement();
                self.advance();

                if self.current_token.r#type == Tokens::Identifier {
                    let new_arg_token = self.current_token.clone();
                    args_name_tokens.push(new_arg_token);
                    res.register_advancement();
                    self.advance();
                } else {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected Identifier",
                    ));
                }
            }

            if self.current_token.r#type != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ')' or ','",
                ));
            }
        } else if self.current_token.r#type != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ')' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Arrow {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '=>'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body_node = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        res.success(Node::FunDef {
            name: fun_name,
            body_node: Box::new(body_node.clone().unwrap()),
            arg_tokens: args_name_tokens,
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }
}
