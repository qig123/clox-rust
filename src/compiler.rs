use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token},
    token_type::{self, TokenType},
    value::Value,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None, // PREC_NONE: Lowest precedence, used for things like statements or expressions at the top level
    Assignment, // =       : Assignment operator
    Or,   // or      : Logical OR
    And,  // and     : Logical AND
    Equality, // == !=   : Equality comparisons
    Comparison, // < > <= >=: Relational comparisons
    Term, // + -     : Addition and subtraction
    Factor, // * /     : Multiplication and division
    Unary, // ! -     : Unary operators
    Call, // . ()    : Function calls and member access
    Primary, // PREC_PRIMARY: Highest precedence, used for literals, variables, grouping, etc.
}
// Type alias for parsing functions. They take a mutable reference to the Parser
// and perform parsing actions, reporting errors via parser.had_error.
pub type ParseFn = fn(&mut Parser); // <--- Changed signature
#[derive(Copy, Clone)] // Need Copy/Clone for static array initialization
pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}
pub struct Parser {
    current: Token,
    previous: Token,
    compiling_chunk: Chunk, // Parser owns the chunk it's compiling into
    scanner: Scanner,       // Parser owns its scanner
    had_error: bool,
    // Implemention note: If you were compiling multiple functions/methods,
    // you might need a stack of Compiler structs or similar state here,
    // but for a simple top-level script, owning the chunk is fine.
}
// ... (previous imports, structs)

// --- Wrapper Functions to bridge static table and methods ---
// These functions have the signature expected by ParseFn and call
// the actual parsing logic methods on the Parser instance.

fn number_rule(parser: &mut Parser) {
    parser.number();
}
fn grouping_rule(parser: &mut Parser) {
    parser.grouping();
}
fn unary_rule(parser: &mut Parser) {
    parser.unary();
}
fn binary_rule(parser: &mut Parser) {
    parser.binary();
}
// You'll need more wrapper functions for other rules:
// fn literal_string_rule(parser: &mut Parser) { parser.literal_string(); } // Implement parser.literal_string()
// fn identifier_rule(parser: &mut Parser) { parser.identifier(); } // Implement parser.identifier()
// fn dot_rule(parser: &mut Parser) { parser.dot(); } // Implement parser.dot() for method calls/property access
// fn call_rule(parser: &mut Parser) { parser.call(); } // Implement parser.call() for function calls
// fn and_rule(parser: &mut Parser) { parser.and(); } // Implement parser.and() for logical AND
// fn or_rule(parser: &mut Parser) { parser.or(); } // Implement parser.or() for logical OR
// fn assignment_rule(parser: &mut Parser) { parser.assignment(); } // Implement parser.assignment() for =
// ... and so on for any token that can start an expression (prefix)
// or appear between expressions (infix).

// Placeholder for methods not yet implemented, add these to impl Parser later:
// fn literal_string(&mut self) { /* ... */ }
// fn identifier(&mut self) { /* ... */ }
// fn dot(&mut self) { /* ... */ }
// fn call(&mut self) { /* ... */ }
// fn and(&mut self) { /* ... */ }
// fn or(&mut self) { parser.or(); }
// fn assignment(&mut self) { /* ... */ }
// ... (previous imports, structs, wrapper functions)

// --- Parse Rule Table ---
// This table maps TokenType to ParseRule structs.
// It defines how each token is parsed based on its position (prefix/infix)
// and its operator precedence.

// A helper function to get a rule from the static table
fn get_rule(kind: TokenType) -> &'static ParseRule {
    // Use the TokenType as an index. #[repr(usize)] ensures this is safe.
    &PARSE_RULES[kind as usize]
}

// The static parse rule table. Needs to be initialized completely.
// The order MUST match the order of variants in the TokenType enum
// because we are using `as usize` for indexing.
static PARSE_RULES: [ParseRule; TokenType::Count as usize] = [
    /* TokenType::LeftParen    */
    ParseRule {
        prefix: Some(grouping_rule),
        infix: None,
        precedence: Precedence::Call,
    }, // '(' can start a group or be part of a function call
    /* TokenType::RightParen   */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    /* TokenType::LeftBrace    */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Assuming not used in expressions
    /* TokenType::RightBrace   */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Assuming not used in expressions
    /* TokenType::Comma        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Assuming not used as operator
    /* TokenType::Dot          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::Call,
    }, // '.' for property access/method calls
    /* TokenType::Minus        */
    ParseRule {
        prefix: Some(unary_rule),
        infix: Some(binary_rule),
        precedence: Precedence::Term,
    }, // '-' as unary and binary
    /* TokenType::Plus         */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Term,
    },
    /* TokenType::Semicolon    */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Slash        */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Factor,
    },
    /* TokenType::Star         */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Factor,
    },
    /* TokenType::Bang         */
    ParseRule {
        prefix: Some(unary_rule),
        infix: None,
        precedence: Precedence::None,
    }, // '!' as unary prefix
    /* TokenType::BangEqual    */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Equality,
    },
    /* TokenType::Equal        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::Assignment,
    }, // '=' as assignment operator
    /* TokenType::EqualEqual   */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Equality,
    },
    /* TokenType::Greater      */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Comparison,
    },
    /* TokenType::GreaterEqual */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Comparison,
    },
    /* TokenType::Less         */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Comparison,
    },
    /* TokenType::LessEqual    */
    ParseRule {
        prefix: None,
        infix: Some(binary_rule),
        precedence: Precedence::Comparison,
    },
    /* TokenType::Identifier   */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    /* TokenType::String       */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    },
    /* TokenType::Number       */
    ParseRule {
        prefix: Some(number_rule),
        infix: None,
        precedence: Precedence::None,
    },
    /* TokenType::And          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::And,
    },
    /* TokenType::Class        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Else         */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::False        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Implement parser.literal_false()
    /* TokenType::For          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Fun          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::If           */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Nil          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Implement parser.literal_nil()
    /* TokenType::Or           */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::Or,
    },
    /* TokenType::Print        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Return       */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Super        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Implement parser.super_rule()
    /* TokenType::True         */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Implement parser.literal_true()
    /* TokenType::Var          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::While        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Not used in expressions
    /* TokenType::Error        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Handled by advance loop
    /* TokenType::Eof          */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Handled explicitly in compile
    /* TokenType::Count        */
    ParseRule {
        prefix: None,
        infix: None,
        precedence: Precedence::None,
    }, // Placeholder, never used for lookup
];

// ... (previous imports, enums, ParseFn, ParseRule, wrapper functions, PARSE_RULES, get_rule)

impl Parser {
    /* ========== 构造函数 ========== */
    // Constructor now takes source and chunk, and initializes the scanner
    pub fn new(source: String) -> Self {
        // Changed order to match typical usage
        Parser {
            current: Token {
                kind: token_type::TokenType::Eof,
                lexeme: "".to_string(),
                line: 0,
            },
            previous: Token {
                kind: token_type::TokenType::Eof,
                lexeme: "".to_string(),
                line: 0,
            },
            compiling_chunk: Chunk::new(), // Create the chunk here
            scanner: Scanner::new(source), // Initialize the scanner here
            had_error: false,
        }
    }

    /* ========== 主要编译入口 ========== */
    pub fn compile(&mut self) -> Result<Chunk, String> {
        // Does not take source or chunk now, they are owned by self
        self.had_error = false;
        // Note: panic_mode is sometimes used to track if we are currently
        // in a state where we are recovering from an error.
        // let mut panic_mode = false; // Could add a field for this if needed

        self.advance(); // Get the first token

        // In a real compiler, you would loop through declarations here:
        // while !self.match_token(TokenType::Eof) { // Need a match_token helper or check current.kind
        //    self.declaration(); // Implement statement/declaration parsing rules
        //    // If a parsing rule set the error flag, enter panic mode and synchronize
        //    if self.had_error && !panic_mode {
        //        panic_mode = true;
        //        self.synchronize();
        //    }
        //    if panic_mode && !self.check(TokenType::Semicolon) && !self.check(/* sync token */) {
        //        // Stay in panic mode, keep skipping if not at a sync point
        //    } else {
        //        // Exit panic mode if we hit a semicolon or sync token
        //        panic_mode = false;
        //    }
        // }

        // For a simple expression-only compiler:
        self.expression(); // Parse the single expression
        self.consume(TokenType::Eof, "Expect end of expression.".to_string()); // Expect EOF after the expression

        // Emit the final return instruction *after* the expression is compiled
        // and after consuming EOF (if successful).
        // This assumes the expression leaves a value on the stack that should be returned.
        // If there was a parsing error, we might not want to emit return,
        // or handle it differently.
        // Only emit return if no parsing error occurred before EOF.
        if !self.had_error {
            self.emit_return();
        }

        if self.had_error {
            // Error messages were printed by error_* methods.
            Err("Parsing failed.".to_string())
        } else {
            // No parsing errors.
            Ok(std::mem::take(&mut self.compiling_chunk))
        }
    }

    /* ========== 核心解析方法 (Pratt Parser) ========== */

    // Public facing expression parser (often just calls parse_precedence with lowest precedence)
    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment); // Start parsing with the lowest precedence
    }

    // --- Parse Precedence Algorithm ---
    fn parse_precedence(&mut self, precedence: Precedence) {
        // Advance must be called by the rule that *starts* the precedence parse.
        // For the initial call from `expression`, the first `advance` is done in `compile`.
        // For recursive calls (e.g., after an infix operator), `advance` is called
        // by the consuming rule before calling `parse_precedence`.

        // Consume the token that starts the expression at this precedence level.
        // This token is now the 'prefix' token.
        self.advance(); // Make current -> previous

        let prefix_rule = get_rule(self.previous.kind); // Get the rule for the prefix token

        // 1. Handle the prefix part
        if let Some(prefix_fn) = prefix_rule.prefix {
            prefix_fn(self); // Call the prefix parse function
        } else {
            // If no prefix function is defined for this token, it's a syntax error
            self.error("Expect expression.".to_string());
            // Note: We don't return immediately here. We let the rest of the
            // parse_precedence logic run, but subsequent calls will likely
            // also fail or the loop condition will prevent them. The `had_error`
            // flag ensures overall failure.
            return; // Or rely on had_error check later
        }

        // 2. Handle the infix part (loop)
        // Keep parsing infix expressions as long as the *current* token's
        // precedence is greater than or equal to the precedence level we
        // are currently parsing.
        while precedence <= get_rule(self.current.kind).precedence {
            // Get the rule for the current token (which is the potential infix operator)
            let infix_rule = get_rule(self.current.kind);

            // Consume the infix operator (it becomes `self.previous`)
            self.advance(); // Make current -> previous

            // Call the infix parse function for the now `self.previous` token
            if let Some(infix_fn) = infix_rule.infix {
                infix_fn(self); // Call the infix parse function
            } else {
                // This should not happen if the precedence check passed,
                // but as a safeguard.
                break; // Exit the loop
            }
            // The infix function (e.g., binary, and, or, assignment)
            // will recursively call parse_precedence with the appropriate
            // higher precedence for the right-hand operand.
        }

        // Special handling for assignment's right-associativity can be needed.
        // If the precedence we just finished parsing is Assignment or lower,
        // and the *current* token is "=", it's an unexpected assignment
        // that wasn't consumed by the assignment_rule.
        // This check is sometimes placed here or within the assignment_rule.
        // if precedence <= Precedence::Assignment && self.check(TokenType::Equal) {
        //     self.error("Invalid assignment target.".to_string());
        // }
    }

    // --- Specific Parsing Rules (called by wrapper functions) ---
    // Implement these methods to perform the actual parsing for each rule type.
    // They should emit bytecode and call parse_precedence for operands.

    fn number(&mut self) {
        // `self.previous` is the number token.
        // Convert the lexeme to a number value.
        let value = self.previous.lexeme.parse::<f64>().unwrap_or_else(|_| {
            // This parse error should ideally not happen if the scanner is correct,
            // but handle defensively. Report error if needed.
            self.error(format!("Failed to parse number: {}", self.previous.lexeme));
            0.0 // Provide a default value to continue parsing
        });
        // Emit bytecode to push the number constant onto the stack.
        self.emit_constant(Value::Number(value));
    }

    fn grouping(&mut self) {
        // `self.previous` is the opening parenthesis.
        // Recursively parse the expression inside the parentheses.
        self.expression(); // This calls parse_precedence with Assignment (lowest)

        // Expect the closing parenthesis.
        // consume now takes self.scanner implicitly and returns ().
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_string(),
        );
        // If consume fails, it sets had_error. Parsing will attempt to continue.
    }

    fn unary(&mut self) {
        // `self.previous` is the unary operator ('-' or '!').
        let operator = self.previous.clone();

        // Recursively parse the operand. Unary operators have high precedence,
        // so we parse with Unary precedence.
        self.parse_precedence(Precedence::Unary);

        // Emit the unary operation bytecode after the operand's code has been emitted.
        match operator.kind {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            // TokenType::Bang => self.emit_byte(OpCode::Not), // Assuming you have a Not opcode
            _ => self.error(format!("Unexpected unary operator: {}", operator.lexeme)), // Should not happen if table is correct
        }
    }

    fn binary(&mut self) {
        // `self.previous` is the binary operator ('+', '-', '*', '/', '==', '!=', etc.).
        let operator = self.previous.clone();

        // Get the precedence of this operator. The right-hand operand
        // should be parsed with precedence *one level higher* than the operator's
        // own precedence (to ensure correct operator associativity/binding).
        let precedence = get_rule(operator.kind).precedence; // Get the operator's precedence

        // Parse the right-hand operand. Parse with the next higher precedence level.
        // Need a helper function to get the next higher precedence.
        let next_higher_precedence = match precedence {
            Precedence::None => Precedence::Assignment, // Should not happen for infix operators
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None, // Should not happen for infix operators
        };
        self.parse_precedence(next_higher_precedence);

        // Emit the binary operation bytecode after both operands' code.
        match operator.kind {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            // Add cases for comparison operators
            // TokenType::BangEqual => self.emit_bytes(OpCode::Equal, OpCode::Not), // Emit Equal then Not for !=
            // TokenType::EqualEqual => self.emit_byte(OpCode::Equal),
            // TokenType::Greater => self.emit_byte(OpCode::Greater),
            // TokenType::GreaterEqual => self.emit_bytes(OpCode::Less, OpCode::Not), // Emit Less then Not for >=
            // TokenType::Less => self.emit_byte(OpCode::Less),
            // TokenType::LessEqual => self.emit_bytes(OpCode::Greater, OpCode::Not), // Emit Greater then Not for <=
            // Add cases for logical operators (handled differently, see and/or rules)
            _ => self.error(format!("Unexpected binary operator: {}", operator.lexeme)), // Should not happen if table is correct
        }
    }

    // Implement placeholder methods for other rules referenced in the table:
    // fn literal_string(&mut self) { /* ... */ }
    // fn identifier(&mut self) { /* ... */ }
    // fn dot(&mut self) { /* ... */ }
    // fn call(&mut self) { /* ... */ }
    // fn and(&mut self) {
    //    // AND has short-circuiting behavior, requires different bytecode than simple binary ops
    //    // Parse left operand, then emit jump if false, parse right operand, etc.
    //    let end_jump = self.emit_jump(OpCode::JumpIfFalse); // Implement emit_jump
    //    self.emit_byte(OpCode::Pop); // Pop left operand's value if true
    //    self.parse_precedence(Precedence::And); // Parse right operand
    //    self.patch_jump(end_jump); // Implement patch_jump
    // }
    // fn or(&mut self) { /* Similar short-circuiting logic */ }
    // fn assignment(&mut self) { /* ... */ }
    // fn literal_false(&mut self) { self.emit_byte(OpCode::False); }
    // fn literal_nil(&mut self) { self.emit_byte(OpCode::Nil); }
    // fn literal_true(&mut self) { self.emit_byte(OpCode::True); }
    // fn super_rule(&mut self) { /* ... */ } // For 'super' keyword
    // fn self_rule(&mut self) { /* ... */ } // For 'this' keyword (often 'self' in Rust examples)

    /* ========== 发出字节码 (Adjusted) ========== */
    // These methods use self.compiling_chunk
    fn emit_byte(&mut self, op_code: OpCode) {
        // Changed byte to OpCode
        self.compiling_chunk
            .write_chunk(op_code, self.previous.line);
    }
    // end_compilation might be called by the outer compilation loop, not expression
    // fn end_compilation(&mut self) {
    //     self.emit_return();
    // }
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }
    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        // Changed byte1/byte2 to OpCode
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
    fn emit_constant(&mut self, value: Value) {
        let constant_index = self.make_constant(value);
        // OpCode::Constant takes the index as an operand.
        // Need to ensure your OpCode enum handles operands, or emit the index separately.
        // Example assuming OpCode::Constant variant takes usize:
        self.emit_byte(OpCode::Constant(constant_index));
        // The original example emitted OpCode::Return right after constant.
        // This is only correct if the expression is just the constant.
        // In a full expression, only one return is needed at the end of the function/script.
        // So remove `OpCode::Return` here.
        // self.emit_bytes(OpCode::Constant(constant_index), OpCode::Return); // REMOVE RETURN HERE
    }
    fn make_constant(&mut self, value: Value) -> usize {
        self.compiling_chunk.add_constant(value)
    }

    // You'll likely need emit_jump and patch_jump for control flow (if, while, and, or)
    // fn emit_jump(&mut self, opcode: OpCode) -> usize { /* Emit jump instruction, placeholder for offset, return address */ }
    // fn patch_jump(&mut self, offset: usize) { /* Write the correct jump offset at the placeholder */ }

    /* ========== Token 流控制 (Using self.scanner) ========== */
    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            self.current = self.scanner.scan_token(); // Use self.scanner
            if self.current.kind != TokenType::Error {
                break;
            }
            // Report the error token's message, but keep looping to skip it
            self.error_at_current(self.current.lexeme.clone());
            // error_at_current now sets had_error and returns ().
        }
    }

    // consume no longer needs scanner argument as it uses self.scanner
    fn consume(&mut self, kind: TokenType, msg: String) {
        if self.current.kind == kind {
            self.advance(); // Use self.scanner implicitly
        } else {
            // Report the error and set the flag, but do *not* stop parsing here.
            self.error_at_current(msg);
            // The parsing rule calling `consume` might check `had_error`
            // after this call and decide to synchronize.
        }
    }

    // Helper to check current token without consuming
    fn check(&self, kind: TokenType) -> bool {
        self.current.kind == kind
    }

    // Helper to match current token and consume if matches
    // Used for optional tokens or where you don't want to error on mismatch immediately
    fn match_token(&mut self, kind: TokenType) -> bool {
        // No scanner arg needed
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /* ========== 错误恢复 (Using self.scanner) ========== */
    // synchronize now uses self.scanner
    fn synchronize(&mut self) {
        // No scanner arg needed
        // Exit panic mode (if you implement panic_mode flag)
        // self.panic_mode = false;

        // Keep advancing as long as we're not at EOF and haven't found a synchronization point.
        while self.current.kind != TokenType::Eof {
            // If the *previous* token was a semicolon, we just finished a statement. This is a good place to stop skipping.
            if self.previous.kind == TokenType::Semicolon {
                return;
            }

            // Check if the *current* token is a keyword that typically starts a new declaration or statement.
            // If so, we stop *before* consuming this token, so the next parsing rule can consume it.
            match self.current.kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return; // Stop skipping *at* this keyword
                }
                // If it's none of the above, it's likely part of the erroneous code we want to skip.
                _ => {} // Continue the loop
            }

            // Skip the current token and get the next one.
            self.advance(); // Uses self.scanner
        }
        // If we reach EOF, just stop.
    }

    /* ========== 错误报告 (Using had_error) ========== */
    // These methods now set the had_error flag and print, but return nothing.
    fn error_at_current(&mut self, message: String) {
        // Use clone if error_at needs to own the token, otherwise pass reference.
        // Let's pass a reference for efficiency.
        self.error_at(self.current.clone(), message);
    }

    fn error(&mut self, message: String) {
        // Use clone if error_at needs to own the token, otherwise pass reference.
        self.error_at(self.previous.clone(), message);
    }

    // Changed signature to accept a reference to Token
    fn error_at(&mut self, token: Token, message: String) {
        // Don't report if we're already in a state where we know there's an error
        // and haven't synchronized yet. (This is where a `panic_mode` field helps).
        // if self.panic_mode { return; }
        // self.panic_mode = true; // Enter panic mode when error is reported

        eprintln!("[line {}] Error", token.line);
        match token.kind {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error => {} // Error token's lexeme is the error message, already printed in advance's loop
            _ => eprint!(" at '{}'", token.lexeme),
        }
        eprintln!(": {}", message);
        self.had_error = true; // <--- Set the error flag
        // No return value anymore
    }
}
