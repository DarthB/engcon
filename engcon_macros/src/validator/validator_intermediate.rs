use syn::{punctuated::Punctuated, Attribute, Token};

use super::{FieldInfo, IntermediateCode, PartialAST, ValidationRule};

pub(super) fn intermediate_code(ast: PartialAST) -> Result<IntermediateCode, syn::Error> {
    let self_type = ast.self_type;

    let mut field_infos = vec![];
    let mut error: Option<syn::Error> = None;
    for field in &ast.validated_fields {
        // we know there is only validated_value attributes left:
        // we know they are meta lists with commas:

        let mut rules = vec![];
        for attr in &field.attrs {
            match parse_validate_rules_from_attribute(attr) {
                Ok(r) => {
                    rules.extend(r);
                }
                Err(err) => match &mut error {
                    Some(e) => {
                        e.combine(err);
                    }
                    None => {
                        error = Some(err);
                    }
                },
            }
        }

        field_infos.push(FieldInfo {
            field_name: field.ident.clone().unwrap(),
            ty: field.ty.clone(),
            rules,
        });
    }

    match error {
        Some(err) => Err(err),
        None => Ok(IntermediateCode {
            self_type,
            field_infos,
        }),
    }
}

fn parse_validate_rules_from_attribute(
    attribute: &Attribute,
) -> Result<Vec<ValidationRule>, syn::Error> {
    let list = attribute.meta.require_list()?;

    let parser = Punctuated::<ValidationRule, Token![,]>::parse_separated_nonempty;

    let container = list.parse_args_with(parser)?;
    let reval: Vec<ValidationRule> = container.into_iter().collect();
    Ok(reval)
}
