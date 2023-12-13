pub enum ParseError {
    RonError(ron::error::SpannedError),
    JsonError(serde_json::Error),
    YamlError(serde_yaml::Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::RonError(err) => write!(
                f,
                "Cannot parse the file due to error {} at position {}",
                err.code, err.position
            ),
            ParseError::JsonError(err) => write!(
                f,
                "Cannot parse the file due to error {:?} at {}:{}",
                err.classify(),
                err.line(),
                err.column()
            ),
            ParseError::YamlError(err) => write!(
                f,
                "Cannot parse the file due to error {} at {}:{}",
                err,
                err.location().map_or(0, |loc| loc.line()),
                err.location().map_or(0, |loc| loc.column())
            ),
        }
    }
}

impl From<serde_yaml::Error> for ParseError {
    fn from(err: serde_yaml::Error) -> Self {
        ParseError::YamlError(err)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(err: serde_json::Error) -> Self {
        ParseError::JsonError(err)
    }
}

impl From<ron::error::SpannedError> for ParseError {
    fn from(err: ron::error::SpannedError) -> Self {
        ParseError::RonError(err)
    }
}
