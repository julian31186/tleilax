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

                    if stack.len() % 2 != 0 {
                        return Err(YamlError::InvalidIndentation);
                    }

                    current_indentation = stack.len() / 2;

                    if current_indentation > parent_indentation && current_indentation - parent_indentation > 1 {
                        return Err(YamlError::InvalidIndentation);
                    }

                    let mut has_dedent = false;
                    if current_indentation < parent_indentation {
                        for _ in 0..parent_indentation - current_indentation {
                            res.push(Token::Dedent);
                            has_dedent = true;
                        }
                    }

                    if !has_dedent && current_indentation > parent_indentation {
                        res.push(Token::Indent);
                    }

                    parent_indentation = current_indentation;

                    if !after_colon {
                        key.push(ch);
                    } else {
                        value.push(ch);
                    }
                },
            }
        }
        is_first_line = false;

        if has_colon && key.len() == 0 {
            return Err(YamlError::ColonWithNoKey);
        }
        
        if key.len() > 0 && !has_colon {
            return Err(YamlError::KeyWithNoColon);
        }
        if value.len() > 0 {
            res.push(Token::Identifier(value));
        }
    }

    Ok(res)
}
