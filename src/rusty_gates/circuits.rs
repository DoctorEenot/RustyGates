use crate::rusty_gates::wiring;
pub mod HalfAdder;

pub trait BasicActivity{
    fn process(&self) -> HalfAdder::OutPins;
    fn set_pin(&mut self,index:u64,value:u64) ->Result<(),&'static str>;
    fn get_pin(&self,index:u64) -> Result<u64,&'static str>;
}

pub trait OutPinsActivity{
    fn get_pin(&self,index:u64) -> Result<wiring::Pin::Pin,&'static str>;
}