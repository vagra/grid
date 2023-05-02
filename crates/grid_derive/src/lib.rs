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



#[proc_macro_derive(CellComm)]
pub fn derive_cell(input: TokenStream) -> TokenStream {
    
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {

        impl CellComm for #name {
            
            fn is_empty(&self) -> bool {

                self.head == INVALID
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
                y > -self.half_height && y <= self.half_height
            }

            fn out_bounds(&self, x: i16, y: i16) -> Option<u8> {
                let l = -self.half_width;
                let t = self.half_height;
                let r = self.half_width;
                let b = -self.half_height;
                
                if x < l && y > t {return Some(1);}
                if x < l && y < b {return Some(3);}
                if x > r && y < b {return Some(5);}
                if x > r && y > t {return Some(7);}
            
                if y > t {return Some(0);}
                if x < l {return Some(2);}
                if y < b {return Some(4);}
                if x > r {return Some(6);}
            
                return None;
            }
            
            fn pos2grid(&self, x:i16, y:i16) -> (i16, i16) {

                (self.half_width + x, self.half_height - y)
            }

            fn grid2pos(&self, x:i16, y:i16) -> (i16, i16) {

                (x - self.half_width, self.half_height - y)
            }
        }
    };

    gen.into()
}
