
#[derive(Debug)]
pub struct Pin{
    data:u64,
    bit_size:u8
}

pub fn new(bit_size:u8)->Result<Pin,&'static str>{
    if bit_size>64{
        return Err("Max bit size is 64");
    }
    return Ok(Pin{data:0,bit_size:bit_size})
}

impl Pin{
    pub fn set_bit(&mut self,index:u8,value:bool)->Result<(),&'static str>{
        if index >= self.bit_size{
            return Err("Out of boundaries");
        }
        let mut mask:u64 = 1;
        mask = mask<<index;
        if value{
            self.data |= mask;
        }else{
            self.data &= !mask;
        }
        return Ok(());
    }
    pub fn get_bit(&self,index:u8)->Result<bool,&'static str>{
        if index >= self.bit_size{
            return Err("Out of boundaries");
        }
        let mut mask:u64 = 1;
        mask = mask<<index;
        if self.data&mask!=0{
            return Ok(true);
        }else{
            return Ok(false);
        }
    }
    pub fn get_data(&self)->u64{
        let mask:u64 = (2<<(self.bit_size-1))-1;
        return self.data & mask;
    }
    pub fn set_data(&mut self,input:u64){
        let mask:u64 = (2<<(self.bit_size-1))-1;
        self.data = input & mask;
    }
    pub fn clone(&self) -> Pin{
        return Pin{data:self.get_data(),bit_size:self.bit_size};
    }
}
