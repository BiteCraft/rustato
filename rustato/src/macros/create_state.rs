use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, LitStr, parse::Parse, parse::ParseStream, Token, ItemStruct};

struct CreateStateArgs {
    state_id: LitStr,
    state_struct: ItemStruct,
}

impl Parse for CreateStateArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state_id = input.parse()?;
        input.parse::<Token![,]>()?;
        let state_struct = input.parse()?;

        Ok(CreateStateArgs {
            state_id,
            state_struct,
        })
    }
}

pub(crate) fn create_state_impl(input: TokenStream) -> TokenStream {
    let CreateStateArgs { state_id, state_struct } = parse_macro_input!(input as CreateStateArgs);

    let state_name = &state_struct.ident;
    let fields = &state_struct.fields;

    let field_names = fields.iter().map(|f| &f.ident);
    let field_types = fields.iter().map(|f| &f.ty);

    let wrapped_fields = field_names.clone().zip(field_types.clone()).map(|(name, ty)| {
        quote! {
            #name: rustato_core::state::StateWrapper<#ty>
        }
    });

    let getters = field_names.clone().map(|name| {
        quote! {
            pub fn #name(&self) -> &#ty {
                self.#name.get()
            }
        }
    });

    let setters = field_names.clone().map(|name| {
        let setter_name = format_ident!("set_{}", name.as_ref().unwrap());
        quote! {
            pub fn #setter_name(&mut self, value: #ty) {
                self.#name.set(value);
            }
        }
    });

    let expanded = quote! {
        #[derive(Clone)]
        pub struct #state_name {
            #(#wrapped_fields,)*
        }

        impl #state_name {
            #(#getters)*
            #(#setters)*
        }

        impl rustato_core::state_manager::StateTracker for #state_name {
            fn get_changed_fields(&self) -> Vec<String> {
                vec![
                    #(
                        if self.#field_names.is_changed() {
                            stringify!(#field_names).to_string()
                        } else {
                            String::new()
                        }
                    ),*
                ].into_iter().filter(|s| !s.is_empty()).collect()
            }

            fn reset_changed_fields(&mut self) {
                #(self.#field_names.reset_changed();)*
            }
        }

        impl Default for #state_name {
            fn default() -> Self {
                Self {
                    #(#field_names: rustato_core::state::StateWrapper::new(Default::default()),)*
                }
            }
        }

        rustato_core::GLOBAL_STATE_MANAGER.register_state(
            #state_id,
            #state_name::default()
        );
    };

    TokenStream::from(expanded)
}