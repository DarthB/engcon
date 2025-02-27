use syn::DeriveInput;

use super::PartialAST;

pub(super) fn filter_ast(input: DeriveInput) -> Result<PartialAST, syn::Error> {
    let self_type = input.ident;

    let struct_: syn::DataStruct = match input.data {
        syn::Data::Struct(data) => data,
        _ => {
            return Err(syn::Error::new_spanned(
                self_type,
                "Validator Macros only work on structs yet",
            ));
        }
    };

    // filter fields that have validated rule
    let validated_fields: Vec<syn::Field> = struct_
        .fields
        .iter()
        .filter(|e| {
            e.attrs
                .iter()
                .any(|e| e.meta.path().is_ident("validate_value"))
        })
        .cloned()
        .collect();

    if validated_fields.is_empty() {
        Err(syn::Error::new_spanned(
            self_type,
            "Use at least one validate_value() attribute helper".to_owned(),
        ))
    } else {
        Ok(PartialAST {
            self_type,
            validated_fields,
        })
    }
}
