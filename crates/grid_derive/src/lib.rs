use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(ItemComm)]
pub fn derive_item(input: TokenStream) -> TokenStream {
    
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



#[proc_macro_derive(GridComm)]
pub fn derive_grid(input: TokenStream) -> TokenStream {
    
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {

        impl GridComm for #name {

            fn in_grid(&self, x: i16, y: i16) -> bool {
                x >= -self.half_width && x < self.half_width &&
                y >= -self.half_height && y < self.half_height
            }
            
            fn pos2grid(&self, x:i16, y:i16) -> (i16, i16) {
                (self.half_width + x, self.half_height - y)
            }
        }
    };

    gen.into()
}
