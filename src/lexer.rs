use crate::types::Token;
use crate::errors::YamlError;

pub fn lex(input: &str) -> Result<Vec<Token>, YamlError> {
    let mut res: Vec<Token> = vec![];
    let mut is_first_line = true;
    let mut parent_indentation = 0;
    // When a line ends with a colon and no inline value, the next lines
    // at exactly one deeper indentation are allowed to start with '-'.
    // Track a stack so nested blocks unwind and siblings at parent depth continue to work.
    let mut expected_list_indent_stack: Vec<usize> = Vec::new();

    for line in input.lines() {

        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let mut started = false;
        let mut after_colon = false;

        let mut seen_non_space_after_colon = false;

        let mut pushed_indent = false;

        let mut key = String::new();
        let mut value = String::new();

        let mut has_colon = false;

        let mut stack: Vec<char> = vec![];

        // Track if this line begins a block list item ("- ")
        let mut saw_list_item_on_this_line = false;
        
        while let Some(ch) = chars.next() {
            match ch {
                '#' => {
                    break;
                },
                '\t' => {
                    return Err(YamlError::InvalidIndentCharacter(ch))
                },
                
                ':' => {
                    if seen_non_space_after_colon {
                        value.push(ch);
                        continue
                    }
                    res.push(Token::Identifier(key.clone()));
                    res.push(Token::Colon);
                    after_colon = true;
                    has_colon = true;
                },
                ' ' => {
                    if seen_non_space_after_colon {
                        value.push(ch);
                        continue;
                    }
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
                '-' => {
                    // Treat '-' as a list item indicator only when it's the first
                    // non-space character on the line (block list) AND we are
                    // currently in a context expecting a block (from a prior 'key:').
                    let is_block_list_element = !started && !after_colon;

                    if is_block_list_element {
                        if stack.len() % 2 != 0 {
                            return Err(YamlError::InvalidIndentation);
                        }

                        let current_indentation = stack.len() / 2;

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

                        // Pop any expected list indents deeper than current
                        while let Some(&top) = expected_list_indent_stack.last() {
                            if current_indentation < top {
                                expected_list_indent_stack.pop();
                            } else {
                                break;
                            }
                        }

                        parent_indentation = current_indentation;

                        stack.clear();

                        pushed_indent = true;

                        // Enforce: list items must appear only after a colon,
                        // on a new line at exactly one deeper indentation level.
                        let mut is_valid_list_depth = false;
                        if let Some(&expected_indent) = expected_list_indent_stack.last() {
                            if expected_indent == current_indentation {
                                is_valid_list_depth = true;
                            }
                        }
                        if is_valid_list_depth {
                            started = true;
                            saw_list_item_on_this_line = true;
                            res.push(Token::ListItem);
                        } else {
                            return Err(YamlError::InvalidListItemPosition);
                        }
                    } else {
                        // Regular '-' character (e.g., in a key name or within a value)
                        if after_colon {
                            value.push(ch);
                        } else {
                            key.push(ch);
                        }
                    }
                }
                _ => {
                    started = true;

                    if after_colon {
                        seen_non_space_after_colon = true;
                    }

                    if !pushed_indent {                        
                        if stack.len() % 2 != 0 {
                            return Err(YamlError::InvalidIndentation);
                        }

                        let current_indentation = stack.len() / 2;

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

                        // Pop any expected list indents deeper than current
                        while let Some(&top) = expected_list_indent_stack.last() {
                            if current_indentation < top {
                                expected_list_indent_stack.pop();
                            } else {
                                break;
                            }
                        }

                        parent_indentation = current_indentation;

                        stack.clear();

                        pushed_indent = true;
                    }

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
            // Allow scalar list items like "- value" only when they follow a colon
            // from the previous line (i.e., within an expected list block).
            if saw_list_item_on_this_line {
                res.push(Token::Identifier(key));
            } else {
                return Err(YamlError::KeyWithNoColon);
            }
        }
        let had_inline_value = !value.is_empty();
        if had_inline_value {
            value = value.trim().to_string();
            res.push(Token::Identifier(value));
        }

        // Update expectation for list blocks based on whether this line ended with a colon
        // and no inline value. Push parent_indent+1 so siblings work after nested blocks.
        if has_colon && !had_inline_value {
            expected_list_indent_stack.push(parent_indentation + 1);
        }
    }

    Ok(res)
}
