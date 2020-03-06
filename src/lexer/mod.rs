pub mod lex_token;
use lex_token::LexToken;

pub fn lex(code: &str) -> Option<Vec<LexToken>> {
    let mut tokens: Vec<LexToken> = Vec::new();

    let mut iter = code.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            ';' => tokens.push(LexToken::SemiColon),
            ':' => tokens.push(LexToken::Colon),

            '(' => tokens.push(LexToken::ParanthesisOpen),
            ')' => tokens.push(LexToken::ParanthesisClose),

            '[' => tokens.push(LexToken::BracketOpen),
            ']' => tokens.push(LexToken::BracketClose),

            '{' => tokens.push(LexToken::BraceOpen),
            '}' => tokens.push(LexToken::BraceClose),

            '-' =>  {
                if let Some(n) = iter.peek() {
                    match n {
                        '-' => {
                            tokens.push(LexToken::MinusMinus);
                            iter.next();
                        },
                        '>' => {
                            tokens.push(LexToken::Arrow);
                            iter.next();
                        },
                        '=' => { 
                            tokens.push(LexToken::MinusEquals);
                            iter.next();
                        },
                        _ => tokens.push(LexToken::Minus),
                    }
                } else {
                    tokens.push(LexToken::Minus);
                } 
            },
            
            '+' => {
                if let Some(n) = iter.peek() {
                   match n {
                        '+' => { 
                            tokens.push(LexToken::PlusPlus);
                            iter.next();
                        },
                        '=' => { 
                            tokens.push(LexToken::PlusEquals);
                            iter.next();
                        },
                        _ => tokens.push(LexToken::Plus),
                   } 
                } else {
                    tokens.push(LexToken::Plus);
                }
            },

            '*' => {
                if let Some(n) = iter.peek() {
                    match n {
                   '=' => {
                            tokens.push(LexToken::StarEquals);
                            iter.next();
                        },
                        _ => tokens.push(LexToken::Star),
                    }
                } else {
                    tokens.push(LexToken::Star);
                }
            },

            '/' => {
                if let Some(n) = iter.peek() {
                    match n {
                        '=' => {
                            tokens.push(LexToken::SlashEquals);
                            iter.next();
                        },
                        '/' => {
                            while let Some(n) = iter.peek() {
                                if n == &'\n' {
                                    iter.next();
                                    break;
                                }
                                iter.next();
                            }
                        },
                        '*' => {
                            while let Some(n) = iter.next() {
                                if n == '*' {
                                    if let Some(e) = iter.peek() {
                                        if e == &'/' {
                                            iter.next();
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        _ => tokens.push(LexToken::Slash),
                    }
                } else {
                    tokens.push(LexToken::Slash);
                } 
            },

            '%' => {
                if let Some(n) = iter.peek() {
                    match n {
                        '=' => {
                            tokens.push(LexToken::PercentEquals);
                            iter.next();
                        },
                        _ => tokens.push(LexToken::Percent),
                    }
                } else {
                    tokens.push(LexToken::Percent);
                }
            },

            '>' => {
                if let Some(n) = iter.peek() {
                    match n {
                        '=' => {
                            tokens.push(LexToken::GreatherThenOrEqual);
                        },
                        _ => tokens.push(LexToken::GreatherThen),
                    }
                } else {
                    tokens.push(LexToken::GreatherThen);
                }
            },

            '<' => {
                if let Some(n) = iter.peek() {
                    match n {
                        '=' => {
                            tokens.push(LexToken::SmallerThenOrEuqal);
                        },
                        _ => tokens.push(LexToken::SmallerThen),
                    }
                } else {
                    tokens.push(LexToken::SmallerThen);
                }
            },

            '=' => {
                if let Some(n) = iter.peek() {
                    match n {
                        '=' => {
                            tokens.push(LexToken::EqualsEquals);
                        },
                        _ => tokens.push(LexToken::Equals),
                    }
                } else {
                    tokens.push(LexToken::Equals);
                }
            },

            x if x.is_alphabetic() => {
                let mut ident = String::new();
                ident.push(x);
                while let Some(n) = iter.peek() {
                    if !(n.is_alphabetic() || n.is_numeric()) {
                            break;
                    }
                    ident.push(iter.next().unwrap());
                }

                match &ident[..] {
                    "i8" => tokens.push(LexToken::I8),
                    "i16" => tokens.push(LexToken::I16),
                    "i32" => tokens.push(LexToken::I32),
                    "i64" => tokens.push(LexToken::I64),
                    
                    "u8" => tokens.push(LexToken::U8),
                    "u16" => tokens.push(LexToken::U16),
                    "u32" => tokens.push(LexToken::U32),
                    "u64" => tokens.push(LexToken::U64),

                    "f32" => tokens.push(LexToken::F32),
                    "f64" => tokens.push(LexToken::F64),
                    
                    "bool" => tokens.push(LexToken::Boolean),

                    "true" => tokens.push(LexToken::True),
                    "false" => tokens.push(LexToken::False),

                    "char" => tokens.push(LexToken::Character),
                    
                    "let" => tokens.push(LexToken::Let),
                    "if" => tokens.push(LexToken::If),
                    "else" => tokens.push(LexToken::Else),
                    "for" => tokens.push(LexToken::For),
                    "return" => tokens.push(LexToken::Return),
                    x => tokens.push(LexToken::Identifire(x.into())),
                }
            }
            _ => return None,
        }
    };

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lex(input: &str, expected_output: Vec<LexToken>) {
        if let Some(output) = lex(input) {
            let output = &output;
            if expected_output.len() != output.len() {
                assert!(
                    false,
                    "did not match expected output:\nexpected: {:?}\noutput: {:?}",
                    expected_output,
                    output
                );
            }

            println!("expected: {:?}\noutput: {:?}", expected_output, output);

            assert!(
                expected_output.iter().zip(output).all(|(e, o)| e == o), 
                "test failed lex({:})did not match expected output:\nexpected: {:?}\noutput: {:?}",
                input,
                expected_output,
                output
            );
        }
    }

    #[test]
    fn test_lex_invalid_char_1() {
        assert_eq!(lex("let a = §§"), None);
    }

    #[test]
    fn test_lex_invalid_char_2() {
        assert_eq!(lex("let foo$bar = () => { return true}"), None);
    }

    #[test]
    fn test_lex_single_token_1() {
        test_lex("=", vec![LexToken::Equals]);
    }

    #[test]
    fn test_lex_single_token_2() {
        test_lex("->", vec![LexToken::Arrow]);
    }

    #[test]
    fn test_lex_variable_definition_1() {
        test_lex("let foo: bool = false;", vec![
            LexToken::Let,
            LexToken::Identifire(String::from("foo")),
            LexToken::Colon,
            LexToken::Boolean,
            LexToken::Equals,
            LexToken::False,
            LexToken::SemiColon,
        ]);
    }

    #[test]
    fn test_lex_variable_definition_2() {
        test_lex("let foo = false;", vec![
            LexToken::Let,
            LexToken::Identifire(String::from("foo")),
            LexToken::Equals,
            LexToken::False,
            LexToken::SemiColon,
        ]);
    }

    #[test]
    fn test_lex_function_definition_1() {
        test_lex("let foo = () -> {};", vec![
            LexToken::Let,
            LexToken::Identifire(String::from("foo")),
            LexToken::Equals,
            LexToken::ParanthesisOpen,
            LexToken::ParanthesisClose,
            LexToken::Arrow,
            LexToken::BraceOpen,
            LexToken::BraceClose,
            LexToken::SemiColon,
        ]);
    }

    #[test]
    fn test_lex_function_definition_2() {
        test_lex("let foo: () -> bool = () { return true };", vec![
           LexToken::Let,
           LexToken::Identifire(String::from("foo")),
           LexToken::Colon,
           LexToken::ParanthesisOpen,
           LexToken::ParanthesisClose,
           LexToken::Arrow,
           LexToken::Boolean,
           LexToken::Equals,
           LexToken::ParanthesisOpen,
           LexToken::ParanthesisClose,
           LexToken::BraceOpen,
           LexToken::Return,
           LexToken::True,
           LexToken::BraceClose,
           LexToken::SemiColon,
        ]);
    }

    #[test]
    fn test_lex_comment_1() {
        test_lex("// hello world\n", vec![]);
    }

    #[test]
    fn test_lex_comment_2() {
        test_lex("// hello world\n ==", vec![LexToken::EqualsEquals]);
    }

    #[test]
    fn test_lex_comment_3() {
        test_lex("/* this is a comment */ let this_is_not = false;", vec![
            LexToken::Let,
            LexToken::Identifire(String::from("this_is_not")),
            LexToken::Equals,
            LexToken::False,
            LexToken::SemiColon,
        ]);
    }

    #[test]
    fn test_lex_comment_4() {
        test_lex("/* this text should be ignored */", vec![]);
    }
}
