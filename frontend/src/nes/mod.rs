mod cpu;
mod cpu_opcodes;
mod emulator;

enum InterruptType {
    IRQ,
    NMI,
    BRK,
}