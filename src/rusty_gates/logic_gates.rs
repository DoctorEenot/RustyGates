pub mod OR;
pub mod AND;
pub mod XOR;
pub mod NAND;
pub mod NOR;
pub mod XNOR;
pub mod NOT;

pub trait ProcessMany{
    fn process(&self,input:&[u64]) -> Result<u64,&'static str>;
}

pub trait ProcessOne{
    fn process(&self,input:u64) -> u64;
}