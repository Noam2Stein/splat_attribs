use derive_syn_parse::Parse;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Attribute, Item, Result, Token};

/// Splatters attributes across multiple items.
///
/// ### Syntax
///
/// ```
/// splat_attribs! {
///     ~attributes~:
///     ~items~
/// }
/// ```
///
/// ### Example
/// ```
/// use splat_attribs::splat_attribs;
///
/// fn main() {
///     println!("{Casiopea} < {TSquare} < {Dimension}")
/// }
///
/// splat_attribs! {
///     #[allow(non_upper_case_globals)]
///     #[doc = "Applied to all items"]:
///
///     const Casiopea: u32 = 10 / 10;
///     const TSquare: u32 = 11 / 10;
///     const Dimension: u32 = u32::MAX / 10;
/// }
/// ```
#[proc_macro]
pub fn splat_attribs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { attribs, items } = parse_macro_input!(input as Input);

    let items_with_attribs = items.iter().map(|item| {
        quote! {
            #(#attribs)*
            #item
        }
    });

    quote! {
        #(#items_with_attribs)*
    }
    .into()
}

#[derive(Parse)]
struct Input {
    #[call(Attribute::parse_outer)]
    attribs: Vec<Attribute>,
    #[prefix(Token![:])]
    #[call(parse_items)]
    items: Vec<Item>,
}

fn parse_items(parser: ParseStream) -> Result<Vec<Item>> {
    let mut items = Vec::new();
    while !parser.is_empty() {
        items.push(parser.parse()?);
    }

    Ok(items)
}
