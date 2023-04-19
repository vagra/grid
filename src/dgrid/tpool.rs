use super::agent::*;


#[derive(Debug)]
pub struct TPool{ 
    pub data: Agents,
    pub size: u16,
    pub first_free: u16,
}
