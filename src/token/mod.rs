use value::{FloatType, IntType};

#[derive(Clone, PartialEq)]
pub enum Token {
    // Single character tokens
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    LBrace,
    RBrace,
    Whitespace,

    // Complex tokens
    Identifier(String),
    Float(FloatType),
    Int(IntType),
    Boolean(bool),
}

enum PartialToken {
    Token(Token),
    Literal(String),
}

// Make this a const fn as soon as match gets stable (issue #57563)
fn char_to_token(c: char) -> PartialToken {
    match c {
        '+' => PartialToken::Token(Token::Plus),
        '-' => PartialToken::Token(Token::Minus),
        '*' => PartialToken::Token(Token::Star),
        '/' => PartialToken::Token(Token::Slash),
        '%' => PartialToken::Token(Token::Percent),
        '(' => PartialToken::Token(Token::LBrace),
        ')' => PartialToken::Token(Token::RBrace),
        c => {
            if c.is_whitespace() {
                PartialToken::Token(Token::Whitespace)
            } else {
                PartialToken::Literal(c.to_string())
            }
        }
    }
}

impl Token {
    // Make this a const fn as soon as match gets stable (issue #57563)
    pub fn is_value(&self) -> bool {
        match self {
            Token::Plus => false,
            Token::Minus => false,
            Token::Star => false,
            Token::Slash => false,
            Token::Percent => false,
            Token::LBrace => false,
            Token::RBrace => true,
            Token::Whitespace => false,
            Token::Identifier(_) => true,
            Token::Float(_) => true,
            Token::Int(_) => true,
            Token::Boolean(_) => true,
        }
    }
}

/// Converts a string to a vector of partial tokens.
fn str_to_tokens(string: &str) -> Vec<PartialToken> {
    let mut result = Vec::new();
    for c in string.chars() {
        let partial_token = char_to_token(c);

        let if_let_successful =
            if let (Some(PartialToken::Literal(last)), PartialToken::Literal(literal)) =
                (result.last_mut(), &partial_token)
            {
                last.push_str(literal);
                true
            } else {
                false
            };

        if !if_let_successful {
            result.push(partial_token);
        }
    }
    result
}

/// Resolves all literals in the given vector of partial tokens by converting them to complex tokens.
fn resolve_literals(tokens: &Vec<PartialToken>) -> Vec<Token> {
    tokens
        .iter()
        .map(|token| match token {
            PartialToken::Token(token) => token.clone(),
            PartialToken::Literal(literal) => {
                if let Ok(number) = literal.parse::<IntType>() {
                    Token::Int(number)
                } else if let Ok(number) = literal.parse::<FloatType>() {
                    Token::Float(number)
                } else if let Ok(boolean) = literal.parse::<bool>() {
                    Token::Boolean(boolean)
                } else {
                    Token::Identifier(literal.to_string())
                }
            }
        })
        .collect()
}

pub fn tokenize(string: &str) -> Vec<Token> {
    resolve_literals(&str_to_tokens(string))
}
