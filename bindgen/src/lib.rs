use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::parse;

mod dl_link_item;

#[proc_macro_attribute]
pub fn dl_link(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name: Ident = parse(attr).unwrap();
    let dl_link_item: dl_link_item::DlLinkItem = parse(item).unwrap();
    
    //panic!("{}", dl_link_item.generate(name));
    dl_link_item.generate(name).into()
}