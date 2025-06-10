//! Built-in schema type recognition.

use crate::{
    ty_resolver::{CtorArg, CtorSig, TypeData, TypeResolver},
    tysys::{Ty, TyExpr},
    Identifier,
};

/// Populates a type resolver with the builtin types.
pub(crate) fn populate_builtin_types(resolv: &mut TypeResolver) {
    // Basic types.
    insert_ty(resolv, "boolean");
    for i in [8, 16, 32, 64, 128, 256] {
        insert_ty(resolv, &format!("uint{i}"));
    }

    // Composite types.
    insert_ty(resolv, "Container");
    insert_ty_ctor(
        resolv,
        "Vector",
        CtorSig::Fixed(vec![CtorArg::Ty, CtorArg::Int]),
    );
    insert_ty_ctor(resolv, "Bitvector", CtorSig::Fixed(vec![CtorArg::Int]));
    insert_ty_ctor(
        resolv,
        "List",
        CtorSig::Fixed(vec![CtorArg::Ty, CtorArg::Int]),
    );
    insert_ty_ctor(resolv, "Bitlist", CtorSig::Fixed(vec![CtorArg::Int]));
    insert_ty_ctor(resolv, "Union", CtorSig::VariableTy);

    // Composite types from other specs.
    insert_ty_ctor(
        resolv,
        "StableContainer",
        CtorSig::Fixed(vec![CtorArg::Int]),
    );
    insert_ty_ctor(resolv, "Profile", CtorSig::Fixed(vec![CtorArg::Ty]));
    insert_ty_ctor(resolv, "Optional", CtorSig::Fixed(vec![CtorArg::Ty]));

    // Aliases.
    insert_alias_simple(resolv, "bit", "boolean");
    insert_alias_simple(resolv, "byte", "uint8");

    let vector_ident = make_ident("Vector");
    let byte_ident = make_ident("uint8"); // really "byte", but this is what that resolves to
    for i in 1..64 {
        let byte_te = TyExpr::new_simple(byte_ident.clone());
        let len_te = TyExpr::new_int(i);
        let complex = Ty::Complex(vector_ident.clone(), vec![byte_te, len_te]);
        insert_alias(resolv, &format!("Bytes{i}"), complex);
    }

    // FIXME this is wrong, but works, handle it at a higher level
    insert_alias_simple(resolv, "null", "boolean");
}

fn make_ident(s: &str) -> Identifier {
    Identifier::try_from(s).expect("builtins: parse ident")
}

fn insert_ty(resolv: &mut TypeResolver, name: &str) {
    resolv
        .insert_type(make_ident(name), TypeData {})
        .expect("builtins: decl builtin type");
}

fn insert_ty_ctor(resolv: &mut TypeResolver, name: &str, sig: CtorSig) {
    resolv
        .insert_type_ctor(make_ident(name), sig)
        .expect("builtins: decl builtin type ctor");
}

fn insert_alias(resolv: &mut TypeResolver, name: &str, ty: Ty) {
    resolv
        .decl_type_alias(make_ident(name), ty)
        .expect("builtins: decl builtin type alias");
}

fn insert_alias_simple(resolv: &mut TypeResolver, name: &str, target: &str) {
    insert_alias(resolv, name, Ty::Simple(make_ident(target)))
}
