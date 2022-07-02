use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Snaked)]
pub fn snaked(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let snake_case = snake(&ident.to_string());
    quote::quote!(
        impl snake_cased::Snaked for #ident {
            fn snaked() -> &'static str {
                #snake_case
            }
        }
    )
    .into()
}

#[proc_macro_derive(SnakedEnum)]
pub fn snaked_enum(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let data = match input.data {
        syn::Data::Enum(data) => data,
        _ => {
            let error = syn::Error::new(ident.span(), "expect enum").to_compile_error();
            return quote::quote!(
                #error
            )
            .into();
        }
    };
    let enum_idents = data
        .variants
        .into_iter()
        .map(|v| v.ident)
        .collect::<Vec<_>>();
    let enum_snakes = enum_idents
        .iter()
        .map(|i| snake(&i.to_string()))
        .collect::<Vec<_>>();
    quote::quote!(
        impl snake_cased::SnakedEnum for #ident {
            fn snaked_enum(&self) -> &'static str {
                match self {
                    #(Self::#enum_idents => #enum_snakes,)*
                }
            }
        }
    )
    .into()
}

fn snake(input: &str) -> String {
    let mut out = String::default();
    let mut chars = input.chars();
    out.push(chars.next().unwrap().to_ascii_lowercase());
    while let Some(ch) = chars.next() {
        match ch.is_ascii_uppercase() {
            true => {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            }
            false => out.push(ch),
        }
    }
    out
}

#[test]
fn t() {
    assert_eq!("a", &snake("A"));
    assert_eq!("a", &snake("a"));
    assert_eq!("a_a", &snake("AA"));
    assert_eq!("aa", &snake("Aa"));
    assert_eq!("aa_a", &snake("AaA"));
}
