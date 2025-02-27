use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;

use super::IntermediateCode;

pub(super) fn codegen(ic: IntermediateCode) -> TokenStream {
    let type_name = ic.self_type;
    let type_name_as_str = type_name.to_string();

    let mut code = Vec::new();

    // 1. implement contract helper methods:
    let mut free_contract_functions = Vec::new();
    let mut free_args: Vec<proc_macro2::Ident> = Vec::new();
    let mut contract_functions = Vec::new();
    let mut contract_function_calls = Vec::new();
    for field in ic.field_infos {
        let field_name = field.field_name;
        let ty = field.ty;
        let field_name_str = field_name.to_string();
        let fn_name = format_ident!("contract_{}", field_name);
        let mut rules = Vec::new();
        let mut free_rules = Vec::new();

        let mut num_args = 0;
        for rule in field.rules {
            let op = rule.cmp_op;
            let right = rule.rigth;
            let op_str = quote! {#op}.to_string();
            let right_str = quote! {#right}.to_string();

            if rule.right_is_field_on_self {
                let ch = char::from_u32(97 + num_args).expect("valid char");
                let arg = syn::Ident::new(ch.to_string().as_str(), proc_macro2::Span::call_site());
                free_args.push(arg.clone());
                num_args += 1;

                free_rules.push(quote! {
                    if !(value #op #arg) {
                        let inject_msg = format!("value={}: '{}' {} '{}'", value, #field_name_str, #op_str, #right_str);
                        return Err(ValidationError::new(inject_msg, #type_name_as_str.to_owned()));
                    }
                });
            } else {
                free_rules.push(quote! {
                    if !(value #op #right) {
                        let inject_msg = format!("value={}: '{}' {} '{}'", value, #field_name_str, #op_str, #right_str);
                        return Err(ValidationError::new(inject_msg, #type_name_as_str.to_owned()));
                    }
                });
            }

            rules.push(quote! {
                if !(self.#field_name #op #right) {
                    let inject_msg = format!("value={}: '{}' {} '{}'", self.#field_name, #field_name_str, #op_str, #right_str);
                    return Err(ValidationError::new(inject_msg, #type_name_as_str.to_owned()));
                }
            });
        }

        free_contract_functions.push(quote! {
            #[inline]
            pub fn #fn_name(value: #ty, #(#free_args: #ty,)*) -> Result<(), ValidationError> {
                #(#free_rules)*
                Ok(())
            }
        });
        free_args.clear();

        contract_functions.push(quote! {
            #[inline]
            pub fn #fn_name(&self) -> Result<(), ValidationError> {
                #(#rules)*
                Ok(())
            }
        });

        contract_function_calls.push(quote! {
            self.#fn_name()?;
        });
    }

    code.push(quote! {
        #(#free_contract_functions)*
    });

    code.push(quote! {
        impl #type_name {
            #(#contract_functions)*
        }
    });

    // 2. Implement the Validator trait
    code.push(quote! {
        #[automatically_derived]
        impl Validator for #type_name {
            fn validate(&self) -> Result<(), ValidationError> {
                #(#contract_function_calls)*
                Ok(())
            }
        }
    });

    // 3. Implmentation of TryFrom
    // todo: try to implement this generically, but blocked by error: see engcon/lib.rs
    code.push(quote! {
        #[automatically_derived]
        impl TryFrom<#type_name> for Validated<#type_name> {
            type Error = ValidationError;

            fn try_from(value: #type_name) -> Result<Self, Self::Error> {
                match value.validate() {
                    Ok(_) => {
                        let reval = unsafe { Validated::new_unchecked(value)};
                        Ok(reval)
                    }
                    Err(err) => Err(err)
                }
            }
        }
    });

    quote! {
        #(#code)*
    }
    .into()
}
