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
    ReturnType,
};


pub(crate) struct DlLinkItem {
    libname: Option<Literal>,
    descriptors: Vec<Descriptor>,
}

impl DlLinkItem {
    pub fn generate(&self, name: Ident) -> TokenStream {
        let mut tokens = TokenStream::new();

        let store_name = Ident::new(&format!("Store{name}"), name.span());
        let mut ptr_items = TokenStream::new();
        let mut load_items = TokenStream::new();
        let mut fn_store = TokenStream::new();
        let mut fn_caller = TokenStream::new();
        let libname = if let Some(name) = &self.libname {
            name.clone()
        } else {
            Literal::string(&name.to_string())
        };
        
        for desc in self.descriptors.clone() {
            desc.generate_fn_ptr(&mut ptr_items);
            desc.generate_loading(&mut load_items);
            desc.generate_fn_store(&mut fn_store);
            desc.generate_fn_caller(&mut fn_caller);
        }
        
        
        // Generate implementation
        tokens.extend(quote::quote!(
            struct #store_name {
                library: super::DynamicLibrary,
                #ptr_items
            }

            #[derive(Clone)]
            pub struct #name(std::sync::Arc<#store_name>);

            impl #name {
                pub fn load() -> Option<Self> {
                    let library = super::DynamicLibrary::load(#libname)?;

                    unsafe {
                        #load_items

                        Some(Self(std::sync::Arc::new(#store_name {
                            library
                            #fn_store
                        })))
                    }
                }

                #fn_caller
            }
        ));

        tokens
    }
}

impl Parse for DlLinkItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let block: ItemForeignMod = input.parse()?;
        let libname = block.attrs.iter().
            find(|attr| {
                if let Some(ident) = attr.path.get_ident() {
                    if *ident == "libname" { return true; }
                }
                
                false
            }).map(|lib_attr| {
                Literal::string(lib_attr.tokens.to_string()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim_matches('"'))
            });
        
        let descriptors = (block.items.iter()
            .try_fold(Vec::new(), |mut desc_accumulator, 
                item| match item {
                ForeignItem::Fn(item) => {
                    let fn_ident = item.sig.ident.clone();
                    let ret_type = item.sig.output.clone();
                    let mut symbol = if let Some(sym_attr) = item.attrs.iter().
                        find(|attr| {
                            if let Some(ident) = attr.path.get_ident() {
                                if *ident == "symbol" { return true; }
                            }
                            
                            false
                        })
                    {
                        sym_attr.tokens.to_string()
                            .trim_start_matches('(')
                            .trim_end_matches(')')
                            .to_string()
                    } else {
                        item.sig.ident.to_string()
                    };

                    symbol.push('\0');

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
                        ptr_name: format!("ptr_{fn_ident}"),
                        parameters,
                        symbol,
                        fn_ident,
                        ret_type
                    });
                    
                    Ok(desc_accumulator)
                },
                _ => Ok(desc_accumulator), 
            }
        ) as syn::Result<Vec<Descriptor>>)?;
        

        Ok(Self { descriptors, libname })
    }
}

#[derive(Clone)]
struct Descriptor {
    ptr_name: String,
    parameters: Vec<(Ident, Type)>,
    fn_ident: Ident,
    symbol: String,
    ret_type: ReturnType,
}

impl Descriptor {
    fn generate_fn_ptr(&self, tokens: &mut proc_macro2::TokenStream) {
        // Generate fn ptr parameters
        let mut param_tokens = TokenStream::new();
        if let Some(first) = self.parameters.first() {
            param_tokens.extend([
                // Specify type
                first.1.to_token_stream()]);
            
            for param in self.parameters.iter().skip(1) {
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
            Group::new(Delimiter::Parenthesis, param_tokens).into_token_stream(),
            self.ret_type.to_token_stream(),
            Punct::new(',', Spacing::Alone).into_token_stream(),
        ]);
    }

    fn generate_loading(&self, tokens: &mut proc_macro2::TokenStream) {
        let ptr_name = Ident::new(&self.ptr_name, Span::call_site());
        let symbol = Literal::byte_string(self.symbol.as_bytes());
        tokens.extend(quote::quote!(
            let #ptr_name = std::mem::transmute(library.get_function(#symbol)?.as_ptr());
        ))
    }

    fn generate_fn_store(&self, tokens: &mut proc_macro2::TokenStream) {
        let ptr_name = Ident::new(&self.ptr_name, Span::call_site());
        tokens.extend([
            Punct::new(',', Spacing::Alone).into_token_stream(),
            ptr_name.into_token_stream()
        ]);
    }

    fn generate_fn_caller(&self, tokens: &mut proc_macro2::TokenStream) {
        let ptr_name = Ident::new(&self.ptr_name, Span::call_site());
        let fn_name = self.fn_ident.clone();
        let ret_type = self.ret_type.clone();

        // Generate fn parameters
        let mut param_tokens = TokenStream::new();
        if let Some(first) = self.parameters.first() {
            param_tokens.extend([
                // Specify ident
                first.0.to_token_stream(),
                // Specify type
                Punct::new(':', Spacing::Alone).into_token_stream(),
                first.1.to_token_stream()]);
                
                for param in self.parameters.iter().skip(1) {
                    param_tokens.extend([
                    Punct::new(',', Spacing::Alone).into_token_stream(), // Comma separator
                    // Specify ident
                    param.0.to_token_stream(),
                    Punct::new(':', Spacing::Alone).into_token_stream(),
                    // Specify type
                    param.1.to_token_stream()]);
            }
        }

        // Generate fn arguments
        let mut arg_tokens = TokenStream::new();
        if let Some(first) = self.parameters.first() {
            arg_tokens.extend(first.0.to_token_stream());
                
                for param in self.parameters.iter().skip(1) {
                    arg_tokens.extend([
                    Punct::new(',', Spacing::Alone).into_token_stream(), // Comma separator
                    param.0.to_token_stream()]);
            }
        }

        tokens.extend([ quote::quote!(
            pub(super) unsafe fn #fn_name(&self, #param_tokens) #ret_type {
                (self.0.as_ref().#ptr_name)(#arg_tokens)
            }
        )]);
    }
}
