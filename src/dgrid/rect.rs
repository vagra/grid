use crate::*;


#[derive(Debug, Clone)]
pub struct LRect {
    pub l: i16,
    pub t: i16,
    pub r: i16,
    pub b: i16,
}

impl Default for LRect {

    fn default() -> Self {
        
        Self { 
            l: I16MAX,
            t: I16MAX,
            r: I16MIN,
            b: I16MIN,
        }
    }
}

impl LRect {

    pub fn new(l:i16, t:i16, r:i16, b:i16) -> Self {

        Self { l, t, r, b }
    }

    pub fn is_empty(&self) -> bool {

        self.l > self.r
    }
}



#[derive(Debug, Clone)]
pub struct TRect {
    pub l: u16,
    pub t: u16,
    pub r: u16,
    pub b: u16,
}

impl Default for TRect {

    fn default() -> Self {
        
        Self { 
            l: U16MAX,
            t: U16MAX,
            r: 0,
            b: 0,
        }
    }
}

impl TRect {

    pub fn new(l:u16, t:u16, r:u16, b:u16) -> Self {

        Self { l, t, r, b }
    }

    pub fn is_empty(&self) -> bool {

        self.l > self.r
    }
}