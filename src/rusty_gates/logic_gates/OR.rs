pub use crate::rusty_gates::logic_gates::ProcessMany;

pub struct OR{
    inputs:u8,
    bit_size:u8,
}

pub fn new(inputs:u8,bit_size:u8)->Result<OR,&'static str>{
    if bit_size>64{
        return Err("Max bit size is 64");
    }
    else if bit_size == 0{
        return Err("bit size cannot be 0");
    }
    return Ok(OR{inputs:inputs,bit_size:bit_size});
}

impl ProcessMany for OR{
    fn process(&self,input:&[u64])->Result<u64,&'static str>{
        if input.len() != self.inputs as usize{
            return Err("Wrong input size");
        }
        let mut result:u64 = input[0];
        for i in 1..self.inputs{
            result |= input[i as usize];
        }
        let mask:u64 = (2<<(self.bit_size-1))-1;
        return Ok(result&mask);
    }
}