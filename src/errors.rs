#[derive(Debug, Clone)]
pub enum YamlError {
   InvalidIndentCharacter(char),
   FirstLineNotZeroIndentation,
   ColonWithNoKey,
   InvalidIndentation,
   KeyWithNoColon,
   InvalidListItemPosition,
}

impl std::fmt::Display for YamlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YamlError::InvalidIndentCharacter(ch) => {
                write!(f, "Invalid indentation character: '{}'", ch)
            },
            YamlError::FirstLineNotZeroIndentation => {
                write!(f, "First line does not have zero indentation")
            },
            YamlError::ColonWithNoKey => {
                write!(f, "Colon does not have an associated key")
            },
            YamlError::InvalidIndentation => {
                write!(f, "Invalid indentation")
            },
            YamlError::KeyWithNoColon => {
                write!(f, "This key does not have a colon")
            },
            YamlError::InvalidListItemPosition => {
                write!(f, "Block list items must appear on a new indented line after a colon")
            },
        }
    }
}

impl std::error::Error for YamlError {}
