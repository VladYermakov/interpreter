// Copyright 2018 Vlad Yermakov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use numbers::Number;

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    /// types
    NUMBER {
        value: Number,
    },
    BOOL(bool),
    /// identifiers
    IDENT {
        name: String,
    },
    BEGIN,
    END,
    /// comparison
    LESS,    // <
    GREATER, // >
    EQUAL,   // =
    NEQUAL,  // !=
    LEQUAL,  // <=
    GEQUAL,  // >=
    /// comparison operators
    AND, // &
    OR,  // |
    NOT, // !
    XOR, // ^
    /// operations
    PLUS,
    MINUS,
    MUL,
    DIV,
    /// parentheses
    LPAREN,
    RPAREN,
    SEMI,
    COLON,
    COMMA,

    RETURNING,

    EMPTY,
    EOF,
}

impl Token {
    fn token_type(&self) -> String {
        use Token::*;
        match self {
            NUMBER { .. } => "NUMBER",
            BOOL(..) => "BOOL",
            IDENT { .. } => "IDENT",
            BEGIN => "BEGIN",
            END => "END",
            LESS => "LESS",
            GREATER => "GREATER",
            EQUAL => "EQUAL",
            NEQUAL => "NEQUAL",
            LEQUAL => "LEQUAL",
            GEQUAL => "GEQUAL",
            AND => "AND",
            OR => "OR",
            NOT => "NOT",
            XOR => "XOR",
            PLUS => "PLUS",
            MINUS => "MINUS",
            MUL => "MUL",
            DIV => "DIV",
            SEMI => "SEMI",
            COLON => "COLON",
            COMMA => "COMMA",
            EOF => "EOF",
            RETURNING => "RETURNING",
            LPAREN => "LPAREN",
            RPAREN => "RPAREN",
            EMPTY => "EMPTY",
        }.to_string()
    }

    fn value(&self) -> Option<Number> {
        use Token::*;
        match *self {
            NUMBER { value } => Some(value),
            _ => None,
        }
    }

    fn is_true(&self) -> Option<bool> {
        use Token::*;
        match *self {
            BOOL(value) => Some(value),
            _ => None,
        }
    }

    fn name(&self) -> Option<String> {
        use Token::*;
        match self {
            IDENT { name } => Some(name.to_owned()),
            _ => None,
        }
    }
}

struct Lexer {
    text: String,
    pos: usize,
    current_token: Token,
}

impl Lexer {
    fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            pos: 0,
            current_token: Token::EMPTY,
        }
    }

    fn get_current_token(&mut self) -> Token {
        if self.current_token == Token::EMPTY {
            self.next_token()
        }
        self.current_token.clone()
    }

    fn eof(&mut self) -> bool {
        self.current_token == Token::EOF
    }

    fn current_char(&self) -> Option<char> {
        self.get_char(self.pos)
    }

    fn get_char(&self, pos: usize) -> Option<char> {
        if pos >= self.text.len() {
            None
        } else {
            Some(self.text[pos..].chars().next().unwrap())
        }
    }

    fn error<T: Into<String>>(&self, message: T) {
        panic!(format!(
            "Syntax Error: {}, pos: {}, current_char: {:?}",
            message.into(),
            self.pos,
            self.current_char()
        ))
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn peek(&mut self) -> Option<char> {
        let peek_pos = self.pos + 1;
        self.get_char(peek_pos)
    }

    fn skip_whitespace(&mut self) {
        while let Some(cs) = self.current_char() {
            if cs.is_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }

    fn ident(&mut self) -> String {
        let mut result = String::new();
        while let Some(cs) = self.current_char() {
            if cs.is_alphanumeric() {
                result.push(cs);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn number(&mut self) -> Number {
        let mut num = String::new();
        let mut den = String::new();
        let mut rat = false;
        let mut rea = false;
        let mut com = false;

        while let Some(cs) = self.current_char() {
            if cs.is_digit(10) {
                if rat {
                    den.push(cs);
                } else {
                    num.push(cs);
                }
                self.advance();
            } else {
                if cs == '/' && self.peek() == Some('/') {
                    if rat || rea {
                        if rat {
                            self.error(format!("expected 0..9 found //"))
                        } else {
                            self.error(format!("expected 0..9 or i found ."))
                        }
                    } else {
                        rat = true;
                        self.advance();
                        self.advance();
                        continue;
                    }
                }
                if cs == '.' {
                    if rat || rea {
                        if rat {
                            self.error(format!("expected 0..9 found ."))
                        } else {
                            self.error(format!("expected 0..9 or i found ."))
                        }
                    } else {
                        num.push(cs);
                        rea = true;
                        self.advance();
                        continue;
                    }
                }
                if cs == 'i' {
                    if rat {
                        self.error(format!("expected 0..9 found i"))
                    } else {
                        rea = true;
                        com = true;
                        self.advance();
                        continue;
                    }
                }
                if cs.is_digit(10) {
                    self.error(format!("expected \" \" found {}", cs))
                } else if cs.is_alphabetic() {
                    self.error(format!("expected 0..9 found {}", cs))
                } else {
                    break;
                }
            }
        }
        if com {
            Number::complex(num).unwrap()
        } else if rea {
            Number::real(num).unwrap()
        } else if rat {
            Number::rational(num, den).unwrap()
        } else {
            Number::natural(num).unwrap()
        }
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(cs) = self.current_char() {
            if cs.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if cs.is_alphabetic() {
                return Token::IDENT { name: self.ident() };
            }

            if cs.is_digit(10) {
                return Token::NUMBER {
                    value: self.number(),
                };
            }

            match cs {
                '{' => {
                    self.advance();
                    return Token::BEGIN;
                }
                '}' => {
                    self.advance();
                    return Token::END;
                }
                '≤' => {
                    self.advance();
                    return Token::LEQUAL;
                },
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        return Token::LEQUAL;
                    } else {
                        self.advance();
                        return Token::LESS;
                    }
                }
                '≥' => {
                    self.advance();
                    return Token::GEQUAL;
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        return Token::GEQUAL;
                    } else {
                        self.advance();
                        return Token::GREATER;
                    }
                }
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                    }
                    self.advance();
                    return Token::EQUAL;
                }
                '≠' => {
                    self.advance();
                    return Token::NEQUAL;
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        return Token::NEQUAL;
                    } else {
                        self.advance();
                        return Token::NOT;
                    }
                }
                '&' => {
                    self.advance();
                    return Token::AND;
                }
                '|' => {
                    self.advance();
                    return Token::OR;
                }
                '^'  => {
                    self.advance();
                    return Token::XOR;
                }
                '+' => {
                    self.advance();
                    return Token::PLUS;
                }
                ';' => {
                    self.advance();
                    return Token::SEMI;
                }
                ':' => {
                    self.advance();
                    return Token::COLON;
                }
                ',' => {
                    self.advance();
                    return Token::COMMA;
                }
                '-' => {
                    if self.peek() == Some('>') {
                        self.advance();
                        self.advance();
                        return Token::RETURNING;
                    } else {
                        self.advance();
                        return Token::MINUS;
                    }
                }
                '*' => {
                    self.advance();
                    return Token::MUL;
                }
                '/' => {
                    self.advance();
                    return Token::DIV;
                }
                '(' => {
                    self.advance();
                    return Token::LPAREN;
                }
                ')' => {
                    self.advance();
                    return Token::RPAREN;
                }
                _ => self.error(""),
            }
        }
        Token::EOF
    }

    fn next_token(&mut self) {
        self.current_token = self.get_next_token()
    }
}

trait NodeTrait {
    fn node_type(&self) -> String;
    fn print(&self);
    fn format(&self) -> String;
    fn is_true(&self) -> bool;
    fn value(&self) -> Number;
}

#[derive(Clone, Debug)]
enum Statement {
    Expression(Box<Node>),
    Condition {
        condition: Box<Node>,
        statement: Box<Node>,
        statement_else: Box<Node>,
    },
}

#[derive(Clone, Debug)]
enum Node {
    UnaryOperation {
        token: Token,
        right: Box<Node>,
    },
    BinaryOperation {
        left: Box<Node>,
        token: Token,
        right: Box<Node>,
    },
    Function {
        name: String,
        arguments: Vec<(String, String)>,
        types: Vec<String>,
        body: Box<Node>,
    },
    Statement(Statement),
    Number {
        token: Token,
    },
    Variable {
        name: String,
        token: Token,
    }, //    Empty
}

impl NodeTrait for Node {
    fn node_type(&self) -> String {
        use Node::*;
        match self {
            UnaryOperation { token, .. } | BinaryOperation { token, .. } | Number { token } => {
                token.token_type()
            }
            Function { .. } => "FUNCTION".to_string(),
            Statement(_) => "STATEMENT".to_string(),
            Variable { .. } => "VARIABLE".to_string(),
        }
    }

    fn print(&self) {
        use Node::*;
        match self {
            UnaryOperation { .. }
            | BinaryOperation { .. }
            | Number { .. }
            | Function { .. }
            | Statement(..)
            | Variable { .. } => println!("{}", self.value()),
        }
    }

    fn format(&self) -> String {
        use Node::*;
        match self {
            UnaryOperation { .. }
            | BinaryOperation { .. }
            | Number { .. }
            | Function { .. }
            | Statement(..)
            | Variable { .. } => format!("{}", self.value()),
        }
    }

    fn is_true(&self) -> bool {
        use Node::*;
        match self {
            UnaryOperation { token, right } => match token {
                Token::NOT => !right.is_true(),
                _ => panic!("Bad operation")
            },
            BinaryOperation {left, token, right } => match token {
                Token::AND => left.is_true() && right.is_true(),
                Token::OR => left.is_true() || right.is_true(),
                Token::XOR => left.is_true() ^ right.is_true(),
                Token::EQUAL => left.value() == right.value(),
                Token::NEQUAL => left.value() != right.value(),
                Token::LESS => left.value() < right.value(),
                Token::GREATER => left.value() > right.value(),
                Token::LEQUAL => left.value() <= right.value(),
                Token::GEQUAL => left.value() >= right.value(),
                _ => panic!("Bad operation")
            }
            Function { body, .. } => body.is_true(),
            Statement(statement) => {
                use Statement::*;
                match statement {
                    Expression(expr) => expr.is_true(),
                    Condition { condition, statement, statement_else } => {
                        if condition.is_true() {
                            statement.is_true()
                        } else {
                            statement_else.is_true()
                        }
                    }
                }
            },
            Number { .. } => panic!("trying to evaluate number as bool"),
            Variable { .. } => unimplemented!()
        }
    }

    fn value(&self) -> Number {
        use Node::*;
        match self {
            UnaryOperation { token, right } => match token {
                Token::PLUS => right.value(),
                Token::MINUS => -right.value(),
                _ => panic!("Bad operation"),
            },
            BinaryOperation { left, token, right } => match token {
                Token::PLUS => left.value() + right.value(),
                Token::MINUS => left.value() - right.value(),
                Token::MUL => left.value() * right.value(),
                Token::DIV => left.value() / right.value(),
                _ => panic!("Bad operation"),
            },
            Number { token } => token.value().unwrap(),
            Function { body, .. } => body.value(),
            Statement(statement) => {
                use Statement::*;
                match statement {
                    Expression(expr) => expr.value(),
                    Condition { condition, statement, statement_else } => {
                        if condition.is_true() {
                            statement.value()
                        } else {
                            statement_else.value()
                        }
                    },
                }
            }
            Variable { .. } => unimplemented!(),
        }
    }
}

struct Parser {
    lexer: Lexer,
    functions: BTreeMap<String, Node>,
}

impl Parser {
    fn new() -> Self {
        Self {
            lexer: Lexer::new(""),
            functions: BTreeMap::new(),
        }
    }

    fn append_text<T: Into<String>>(&self, text: T) -> Self {
        Self {
            lexer: Lexer::new(text),
            functions: self.functions.clone()
        }
    }

    fn error<T: Into<String>>(&mut self, message: T) {
        self.lexer.error(message);
    }

    fn eat<T: Into<String>>(&mut self, tt: T) -> Token {
        let tt = tt.into();
        let token = self.lexer.get_current_token();

        if token.token_type() == tt {
            self.lexer.next_token();
        } else {
            self.error(format!(
                "can't parse {} expected {}",
                token.token_type(),
                tt
            ))
        }
        token
    }

    fn factor(&mut self) -> Node {
        let token = self.lexer.get_current_token();

        match token.clone() {
            Token::PLUS => {
                self.eat("PLUS");
                Node::UnaryOperation {
                    token,
                    right: Box::new(self.factor()),
                }
            }
            Token::MINUS => {
                self.eat("MINUS");
                Node::UnaryOperation {
                    token,
                    right: Box::new(self.factor()),
                }
            }
            Token::NUMBER { .. } => {
                self.eat("NUMBER");
                Node::Number { token }
            }
            Token::IDENT { name } => {
                self.eat("IDENT");
                Node::Variable { name, token }
            }
            Token::LPAREN => {
                self.eat("LPAREN");
                let node = self.expr();
                self.eat("RPAREN");
                node
            }
            _ => unreachable!(),
        }
    }

    fn term(&mut self) -> Node {
        let mut node = self.factor();

        while !self.lexer.eof() {
            let op = self.lexer.get_current_token();

            match op {
                Token::MUL => {
                    self.eat("MUL");
                }
                Token::DIV => {
                    self.eat("DIV");
                }
                _ => break,
            }

            node = Node::BinaryOperation {
                left: Box::new(node),
                token: op,
                right: Box::new(self.factor()),
            }
        }

        node
    }

    fn expr(&mut self) -> Node {
        let mut node = self.term();

        while !self.lexer.eof() {
            let op = self.lexer.get_current_token();

            match op {
                Token::PLUS => {
                    self.eat("PLUS");
                }
                Token::MINUS => {
                    self.eat("MINUS");
                }
                _ => break //self.error(format!("bad operation ({:?})", op)),
            }

            node = Node::BinaryOperation {
                left: Box::new(node),
                token: op,
                right: Box::new(self.term()),
            }
        }

        node
    }

    fn parse(&mut self) -> Node {
        self.line()
    }

    fn simple_condition(&mut self) -> Node {
        let mut node = self.expr();

        let op = self.lexer.get_current_token();

        match op {
            Token::EQUAL => {
                self.eat("EQUAL");
            }
            Token::NEQUAL => {
                self.eat("NEQUAL");
            }
            Token::LESS => {
                self.eat("LESS");
            }
            Token::GREATER => {
                self.eat("GREATER");
            }
            Token::LEQUAL => {
                self.eat("LEQUAL");
            }
            Token::GEQUAL => {
                self.eat("GEQUAL");
            }
            _ => break //self.error(format!("bad operation ({:?})", op)),
        }

        Node::BinaryOperation {
            left: Box::new(node),
            token: op,
            right: Box::new(self.expr()),
        }
    }

    fn condition(&mut self) -> Node {
        let token = self.lexer.get_current_token();
        match token.clone() {
            Token::LPAREN => {
                self.eat("LPAREN");
                node = self.compound_condition();
                self.eat("RPAREN");
                node
            }
            Token::NOT => {
                self.eat("NOT");
                Node::UnaryOperation {
                    token: Token::Not,
                    right: Box::new(self.compound_condition())
                }
            }
            _ => {
                self.simple_condition()
            }
        }

    }

    fn compound_condition(&mut self) -> Node {
        let mut node = self.condition();

        while !self.lexer.eof() {
            let op = self.lexer.get_current_token();

            match op {
                Token::AND => {
                    self.eat("AND");
                }
                Token::OR => {
                    self.eat("OR");
                }
                Token::XOR => {
                    self.eat("XOR");
                }
                _ => break //self.error(format!("bad operation ({:?})", op)),
            }

            node = Node::BinaryOperation {
                left: Box::new(node),
                token: op,
                right: Box::new(self.condition()),
            }
        }

        node
    }

    fn line(&mut self) -> Node {
        let token = self.lexer.get_current_token();
        if token.token_type() == "IDENT" {
            if token.name() == Some("fn".to_string()) {
                return self.function();
            }
        }
        self.statement()
    }

    fn statement(&mut self) -> Node {
        let token = self.lexer.get_current_token();
        if token.token_type() == "IDENT" {
            if token.name() == Some("if".to_string()) {
                self.eat("IDENT");
                let condition = self.compound_condition();
                self.eat("BEGIN");
                let statement = Box::new(self.statement());
                let mut statement_else = None;
                self.eat("END");

                if self.lexer.get_current_token().token_type() == "IDENT" {
                    if self.lexer.get_current_token().name() == Some("else".to_string()) {
                        self.eat("IDENT");
                        self.eat("BEGIN");
                        statement_else = Box::new(self.statement());
                        self.eat("END");
                    } else {
                        let name = self.lexer.get_current_token().name().unwrap();
                        self.error(format!("expected \"else\" found \"{:?}\"", name))
                    }
                }

                return Node::Statement(Statement::Condition {
                    condition: Box::new(condition),
                    statement,
                    statement_else,
                });
            }
        }
        let expression = self.expr();
        Node::Statement(Statement::Expression(Box::new(expression)))
    }

    fn function(&mut self) -> Node {
        self.eat("IDENT");
        let mut args = Vec::new();
        let mut return_types = Vec::new();
        let name;

        if let Token::IDENT { name: name_ } = self.lexer.get_current_token() {
            name = name_;
            self.eat("IDENT");
        } else {
            unreachable!()
        }

        self.eat("LPAREN");

        if self.lexer.get_current_token().token_type() == "RPAREN" {
            self.eat("RPAREN");
        } else {
            let arg;
            let arg_type;
            if let Token::IDENT { name: arg_ } = self.lexer.get_current_token() {
                self.eat("IDENT");
                arg = arg_;
            } else {
                unreachable!()
            }
            self.eat("COLON");
            if let Token::IDENT { name: type_ } = self.lexer.get_current_token() {
                self.eat("IDENT");
                arg_type = type_;
            } else {
                unreachable!()
            }
            args.push((arg, arg_type));

            while !self.lexer.eof() {
                if self.lexer.get_current_token().token_type() == "RPAREN" {
                    break;
                }
                self.eat("COMMA");
                let arg;
                let arg_type;
                if let Token::IDENT { name: arg_ } = self.lexer.get_current_token() {
                    self.eat("IDENT");
                    arg = arg_;
                } else {
                    unreachable!()
                }
                self.eat("COLON");
                if let Token::IDENT { name: type_ } = self.lexer.get_current_token() {
                    self.eat("IDENT");
                    arg_type = type_;
                } else {
                    unreachable!()
                }
                args.push((arg, arg_type));
            }

            self.eat("RPAREN");
        }

        if self.lexer.get_current_token().token_type() == "RETURNING" {
            self.eat("RETURNING");
            let return_type;
            if let Token::IDENT { name: return_type_ } = self.lexer.get_current_token() {
                self.eat("IDENT");
                return_type = return_type_;
            } else {
                unreachable!()
            }
            return_types.push(return_type);

            while !self.lexer.eof() {
                if self.lexer.get_current_token().token_type() == "BEGIN" {
                    break;
                }
                self.eat("COMMA");
                let return_type;
                if let Token::IDENT { name: return_type_ } = self.lexer.get_current_token() {
                    self.eat("IDENT");
                    return_type = return_type_;
                } else {
                    unreachable!()
                }
                return_types.push(return_type);
            }
        }

        self.eat("BEGIN");

        let body = Box::new(self.statement());

        self.eat("END");

        let function = Node::Function {
            name: name.clone(),
            arguments: args.clone(),
            types: return_types.clone(),
            body,
        };
        self.functions.insert(name, function.clone());
        function
    }
}

struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    fn new_with_text<T: Into<String>>(text: T) -> Self {
        Self {
            parser: Parser::new(),
        }.append_text(text)
    }

    fn interpret(&mut self) -> String {
        self.parser.parse().format()
    }

    fn eval(&mut self) {
        self.parser.parse().print()
    }

    fn append_text<T: Into<String>>(&self, text: T) -> Self {
        Self {
            parser: self.parser.append_text(text)
        }
    }
}

#[cfg(test)]
mod tests;
#[macro_use]
mod numbers;
mod utils;

fn main() {
    use std::io;
    let stdin = io::stdin();
    let buf = &mut String::new();
    let interpreter = Interpreter::new();

    while let Ok(_) = stdin.read_line(buf) {
        interpreter.append_text(buf.to_owned()).eval();
    }
}

// 2
// 2 // 3
// 2 + 3i

// `1 + 2 + 3 + 4 + 5 + 6 + 7 + 8`
// ```
//                 +
//                / \
//               +   8
//              / \
//             +   7
//            / \
//           +   6
//          / \
//         +   5
//        / \
//       +   4
//      / \
//     +   3
//    / \
//   1   2
// ```
// `((1 + 2) + (3 + 4)) + ((5 + 6) + (7 + 8))`
// ```
//                +
//               / \
//              /   \
//             /     \
//            /       \
//           /         \
//          +           +
//         / \         / \
//        /   \       /   \
//       +     +     +     +
//      / \   / \   / \   / \
//     1   2 3   4 5   6 7   8
// ```
