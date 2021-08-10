//this file provides imple usage of library
mod rusty_gates;

use rusty_gates::wiring::Pin;
use rusty_gates::logic_gates::OR;
use rusty_gates::logic_gates::AND;
use rusty_gates::logic_gates::NAND;
use rusty_gates::logic_gates::XOR;
use rusty_gates::logic_gates::NOR;
use rusty_gates::logic_gates::XNOR;
use rusty_gates::logic_gates::NOT;
use rusty_gates::circuits::HalfAdder;

use rusty_gates::logic_gates::{ProcessMany,ProcessOne};
use rusty_gates::circuits::BasicActivity;

fn main() {
    let mut A = Pin::new(4).unwrap();
    A.set_bit(2, true);
    A.set_bit(0, true);
    println!("{:?}",A);

    let mut or_gate = OR::new(2,1).unwrap();
    let res =  or_gate.process(&[1u64,1u64]).unwrap();
    println!("OR: {:b}",res);

    let mut and_gate = AND::new(2,1).unwrap();
    let res =  and_gate.process(&[1u64,1u64]).unwrap();
    println!("AND: {:b}",res);

    let mut xor_gate = XOR::new(3,2).unwrap();
    let res =  xor_gate.process(&[0b11u64,0b01u64,0b01u64]).unwrap();
    println!("XOR: {:b}",res);

    let mut nand_gate = NAND::new(3,1).unwrap();
    let res =  nand_gate.process(&[0u64,1u64,1u64]).unwrap();
    println!("NAND: {:b}",res);

    let mut nor_gate = NOR::new(3,1).unwrap();
    let res =  nor_gate.process(&[1u64,1u64,1u64]).unwrap();
    println!("NOR: {:b}",res);

    let mut xnor_gate = XNOR::new(3,1).unwrap();
    let res =  xnor_gate.process(&[1u64,1u64,1u64]).unwrap();
    println!("XNOR: {:b}",res);

    let mut not_gate = NOT::new(5).unwrap();
    let res = not_gate.process(0);
    println!("NOT: {:b}",res);

    let mut half_adder = HalfAdder::new(4).unwrap();
    half_adder.set_pin(0, 0b0001);
    half_adder.set_pin(1, 0b1001);
    let result = half_adder.process();
    println!("{:?}",result);



}
