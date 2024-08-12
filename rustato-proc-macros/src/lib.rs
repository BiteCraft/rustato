use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Field, FieldsNamed, ItemStruct};

#[proc_macro_attribute]
pub fn auto_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    // Extrair os campos da estrutura
    let fields = match &input.fields {
        syn::Fields::Named(FieldsNamed { named, .. }) => named,
        _ => panic!("AutoState only supports structs with named fields"),
    };

    // Gerar a implementação de Fields
    let field_impls = fields.iter().map(|f: &Field| {
        let field_name = &f.ident;
        quote! {
            fields.push(&mut self.#field_name as &mut dyn ::std::any::Any);
        }
    });

    let expanded = quote! {
        #input

        impl #name {
            fn new() -> Self {
                Self::default()
            }
        }

        impl Drop for #name {
            fn drop(&mut self) {
                ::rustato::unregister_global_state(stringify!(#name));
            }
        }

        impl ::rustato::AutoState for #name {}

        impl ::rustato::Fields for #name {
            fn fields_mut(&mut self) -> Vec<&mut dyn ::std::any::Any> {
                let mut fields = Vec::new();
                #(#field_impls)*
                fields
            }
        }

        ::rustato::paste::paste! {
            #[::rustato::ctor::ctor]
            fn [<__register_ #name _state>]() {
                ::rustato::__register_global_state_immediately(stringify!(#name), || ::rustato::GlobalState::new(#name::new()));
            }
        }
    };

    TokenStream::from(expanded)
}
