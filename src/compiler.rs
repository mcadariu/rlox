use crate::chunk::{Chunk, OpCode, Value};
use crate::scanner::{Scanner, Token, TokenType, init_scanner};
use crate::vm::VM;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    fn next(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::Primary,
        }
    }
}

struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    fn new(scanner: Scanner<'a>) -> Self {
        let dummy_token = Token {
            token_type: TokenType::Eof,
            lexeme: "",
            line: 0,
        };

        Parser {
            scanner,
            current: dummy_token,
            previous: dummy_token,
            had_error: false,
            panic_mode: false,
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.lexeme);
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        if token.token_type == TokenType::Eof {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing
        } else {
            eprint!(" at '{}'", token.lexeme);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }
}

struct Compiler<'a> {
    parser: Parser<'a>,
    chunk: Chunk,
    vm: &'a mut VM,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str, vm: &'a mut VM) -> Self {
        let scanner = init_scanner(source);
        let parser = Parser::new(scanner);

        Compiler {
            parser,
            chunk: Chunk::new(),
            vm,
        }
    }

    fn compile(mut self) -> Option<Chunk> {
        self.parser.advance();

        while !self.parser.check(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();

        if self.parser.had_error {
            None
        } else {
            Some(self.chunk)
        }
    }

    fn declaration(&mut self) {
        if self.parser.match_token(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.parser.match_token(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil);
        }

        self.parser
            .consume(TokenType::Semicolon, "Expect ';' after variable declaration.");

        self.define_variable(global);
    }

    fn statement(&mut self) {
        if self.parser.match_token(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OpPrint);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::Semicolon, "Expect ';' after expression.");
        self.emit_byte(OpCode::OpPop);
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode = false;

        while self.parser.current.token_type != TokenType::Eof {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.parser.current.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.parser.advance();
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let value: f64 = self.parser.previous.lexeme.parse().unwrap();
        self.emit_constant(Value::number(value));
    }

    fn string(&mut self) {
        let lexeme = self.parser.previous.lexeme;
        let string_value = lexeme[1..lexeme.len()-1].to_string();
        let interned = self.vm.intern_string(string_value);
        self.emit_constant(Value::string(interned));
    }

    fn variable(&mut self) {
        self.named_variable(self.parser.previous.lexeme, true);
    }

    fn named_variable(&mut self, name: &str, can_assign: bool) {
        let arg = self.identifier_constant(name);

        if can_assign && self.parser.match_token(TokenType::Equal) {
            self.expression();
            self.emit_bytes(OpCode::OpSetGlobal, arg);
        } else {
            self.emit_bytes(OpCode::OpGetGlobal, arg);
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OpFalse),
            TokenType::True => self.emit_byte(OpCode::OpTrue),
            TokenType::Nil => self.emit_byte(OpCode::OpNil),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.parser
            .consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        // Emit the operator instruction
        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::OpNot),
            TokenType::Minus => self.emit_byte(OpCode::OpNegate),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.token_type;
        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::BangEqual => {
                self.emit_byte(OpCode::OpEqual);
                self.emit_byte(OpCode::OpNot);
            }
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater),
            TokenType::GreaterEqual => {
                self.emit_byte(OpCode::OpLess);
                self.emit_byte(OpCode::OpNot);
            }
            TokenType::Less => self.emit_byte(OpCode::OpLess),
            TokenType::LessEqual => {
                self.emit_byte(OpCode::OpGreater);
                self.emit_byte(OpCode::OpNot);
            }
            TokenType::Plus => self.emit_byte(OpCode::OpAdd),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide),
            _ => unreachable!(),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();
        let prefix_rule = self.get_rule(self.parser.previous.token_type).prefix;

        match prefix_rule {
            None => {
                self.parser.error("Expect expression.");
                return;
            }
            Some(prefix_fn) => prefix_fn(self),
        }

        while precedence <= self.get_rule(self.parser.current.token_type).precedence {
            self.parser.advance();
            let infix_rule = self.get_rule(self.parser.previous.token_type).infix;
            if let Some(infix_fn) = infix_rule {
                infix_fn(self);
            }
        }

        if self.parser.check(TokenType::Equal) {
            self.parser.error("Invalid assignment target.");
        }
    }

    fn get_rule(&self, token_type: TokenType) -> ParseRule<'a> {
        match token_type {
            TokenType::LeftParen => {
                ParseRule::new(Some(Compiler::grouping), None, Precedence::None)
            }
            TokenType::Minus => ParseRule::new(
                Some(Compiler::unary),
                Some(Compiler::binary),
                Precedence::Term,
            ),
            TokenType::Plus => ParseRule::new(None, Some(Compiler::binary), Precedence::Term),
            TokenType::Slash => ParseRule::new(None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Star => ParseRule::new(None, Some(Compiler::binary), Precedence::Factor),
            TokenType::Bang => ParseRule::new(Some(Compiler::unary), None, Precedence::None),
            TokenType::BangEqual => {
                ParseRule::new(None, Some(Compiler::binary), Precedence::Equality)
            }
            TokenType::EqualEqual => {
                ParseRule::new(None, Some(Compiler::binary), Precedence::Equality)
            }
            TokenType::Greater => {
                ParseRule::new(None, Some(Compiler::binary), Precedence::Comparison)
            }
            TokenType::GreaterEqual => {
                ParseRule::new(None, Some(Compiler::binary), Precedence::Comparison)
            }
            TokenType::Less => ParseRule::new(None, Some(Compiler::binary), Precedence::Comparison),
            TokenType::LessEqual => {
                ParseRule::new(None, Some(Compiler::binary), Precedence::Comparison)
            }
            TokenType::Identifier => {
                ParseRule::new(Some(Compiler::variable), None, Precedence::None)
            }
            TokenType::Number => ParseRule::new(Some(Compiler::number), None, Precedence::None),
            TokenType::String => ParseRule::new(Some(Compiler::string), None, Precedence::None),
            TokenType::False => ParseRule::new(Some(Compiler::literal), None, Precedence::None),
            TokenType::True => ParseRule::new(Some(Compiler::literal), None, Precedence::None),
            TokenType::Nil => ParseRule::new(Some(Compiler::literal), None, Precedence::None),
            _ => ParseRule::new(None, None, Precedence::None),
        }
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        let line = self.parser.previous.line as usize;
        self.chunk.write(opcode, line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: u8) {
        self.emit_byte(byte1);
        let line = self.parser.previous.line as usize;
        self.chunk.write_byte(byte2, line);
    }

    fn parse_variable(&mut self, error_message: &str) -> u8 {
        self.parser.consume(TokenType::Identifier, error_message);
        self.identifier_constant(self.parser.previous.lexeme)
    }

    fn identifier_constant(&mut self, name: &str) -> u8 {
        let interned = self.vm.intern_string(name.to_string());
        self.chunk.add_constant(Value::string(interned)) as u8
    }

    fn define_variable(&mut self, global: u8) {
        self.emit_bytes(OpCode::OpDefineGlobal, global);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.chunk.add_constant(value);
        self.emit_bytes(OpCode::OpConstant, constant as u8);
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }
}

type ParseFn<'a> = fn(&mut Compiler<'a>);

struct ParseRule<'a> {
    prefix: Option<ParseFn<'a>>,
    infix: Option<ParseFn<'a>>,
    precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn new(
        prefix: Option<ParseFn<'a>>,
        infix: Option<ParseFn<'a>>,
        precedence: Precedence,
    ) -> Self {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }
}

pub fn compile(source: &str, vm: &mut VM) -> Option<Chunk> {
    let compiler = Compiler::new(source, vm);
    compiler.compile()
}
