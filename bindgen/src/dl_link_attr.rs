use syn::parse::{ Parse, ParseStream };


pub(crate) struct DlLinkAttr {

}

impl Parse for DlLinkAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}