use log::{info, trace, warn};
use crate::nes::cpu::CPU;

trait Emulator {
    const NES_VIDEO_WIDTH: u32;
    const NES_VIDEO_HEIGHT: u32;
    fn new() -> dyn Emulator;
    fn run(rom_path: &str);
    fn set_video_width(width: u32);
    fn set_video_height(height: u32);
    fn set_video_scale(scale:f32);
    fn set_keys(p1:Vec<str>, p2:Vec<str>); //TODO Need use keyboard key
}

pub struct NES {
    m_bus:str, //TODO MainBus
    m_picture_bus:str, //TODO PictureBus
    m_cpu:CPU,
    m_ppu:str, //TODO PPU
    m_cartridge: str, //TODO Cartridge

}

impl Emulator for NES {
    const NES_VIDEO_WIDTH: u32 = 420;
    const NES_VIDEO_HEIGHT: u32 = 680;

    fn new() -> Box<NES> {
        todo!()
    }

    fn run(rom_path: &str) {
        todo!()
    }

    fn set_video_width(width: u32) {
        todo!()
    }

    fn set_video_height(height: u32) {
        todo!()
    }

    fn set_video_scale(scale: f32) {
        todo!()
    }

    fn set_keys(p1: Vec<str>, p2: Vec<str>) {
        todo!()
    }
}

impl NES {
    fn dma(page: u8) {
        todo!()
    }

}