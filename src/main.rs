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
use std::io::Write;

#[derive(Clone, Debug, PartialEq)]
enum Token {
    /// types
    NUMBER {
        value: Number,
    },
    BOOL {
        value: bool,
    },
    /// identifiers
    IDENT {
        name: String,
    },
    BEGIN,
    END,
    /// comparison
    GREATER, // >
    LESS,    // <
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
    MOD,
    /// parentheses
    LPAREN,
    RPAREN,
    SEMI,
    COLON,
    COMMA,

    EMPTY,
    EOF,
}

impl Token {
    fn token_type(&self) -> String {
        use Token::*;
        match self {
            NUMBER { .. } => "NUMBER",
            BOOL { .. } => "BOOL",
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
            MOD => "MOD",
            SEMI => "SEMI",
            COLON => "COLON",
            COMMA => "COMMA",
            EOF => "EOF",
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
            BOOL { value } => Some(value),
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
    fn new<T: Into<String> + Clone>(text: T) -> Self {
        Self {
            text: text.into(),
            pos: 0,
            current_token: Token::EMPTY,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.get_char(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn peek(&mut self) -> Option<char> {
        let peek_pos = self.pos + 1;
        self.get_char(peek_pos)
    }

    fn get_current_token(&mut self) -> Token {
        if self.current_token == Token::EMPTY {
            self.next_token()
        }
        self.current_token.clone()
    }

    fn peek_token(&mut self) -> Token {
        let pos = self.pos;
        let current_token = self.current_token.clone();

        let token = self.get_next_token();
        self.pos = pos;
        self.current_token = current_token;
        token
    }

    fn get_next_token(&mut self) -> Token {
        while let Some(cs) = self.current_char() {
            if cs.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if cs.is_alphabetic() {
                let id = self.ident();
                if id == "true" || id == "false" {
                    return Token::BOOL { value: id == "true" };
                }
                return Token::IDENT { name: id };
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
                '^' => {
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
                    self.advance();
                    return Token::MINUS;
                }
                '*' => {
                    self.advance();
                    return Token::MUL;
                }
                '/' => {
                    self.advance();
                    return Token::DIV;
                }
                '%' => {
                    self.advance();
                    return Token::MOD;
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

    fn eof(&mut self) -> bool {
        self.current_token == Token::EOF
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

    fn read_text(&mut self) -> std::io::Result<()> {
        use std::io;
        std::io::stdout().write(b"#/> ")?;
        io::stdout().flush()?;
        let buf = &mut String::new();
        io::stdin().read_line(buf)?;
        self.text = buf.to_owned();
        self.pos = 0;
        self.current_token = Token::EMPTY;
        Ok(())
    }
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
        arguments: Vec<String>,
        body: Box<Node>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Box<Node>>,
        body: Box<Node>,
        scope: BTreeMap<String, Box<Node>>,
    },
    Statement {
        statement: Statement,
    },
    Number {
        token: Token,
    },
    Bool {
        token: Token,
    },
    Variable {
        name: String,
    },
}

impl Node {
    fn node_type(&self) -> String {
        use Node::*;
        match self {
            UnaryOperation { token, .. }
            | BinaryOperation { token, .. }
            | Number { token }
            | Bool { token } => token.token_type(),
            FunctionCall { .. } => "FUNCTION_CALL".to_string(),
            Function { .. } => "FUNCTION".to_string(),
            Statement { .. } => "STATEMENT".to_string(),
            Variable { .. } => "VARIABLE".to_string(),
        }
    }

    fn format(&self) -> (String, String) {
        use Node::*;
        match self {
            UnaryOperation { .. }
            | BinaryOperation { .. }
            | Number { .. }
            | Statement { .. }
            | FunctionCall { .. } => (
                "< ".to_owned(),
                format!("{}", self.value(BTreeMap::new()).unwrap()),
            ),
            Variable { .. } => (
                "< ".to_owned(),
                format!("{}", self.is_true(BTreeMap::new()).unwrap()),
            ),
            Function {
                name, arguments, ..
            } => (
                "# ".to_owned(),
                format!("function {}({}) ", name, arguments.len()),
            ),
            Bool { .. } => (
                "< ".to_owned(),
                format!("{}", self.is_true(BTreeMap::new()).unwrap()),
            ),
        }
    }

    fn is_true(&self, parent_scope: BTreeMap<String, Box<Node>>) -> Option<bool> {
        use Node::*;
        match self {
            UnaryOperation { token, right } => match token {
                Token::NOT => right.is_true(parent_scope.clone()).map(|b| !b),
                _ => None,
            },
            BinaryOperation { left, token, right } => match token {
                Token::AND => left.is_true(parent_scope.clone())
                    .and_then(|a| right.is_true(parent_scope.clone()).map(|b| a && b)),
                Token::OR => left.is_true(parent_scope.clone())
                    .and_then(|a| right.is_true(parent_scope.clone()).map(|b| a || b)),
                Token::XOR => left.is_true(parent_scope.clone())
                    .and_then(|a| right.is_true(parent_scope.clone()).map(|b| a ^ b)),
                Token::EQUAL => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a == b)),
                Token::NEQUAL => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a != b)),
                Token::LESS => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a < b)),
                Token::GREATER => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a > b)),
                Token::LEQUAL => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a <= b)),
                Token::GEQUAL => left.value(parent_scope.clone())
                    .and_then(|a| right.value(parent_scope.clone()).map(|b| a >= b)),
                _ => None,
            },
            Function { .. } => None,
            Statement { statement } => {
                use Statement::*;
                match statement {
                    Expression(expr) => expr.is_true(parent_scope.clone()),
                    Condition {
                        condition,
                        statement,
                        statement_else,
                    } => {
                        if condition.is_true(parent_scope.clone()) == Some(true) {
                            statement.is_true(parent_scope.clone())
                        } else {
                            statement_else.is_true(parent_scope.clone())
                        }
                    }
                }
            }
            Bool { token: val } => val.is_true(),
            Number { .. } => None,
            Variable { name } => {
                let value = parent_scope.get(name).unwrap().to_owned();
                value.is_true(parent_scope.clone())
            }
            FunctionCall { body, scope, .. } => body.is_true(scope.clone()),
        }
    }

    fn value(&self, parent_scope: BTreeMap<String, Box<Node>>) -> Option<Number> {
        use Node::*;
        match self {
            UnaryOperation { token, right } => match token {
                Token::PLUS => right.value(parent_scope.clone()),
                Token::MINUS => right.value(parent_scope.clone()).map(|n| -n),
                _ => None,
            },
            BinaryOperation { left, token, right } => match token {
                Token::PLUS => left.value(parent_scope.clone())
                    .and_then(|x| right.value(parent_scope.clone()).map(|y| x + y)),
                Token::MINUS => left.value(parent_scope.clone())
                    .and_then(|x| right.value(parent_scope.clone()).map(|y| x - y)),
                Token::MUL => left.value(parent_scope.clone())
                    .and_then(|x| right.value(parent_scope.clone()).map(|y| x * y)),
                Token::DIV => left.value(parent_scope.clone())
                    .and_then(|x| right.value(parent_scope.clone()).map(|y| x / y)),
                Token::MOD => left.value(parent_scope.clone())
                    .and_then(|x| right.value(parent_scope.clone()).map(|y| x % y)),
                _ => None,
            },
            Number { token } => token.value(),
            Function { .. } => None,
            Statement { statement } => {
                use Statement::*;
                match statement {
                    Expression(expr) => expr.value(parent_scope.clone()),
                    Condition {
                        condition,
                        statement,
                        statement_else,
                    } => {
                        if condition.is_true(parent_scope.clone()) == Some(true) {
                            statement.value(parent_scope.clone())
                        } else {
                            statement_else.value(parent_scope.clone())
                        }
                    }
                }
            }
            Bool { .. } => None,
            Variable { name } => {
                let value = parent_scope.get(name).unwrap().to_owned();
                value.value(parent_scope.clone())
            }
            FunctionCall { body, scope, .. } => body.value(scope.clone()),
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

    fn with_text<T: Into<String> + Clone>(text: T) -> Self {
        Self {
            lexer: Lexer::new(text),
            functions: BTreeMap::new(),
        }
    }

    fn append_text<T: Into<String> + Clone>(&mut self, text: T) {
        self.lexer.text = text.into();
        self.lexer.pos = 0;
        self.lexer.current_token = Token::EMPTY;
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

    fn error<T: Into<String>>(&mut self, message: T) {
        self.lexer.error(message);
    }

    fn wait(&mut self) {
        if self.lexer.get_current_token() == Token::EOF {
            if let Ok(_) = self.lexer.read_text() {}
        }
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

    fn function(&mut self) -> Node {
        self.eat("IDENT");
        let mut args = Vec::new();
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
            if let Token::IDENT { name: arg_ } = self.lexer.get_current_token() {
                self.eat("IDENT");
                arg = arg_;
            } else {
                unreachable!()
            }
            args.push(arg);

            while !self.lexer.eof() {
                if self.lexer.get_current_token().token_type() == "RPAREN" {
                    break;
                }
                self.eat("COMMA");
                let arg;
                if let Token::IDENT { name: arg_ } = self.lexer.get_current_token() {
                    self.eat("IDENT");
                    arg = arg_;
                } else {
                    unreachable!()
                }
                args.push(arg);
            }

            self.eat("RPAREN");
        }

        self.wait();
        self.eat("BEGIN");

        self.wait();
        let body = Box::new(self.statement());

        self.wait();
        self.eat("END");

        let function = Node::Function {
            name: name.clone(),
            arguments: args.clone(),
            body,
        };
        self.functions.insert(name, function.clone());
        function
    }

    fn statement(&mut self) -> Node {
        let token = self.lexer.get_current_token();
        if token.token_type() == "IDENT" {
            if token.name() == Some("if".to_string()) {
                self.eat("IDENT");
                let condition = self.compound_condition();

                self.wait();
                self.eat("BEGIN");

                self.wait();
                let statement = Box::new(self.statement());

                self.wait();
                self.eat("END");

                self.wait();
                self.eat("IDENT");

                self.wait();
                self.eat("BEGIN");

                self.wait();
                let statement_else = Box::new(self.statement());

                self.wait();
                self.eat("END");

                return Node::Statement {
                    statement: Statement::Condition {
                        condition: Box::new(condition),
                        statement,
                        statement_else,
                    },
                };
            }
        }
        let expression = self.expression();

        Node::Statement {
            statement: Statement::Expression(Box::new(expression)),
        }
    }

    fn expression(&mut self) -> Node {
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
                if self.lexer.peek_token() == Token::LPAREN {
                    self.function_call(name)
                } else {
                    self.variable(name)
                }
            }
            Token::LPAREN => {
                self.eat("LPAREN");
                let node = self.expression();
                self.eat("RPAREN");
                node
            }
            _ => unreachable!(format!(
                "{:?} {:?} {:?}",
                token.clone(),
                self.lexer.pos,
                self.lexer.text
            )),
        }
    }

    fn simple_condition(&mut self) -> Node {
        let node = self.expression();

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
            _ => self.error(format!("bad operation ({:?})", op)),
        }

        Node::BinaryOperation {
            left: Box::new(node),
            token: op,
            right: Box::new(self.expression()),
        }
    }

    fn condition(&mut self) -> Node {
        let token = self.lexer.get_current_token();
        match token.clone() {
            Token::BOOL { .. } => {
                self.eat("BOOL");
                Node::Bool {
                    token: token.clone(),
                }
            }
            Token::LPAREN => {
                self.eat("LPAREN");
                let node = self.compound_condition();
                self.eat("RPAREN");
                node
            }
            Token::NOT => {
                self.eat("NOT");
                Node::UnaryOperation {
                    token: Token::NOT,
                    right: Box::new(self.compound_condition()),
                }
            }
            _ => self.simple_condition(),
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

    fn variable(&mut self, name: String) -> Node {
        self.eat("IDENT");

        Node::Variable { name }
    }

    fn function_call(&mut self, name: String) -> Node {
        self.eat("IDENT");

        if !self.functions.contains_key(&name) {
            self.error(format!("function {} is not exist", name))
        }

        let body;
        let args;
        match self.functions.get(&name).unwrap() {
            &Node::Function {
                body: ref body_,
                arguments: ref args_,
                ..
            } => {
                body = body_.clone();
                args = args_.clone();
            }
            _ => unreachable!(),
        }

        self.eat("LPAREN");
        let (arguments, scope) = self.arguments(args.clone());
        self.eat("RPAREN");

        Node::FunctionCall {
            name,
            arguments,
            body,
            scope,
        }
    }

    fn parse(&mut self) -> Node {
        self.line()
    }

    fn arguments(&mut self, args: Vec<String>) -> (Vec<Box<Node>>, BTreeMap<String, Box<Node>>) {
        let mut scope = BTreeMap::new();

        let mut ans = Vec::new();
        let arg = self.lexer.get_current_token();
        if arg == Token::RPAREN {
            return (ans, scope);
        }
        let mut i = 0;
        let value = Box::new(self.expression());
        ans.push(value.clone());
        scope.insert(args[i].clone(), value.clone());
        i = i + 1;

        while !self.lexer.eof() {
            let arg = self.lexer.get_current_token();
            if arg == Token::RPAREN {
                break;
            }
            self.eat("COMMA");
            let value = Box::new(self.expression());
            ans.push(value.clone());
            scope.insert(args[i].clone(), value.clone());
            i = i + 1
        }

        (ans, scope)
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

    fn with_text<T: Into<String> + Clone>(text: T) -> Self {
        Self {
            parser: Parser::with_text(text),
        }
    }

    fn parse(&mut self) -> Node {
        self.parser.parse()
    }

    fn interpret(&mut self) -> String {
        let res = self.parse().format();
        format!("{}{}", res.0, res.1)
    }

    fn append_text<T: Into<String> + Clone>(&mut self, text: T) {
        self.parser.append_text(text)
    }
}

#[cfg(test)]
mod tests;
#[macro_use]
mod numbers;
mod utils;

fn main() -> std::io::Result<()> {
    use std::io;
    let stdin = io::stdin();
    let buf = &mut String::new();
    let mut interpreter = Interpreter::new();

    std::io::stdout().write(b"#>> ")?;
    std::io::stdout().flush()?;
    while let Ok(_) = stdin.read_line(buf) {
        interpreter.append_text(buf.to_owned());
        let res = interpreter.interpret();
        std::io::stdout().write(format!("#<{}\n", res).as_bytes())?;
        std::io::stdout().flush()?;
        std::io::stdout().write(b"#>> ")?;
        std::io::stdout().flush()?;
        buf.clear();
    }

    Ok(())
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
