use crate::types::Token;
use crate::errors::YamlError;

pub fn lex(input: &str) -> Result<Vec<Token>, YamlError> {
    let mut res: Vec<Token> = vec![];
    let mut is_first_line = true;
    let mut parent_indentation = 0;

    for line in input.lines() {

        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let mut started = false;
        let mut after_colon = false;

        let mut current_indentation = 0;

        let mut key = String::new();
        let mut value = String::new();

        let mut has_colon = false;

        let mut stack: Vec<char> = vec![];
        
        while let Some(ch) = chars.next() {
            match ch {
                '#' => {
                    break;
                },
                '\t' => {
                    return Err(YamlError::InvalidIndentCharacter(ch))
                },
                
                ':' => {
                    res.push(Token::Identifier(key.clone()));
                    res.push(Token::Colon);
                    after_colon = true;
                    has_colon = true;
                },
                ' ' => {
                    if is_first_line && !started {
                        return Err(YamlError::FirstLineNotZeroIndentation);
                    } else {
                        if started {
                            continue;
                        } else {
                            stack.push(' ');
                        }
                    }
                },
                _ => {
                    started = true;

                    if stack.len() > 0 {
                        
                        if stack.len() % 2 != 0 {
                            return Err(YamlError::InvalidIndentation);
                        }

                        for _ in 0..stack.len() / 2 {
                            current_indentation += 1;
                            res.push(Token::Indent);
                        }

                        if current_indentation > parent_indentation && current_indentation - parent_indentation > 1 {
                            return Err(YamlError::InvalidIndentation);
                        }

                        if current_indentation < parent_indentation {
                            for _ in 0..parent_indentation - current_indentation {
                                res.push(Token::Dedent);
                            }
                        }

                        stack.clear();
                    }

                    if !after_colon {
                        key.push(ch);
                    } else {
                        value.push(ch);
                    }                    
                },
            }
        }
        parent_indentation = current_indentation;
        is_first_line = false;
        if key.len() > 0 && value.len() == 0 {
            if !has_colon {
                return Err(YamlError::KeyWithNoColon);
            }
            return Err(YamlError::KeyWithNoValue);
        }
        if value.len() > 0 {
            res.push(Token::Identifier(value));
        }
    }

    Ok(res)
}
