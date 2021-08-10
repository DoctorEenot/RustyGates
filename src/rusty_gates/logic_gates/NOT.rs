pub use crate::rusty_gates::logic_gates::ProcessOne;


pub struct NOT{
    bit_size:u8
}

pub fn new(bit_size:u8)->Result<NOT,&'static str>{
    if bit_size>64{
        return Err("Max bit size is 64");
    }
    else if bit_size == 0{
        return Err("bit size cannot be 0");
    }
    return Ok(NOT{bit_size:bit_size});
}

impl ProcessOne for NOT{
    fn process(&self,input:u64) -> u64{
        let mask:u64 = (2<<(self.bit_size-1))-1;
        return (!input)&mask;
    }
}