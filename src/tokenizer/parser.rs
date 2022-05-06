use nom::IResult;

use crate::tokenizer::{
    small_parsers::{
        parse_left_expression, parse_right_expression, parse_right_expression_no_parenthesis,
        parse_right_expression_with_comma, parse_unit,
    },
    Operation, Token,
};

use super::small_parsers::trim_with_comments;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(usize),
    MissingRightParenthesis(usize),
    MissingArgument,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenizerState {
    LeftExpression,
    RightExpression,
}

#[derive(Debug, Copy, Clone)]
enum ParenthesisState {
    Subexpression,
    Function,
}

#[derive(Debug, Clone)]
pub struct TokenizedString {
    pub tokens: Vec<Token>,
}

impl TokenizedString {
    pub fn new_from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn try_new(input: &str) -> Result<TokenizedString, ParseError> {
        let mut result: Vec<Token> = vec![];
        let mut parenthesis_stack: Vec<ParenthesisState> = vec![];
        let mut state = TokenizerState::LeftExpression;
        let mut next_is_unit = true;
        let mut work_string = input;

        let mut last_string;
        let mut last_state;

        while !work_string.is_empty() {
            if let Ok(trimmed) = trim_with_comments::<nom::error::Error<_>>(work_string) {
                work_string = trimmed.0;
                if work_string.is_empty() {
                    break;
                }
            }

            let mut parsing_result: IResult<&str, Token> = match (&state, parenthesis_stack.last())
            {
                (TokenizerState::LeftExpression, _) => parse_left_expression(work_string),
                (TokenizerState::RightExpression, None) => {
                    parse_right_expression_no_parenthesis(work_string)
                }
                (TokenizerState::RightExpression, Some(&ParenthesisState::Function)) => {
                    parse_right_expression_with_comma(work_string)
                }
                (TokenizerState::RightExpression, Some(&ParenthesisState::Subexpression)) => {
                    parse_right_expression(work_string)
                }
            };

            last_state = state;

            match &mut parsing_result {
                Ok((rest, token)) => {
                    if next_is_unit {
                        next_is_unit = false;

                        match token {
                            Token::Identifier {
                                name,
                                could_be_unit: _,
                            } => {
                                *token = Token::Identifier {
                                    name: name.clone(),
                                    could_be_unit: true,
                                };
                            }
                            Token::LeftParenthesis | Token::RightParenthesis => {
                                next_is_unit = true;
                            }
                            _ => (), //println!("next_is_unit is set, but token is {:?}", token),
                        };
                    }

                    // println!("Token: {:?}\n", token);
                    match token {
                        Token::Binary(_) | Token::Comma => state = TokenizerState::LeftExpression,
                        Token::LeftParenthesis => {
                            parenthesis_stack.push(ParenthesisState::Subexpression)
                        }
                        Token::RightParenthesis => {
                            parenthesis_stack.pop().expect("Missing left parenthesis");
                        }
                        Token::Identifier { .. } | Token::Number(..) => {
                            state = TokenizerState::RightExpression
                        }
                        Token::Function(..) => parenthesis_stack.push(ParenthesisState::Function),
                        _ => (),
                    }

                    result.push(token.clone());
                    last_string = work_string;
                    work_string = rest;

                    if last_state == TokenizerState::RightExpression
                        && parse_unit(last_string).is_ok()
                    {
                        // println!("Overriding unit in identifier");
                        next_is_unit = true;
                    }
                }
                Err(nom::Err::Error(_)) => {
                    // TODO: handle error
                    // println!("Normal: {}\nLast: {}", work_string, last_string);
                    if let Some(last_token) = result.last() {
                        state = TokenizerState::LeftExpression;
                        next_is_unit = true;
                        match last_token {
                            Token::Number(_) | Token::RightParenthesis => {
                                result.push(Token::Binary(Operation::Multiply));
                                continue;
                            }
                            _ => (),
                        };
                    }
                    return Err(ParseError::UnexpectedToken(1));
                }
                Err(error) => panic!("{}", error),
            }
        }

        match state {
            TokenizerState::LeftExpression => Err(ParseError::MissingArgument),
            _ if !parenthesis_stack.is_empty() => {
                Err(ParseError::MissingRightParenthesis(parenthesis_stack.len()))
            }
            _ => Ok(TokenizedString { tokens: result }),
        }
    }
}
