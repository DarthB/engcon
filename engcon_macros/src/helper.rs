use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Path;
use syn::PathSegment;

/// This Result returns in both cases a TokenStream whereby the TokenStream of the Error case
/// contains at least one `compile_error!(...)` expression
pub(crate) type MacroResult = std::result::Result<proc_macro::TokenStream, proc_macro::TokenStream>;

/// converts a syn::Error to a tokenstream, useful in `map_err`
pub(crate) fn synerr_to_tokens(err: syn::Error) -> proc_macro::TokenStream {
    err.to_compile_error().into()
}

/// Adapts an expression that contains an ident `field` with a preceeding self generating: `self.field`
pub(crate) fn adapt_ident_with_self(expr: &mut Expr) -> bool {
    let adaption: Option<syn::ExprField> = match &expr {
        Expr::Path(orig_path) => {
            // check if we're using just an ident:
            if orig_path.path.segments.len() == 1 {
                let ident = orig_path.path.segments.first().unwrap().ident.clone();
                let self_segment = PathSegment {
                    ident: syn::Ident::new("self", Span::call_site()),
                    arguments: syn::PathArguments::None,
                };
                let mut segments = Punctuated::new();
                segments.push(self_segment);
                Some(syn::ExprField {
                    attrs: vec![],
                    base: Box::new(Expr::Path(syn::ExprPath {
                        attrs: vec![],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments,
                        },
                    })),
                    dot_token: Default::default(),
                    member: syn::Member::Named(ident),
                })
            } else {
                None
            }
        }
        _ => None,
    };

    // if we use an identifier without self. we adapt the statement here
    if let Some(a) = adaption {
        *expr = Expr::Field(a);
        true
    } else {
        false
    }
}
