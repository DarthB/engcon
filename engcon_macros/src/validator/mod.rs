use syn::{parse::Parse, DeriveInput, Type};
use validator_analyze::analyze_ast;
use validator_codegen::codegen;
use validator_filter::filter_ast;
use validator_intermediate::intermediate_code;

use crate::helper::*;

mod validator_analyze;
mod validator_codegen;
mod validator_filter;
mod validator_intermediate;

pub(super) fn validator_macro_wrapper(input: DeriveInput) -> MacroResult {
    let ast = filter_ast(input).map_err(synerr_to_tokens)?;
    let ast = analyze_ast(ast).map_err(synerr_to_tokens)?;
    let ic = intermediate_code(ast).map_err(synerr_to_tokens)?;

    Ok(codegen(ic))
}

#[derive(Debug)]
struct PartialAST {
    self_type: syn::Ident,

    validated_fields: Vec<syn::Field>,
}

#[derive(Debug)]
struct ValidationRule {
    left: syn::Ident,

    cmp_op: syn::BinOp,

    rigth: syn::Expr,

    right_is_field_on_self: bool,
}

#[derive(Debug)]
struct FieldInfo {
    field_name: syn::Ident,

    ty: Type,

    rules: Vec<ValidationRule>,
}

#[derive(Debug)]
struct IntermediateCode {
    self_type: syn::Ident,

    field_infos: Vec<FieldInfo>,
}

impl Parse for ValidationRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut candidate = ValidationRule {
            left: input.parse()?,
            cmp_op: input.parse()?,
            rigth: input.parse()?,
            right_is_field_on_self: false,
        };

        // todo: be more strict about phases
        if candidate.left != "x" {
            // a rule starts with an `x` representing the field below
            return Err(syn::Error::new_spanned(
                candidate.left,
                "You are required to call this identifier 'x'",
            ));
        }

        if !matches!(
            candidate.cmp_op,
            syn::BinOp::Ge(_) | syn::BinOp::Gt(_) | syn::BinOp::Le(_) | syn::BinOp::Lt(_)
        ) {
            // operator must be `<`, `<=`, `>=` or `>`
            return Err(syn::Error::new_spanned(
                candidate.cmp_op,
                "Only <, <=, >= and > are supported operators",
            ));
        }

        // if we use a variable name in the contract, we need to preceed it with self.
        let was_adapted = adapt_ident_with_self(&mut candidate.rigth);
        if was_adapted {
            candidate.right_is_field_on_self = true;
        }

        Ok(candidate)
    }
}
