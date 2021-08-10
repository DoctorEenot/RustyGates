use crate::rusty_gates::circuits::{BasicActivity,OutPinsActivity};
use crate::rusty_gates::wiring::Pin;
use crate::rusty_gates::logic_gates;
use crate::rusty_gates::logic_gates::ProcessMany;

#[derive(Debug)]
pub struct OutPins{
    Sum:Pin::Pin,
    Carry:Pin::Pin
}

#[derive(Debug)]
pub struct HalfAdder{
    A:Pin::Pin,
    B:Pin::Pin,
    bit_size:u8
}

pub fn new(bit_size:u8) -> Result<HalfAdder,&'static str>{
    let a = Pin::new(bit_size);
    if a.is_err(){
        return Err(a.unwrap_err());
    }
    
    let b = Pin::new(bit_size);
    if b.is_err(){
        return Err(b.unwrap_err());
    }
    
    let HA = HalfAdder{A:a.unwrap(),B:b.unwrap(),bit_size:bit_size};
    return Ok(HA);
}

impl BasicActivity for HalfAdder{
    fn process(&self) -> OutPins{
        let xor_gate = logic_gates::XOR::new(2, self.bit_size).unwrap();
        let and_gate = logic_gates::AND::new(2, self.bit_size).unwrap();

        let mut sum = Pin::new(self.bit_size).unwrap();
        sum.set_data(xor_gate.process(&[self.A.get_data(),self.B.get_data()]).unwrap());

        let mut carry = Pin::new(self.bit_size).unwrap();
        carry.set_data(and_gate.process(&[self.A.get_data(),self.B.get_data()]).unwrap());

        return OutPins{Sum:sum,Carry:carry};
    }    
    fn set_pin(&mut self,index:u64,value:u64) -> Result<(),&'static str>{
        if index>1{
            return Err("Wrong index of pin");
        }
        match index{
            0 => {self.A.set_data(value)}
            1 => {self.B.set_data(value)}
            _ => {}
        }
        return Ok(());
    }
    fn get_pin(&self,index:u64) -> Result<u64,&'static str>{
        if index>1{
            return Err("Wrong index of pin");
        }
        match index{
            0 => {return Ok(self.A.get_data())}
            1 => {return Ok(self.B.get_data())}
            _ => {return Ok(0)}
        }
    }
}

impl OutPinsActivity for OutPins{
    fn get_pin(&self,index:u64) -> Result<Pin::Pin,&'static str>{
        if index>1{
            return Err("Wrong index of pin");
        }
        match index{
            0 => {return Ok(self.Sum.clone())}
            1 => {return Ok(self.Carry.clone())}
            _ => {return Ok(Pin::new(1).unwrap())}
        }
    }
}