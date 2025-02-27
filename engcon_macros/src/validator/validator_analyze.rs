use super::PartialAST;

pub(super) fn analyze_ast(ast: PartialAST) -> Result<PartialAST, syn::Error> {
    // check if validated type is good:
    #[allow(clippy::unnecessary_filter_map)]
    let errors: Vec<syn::Error> = ast
        .validated_fields
        .iter()
        .filter_map(|el| {
            let errors: Vec<syn::Error> = el
                .attrs
                .iter()
                .filter_map(|e| {
                    if e.meta.path().is_ident("validate_value") {
                        match e.meta.require_list() {
                            Ok(_list) => None,
                            Err(_) => Some(syn::Error::new_spanned(
                                e.meta.clone(),
                                "is not a list of comma separated rules".to_owned(),
                            )),
                        }
                    } else {
                        None
                    }
                })
                .collect();

            Some(errors)
        })
        .flatten()
        .collect();

    if errors.is_empty() {
        Ok(ast)
    } else {
        let mut err = errors.first().unwrap().clone();
        errors.into_iter().skip(1).for_each(|el| err.combine(el));
        Err(err)
    }
}
