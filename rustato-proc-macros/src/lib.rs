use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, LitStr};

#[proc_macro]
pub fn create_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let struct_name = &input.ident;
    let struct_def = &input;

    let expanded = quote! {
        #[derive(Clone, Default)]
        #struct_def

        rustato::GLOBAL_STATE_MANAGER.register_state::<#struct_name>(stringify!(#struct_name), #struct_name::default());
    };

    TokenStream::from(expanded)
}

struct CreateStateInput {
    id: LitStr,
    struct_def: ItemStruct,
}

impl syn::parse::Parse for CreateStateInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let struct_def = input.parse()?;
        Ok(CreateStateInput { id, struct_def })
    }
}