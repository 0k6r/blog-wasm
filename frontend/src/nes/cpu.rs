use crate::nes::InterruptType;


pub trait CPUTrait {
    //pub
    fn new(&mem: MainBus) -> dyn CPUTrait;
    //Assuming sequential execution, for asynchronously calling this with Execute, further work needed
    fn interrupt(interrupt_type: InterruptType);
    fn step();
    fn reset();
    fn reset_by_address(start_addr: Address);
    fn log();
    fn get_pc() -> Address;
    fn skip_dma_cycles();

    //private

    //Instructions are split into five sets to make decoding easier.
    //These functions return true if they succeed
    fn execute_implied(opcode: u8) -> bool;
    fn execute_branch(opcode: u8) -> bool;
    fn execute_type_0(opcode: u8) -> bool;
    fn execute_type_1(opcode: u8) -> bool;
    fn execute_type_2(opcode: u8) -> bool;

    fn read_address(addr: Address) -> Address;

    fn push_stack(value: u8);
    fn pull_stack() -> u8;

    //If a and b are in different pages, increases the m_SkipCycles by inc
    fn set_page_crossed(addr_a: Address, addr_b: Address, ink: i32);
    fn set_zn(value: u8);

    fn m_skip_cycle() -> i32;
    fn m_cycles() -> i32;
}

pub struct CPU {
    //Registers
    r_pc: Address,
    r_sp: u8,
    r_a: u8,
    r_x: u8,
    r_y: u8,

    //Status flags.
    //Is storing them in one byte better ?
    f_c: bool,
    f_z: bool,
    f_i: bool,
    f_d: bool,
    f_v: bool,
    f_n: bool,
}