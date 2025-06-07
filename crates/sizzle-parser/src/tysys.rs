//! Core type system expressions.

use crate::Identifier;

/// A type expression is something that we resolve from an identifier and are
/// valid as type parameters.  These aren't always necessarily types, but are
/// type-related expressions.
#[derive(Clone, Debug)]
pub enum TyExpr {
    /// A type.
    Ty(Ty),

    /// A value, possibly resolved from a const.
    ///
    /// This isn't a normal type, but it's valid and we can resolve to it from
    /// things that look like types and then would proceed to error from that.
    Int(ConstValue),
}

impl TyExpr {
    pub fn new_simple(ident: Identifier) -> Self {
        Self::Ty(Ty::Simple(ident))
    }

    pub fn new_int(v: u64) -> Self {
        Self::Int(ConstValue::Int(v))
    }

    pub fn iter_idents(&self) -> impl Iterator<Item = &Identifier> {
        // FIXME I couldn't figure out how to make this no-alloc.
        let idents = match self {
            TyExpr::Ty(t) => t.iter_idents().collect::<Vec<_>>(),
            TyExpr::Int(_) => Vec::new(),
        };

        idents.into_iter()
    }
}

/// A type expression.
///
/// This can be used either to indicate the class's parent's type or a field's
/// type.
#[derive(Clone, Debug)]
pub enum Ty {
    /// A simple type without arguments.
    Simple(Identifier),

    /// A complex type with arguments (possibly zero, like in an empty `Union`).
    Complex(Identifier, Vec<TyExpr>),
}

impl Ty {
    pub fn base_name(&self) -> &Identifier {
        match self {
            Ty::Simple(name) => name,
            Ty::Complex(name, _) => name,
        }
    }

    pub fn iter_idents(&self) -> impl Iterator<Item = &Identifier> {
        let bn = self.base_name();

        let ext = match self {
            Ty::Simple(_) => &[],
            Ty::Complex(_, ch) => ch.as_slice(),
        };

        std::iter::once(bn).chain(ext.iter().flat_map(|e| e.iter_idents()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstValue {
    /// Literal integer values.
    Int(u64),

    /// An integer value shifted to the left by another integer value.  This is
    /// useful for very large numbers like 2**256 - 1.
    Binop(Binop, u64, u64),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Binop {
    /// Addition.
    Add,

    /// Multiplication.
    Mul,

    /// Shift left.
    Shl,
}

impl ConstValue {
    pub fn eval(&self) -> u64 {
        match self {
            ConstValue::Int(v) => *v,
            ConstValue::Binop(op, a, b) => {
                match op {
                    Binop::Add => a + b,
                    Binop::Mul => a * b,
                    Binop::Shl => a << b,
                }
            }
        }
    }
}
