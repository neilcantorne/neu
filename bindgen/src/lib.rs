use proc_macro::TokenStream;

mod dl_link_item;
mod dl_link_attr;

#[proc_macro_attribute]
pub fn dl_link(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    todo!()
}