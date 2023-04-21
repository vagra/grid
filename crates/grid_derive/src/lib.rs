use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(ItemComm)]
pub fn grid_derive(input: TokenStream) -> TokenStream {
    
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {

        impl ItemComm for #name {
            
            fn next(&self) -> u16 {
                self.next
            }

            fn set_next(&mut self, index:u16) {
                self.next = index;
            }

            fn next_free(&self) -> u16 {
                self.next_free
            }

            fn set_next_free(&mut self, index:u16) {
                self.next_free = index;
            }
        }
    };

    gen.into()
}


