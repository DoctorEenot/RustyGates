pub use crate::rusty_gates::logic_gates::ProcessMany;

pub struct XNOR{
    inputs:u8,
    bit_size:u8,

}

pub fn new(inputs:u8,bit_size:u8)->Result<XNOR,&'static str>{
    if bit_size>64{
        return Err("Max bit size is 64");
    }
    else if bit_size == 0{
        return Err("bit size cannot be 0");
    }
    return Ok(XNOR{inputs:inputs,bit_size:bit_size});
}

impl ProcessMany for XNOR{
    fn process(&self,input:&[u64])->Result<u64,&'static str>{
        if input.len() != self.inputs as usize{
            return Err("Wrong input size");
        }
        let mut result:u64 = 0;
        
        for n in 0..self.bit_size{
            let mut equal = false;
            let mut found_1 = false;
            let mask:u64 = 1<<n;
            for i in 0..self.inputs{
                if input[i as usize]&mask != 0{
                    if found_1{
                        equal = true;
                        break;
                    }
                    found_1 = true;
                }
            }
            if !equal{
                result |= mask;
            }
        }
        result = !result;
        let mask:u64 = (2<<(self.bit_size-1))-1;
        return Ok(result&mask);
    }
}