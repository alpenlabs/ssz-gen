//! Types relating to type and constant names.

use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum NameError {
    #[error("tried to parse empty string")]
    EmptyString,

    #[error("invalid identifier '{0}'")]
    InvalidIdentifier(String),

    #[error("invalid type name '{0}'")]
    InvalidTypeName(String),

    #[error("invalid const name '{0}'")]
    InvalidConstName(String),

    #[error("invalid field name '{0}'")]
    InvalidFieldName(String),
}

/// Identifiers are generic blobs of textual-ish that are not keywords.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Identifier(pub String);

impl TryFrom<String> for Identifier {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_str(
            &value,
            is_valid_ident_initial_char,
            is_valid_ident_continuing_char,
            NameError::InvalidIdentifier,
        )?;
        Ok(Self(value))
    }
}

impl TryFrom<&'_ str> for Identifier {
    type Error = NameError;

    fn try_from(value: &'_ str) -> Result<Self, Self::Error> {
        check_str(
            value,
            is_valid_ident_initial_char,
            is_valid_ident_continuing_char,
            NameError::InvalidIdentifier,
        )?;
        Ok(Self(value.to_string()))
    }
}

pub(crate) fn is_valid_ident_initial_char(c: char) -> bool {
    c.is_alphabetic()
}

pub(crate) fn is_valid_ident_continuing_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// A type name.
///
/// Type names are always UpperCamelCase.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TypeName(pub String);

impl TryFrom<String> for TypeName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_str(
            &value,
            is_valid_ty_name_char,
            is_valid_ty_name_char,
            NameError::InvalidTypeName,
        )?;
        Ok(TypeName(value))
    }
}

fn is_valid_ty_name_char(c: char) -> bool {
    c.is_alphabetic()
}

/// A const name.
///
/// Const names are always SCREAMING_SNAKE_CASE.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ConstName(pub String);

impl TryFrom<String> for ConstName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_str(
            &value,
            is_valid_const_name_char,
            is_valid_const_name_char,
            NameError::InvalidConstName,
        )?;
        Ok(ConstName(value))
    }
}

fn is_valid_const_name_char(c: char) -> bool {
    c == '_' || c.is_ascii_uppercase()
}

/// A field name.
///
/// Field names are always snake_case.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldName(pub String);

impl TryFrom<String> for FieldName {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        check_str(
            &value,
            is_valid_field_name_char,
            is_valid_field_name_char,
            NameError::InvalidFieldName,
        )?;
        Ok(FieldName(value))
    }
}

fn is_valid_field_name_char(c: char) -> bool {
    c == '_' || c.is_ascii_lowercase()
}

fn check_str(
    s: &str,
    initial: impl Fn(char) -> bool,
    rest: impl Fn(char) -> bool,
    make_err: impl Fn(String) -> NameError,
) -> Result<(), NameError> {
    let mut chars = s.chars();
    let first = chars.next().ok_or(NameError::EmptyString)?;

    if !initial(first) {
        return Err(make_err(s.to_owned()));
    }

    if !chars.all(rest) {
        return Err(make_err(s.to_owned()));
    }

    Ok(())
}
