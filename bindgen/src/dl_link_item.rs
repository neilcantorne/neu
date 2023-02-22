use std::str::FromStr;
use quote::ToTokens;
use proc_macro2::{
    Ident,
    Span,
    Punct,
    Spacing,
    TokenStream,
    Group,
    Delimiter, Literal,
};
use syn::{
    parse::{ Parse, ParseStream },
    ItemForeignMod,
    ForeignItem,
    Pat,
    Type,
};


pub(crate) struct DlLinkItem {
    descriptors: Vec<Descriptor>,
}

impl DlLinkItem {
    pub fn generate(&self, name: Ident) -> TokenStream {
        let mut tokens = TokenStream::new();

        let mut items = TokenStream::from_iter([
            Ident::new("library", Span::call_site()).into_token_stream(),
            Punct::new(':', Spacing::Joint).into_token_stream(),
            Ident::new("usize", Span::call_site()).into_token_stream(),
            Punct::new(',', Spacing::Alone).into_token_stream(),
            
            Ident::new("counter", Span::call_site()).into_token_stream(),
            Punct::new(':', Spacing::Joint).into_token_stream(),
            TokenStream::from_str("std::sync::atomic::AtomicUsize").unwrap().into_token_stream(),
            Punct::new(',', Spacing::Alone).into_token_stream(),
        ]);
        
        for desc in self.descriptors.clone() {
            desc.generate_fn_ptr(&mut items);
            items.extend(Punct::new(',', Spacing::Alone).into_token_stream());
        }
        
        tokens.extend([
            Ident::new("pub", Span::call_site()).into_token_stream(),
            Ident::new("struct", Span::call_site()).into_token_stream(),  
            name.into_token_stream(),
            Group::new(Delimiter::Brace, items).into_token_stream(),
        ]);

        tokens
    }
}

impl Parse for DlLinkItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let block: ItemForeignMod = input.parse()?;
        
        let descriptors = (block.items.iter()
            .try_fold(Vec::new(), |mut desc_accumulator, 
                item| match item {
                ForeignItem::Fn(item) => {
                    let fn_ident = item.sig.ident.clone();
                    let parameters = item.sig.inputs.iter()
                        .try_fold(Vec::new(), |mut param_accumulator, argument| match argument {
                            syn::FnArg::Typed(arg) => {
                                
                                if let Pat::Ident(ident) = *arg.pat.clone() {
                                    param_accumulator.push((ident.ident, *arg.ty.clone()));
                                } else {
                                    return Err(syn::Error::new_spanned(arg, "Expect Identifier"));
                                }
                                
                                Ok(param_accumulator)
                            },
                            syn::FnArg::Receiver(r) => Err(syn::Error::new_spanned(r, "Invalid parameter")),
                        })?;
                    
                    desc_accumulator.push(Descriptor {
                        ptr_name: format!("ptr_{}", fn_ident.to_string()),
                        parameters,
                        fn_ident,
                    });
                    
                    Ok(desc_accumulator)
                },
                _ => Ok(desc_accumulator), 
            }
        ) as syn::Result<Vec<Descriptor>>)?;
        

        Ok(Self { descriptors })
    }
}

#[derive(Clone)]
struct Descriptor {
    ptr_name: String,
    parameters: Vec<(Ident, Type)>,
    fn_ident: Ident,
}

impl Descriptor {
    fn generate_fn_ptr(&self, tokens: &mut proc_macro2::TokenStream) {
        // Generate fn ptr parameters
        let mut param_tokens = TokenStream::new();
        if let Some(first) = self.parameters.first() {
            param_tokens.extend([
                // Specify type
                first.1.to_token_stream()]);
            
            for param in self.parameters.iter().step_by(1) {
                param_tokens.extend([
                    Punct::new(',', Spacing::Alone).into_token_stream(), // Comma separator
                    // Specify type
                    param.1.to_token_stream()]);
            }
        }
        
        tokens.extend([
            // Emit pointer name
            Ident::new(&self.ptr_name, Span::call_site()).into_token_stream(),
            // Emit fn pointer type
            Punct::new(':', Spacing::Alone).into_token_stream(),
            Ident::new("extern", Span::call_site()).into_token_stream(),
            Literal::string("C").into_token_stream(),
            Ident::new("fn", Span::call_site()).into_token_stream(),
            Group::new(Delimiter::Parenthesis, param_tokens).into_token_stream()
        ]);
    }
}
