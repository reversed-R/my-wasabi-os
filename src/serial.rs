use core::fmt;

use crate::arch::x86::{busy_loop_hint, read_io_port_u8, write_io_port_u8};

pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    pub fn new(base: u16) -> Self {
        Self { base }
    }

    pub fn new_for_com1() -> Self {
        // Use COM1 at I/O port 0x3f8
        Self::new(0x3f8)
    }

    // UART などのシリアル通信では
    // 通信速度(baud rate) = clock / ( 除算値(divisor) * prescaler(マシン依存の定数) )
    // の計算式で通信速度を計算する回路を介して、
    // divisorを設定することを通して通信速度を設定するらしい
    pub fn init(&mut self) {
        // Disable all interrupts
        write_io_port_u8(self.base + 1, 0x00);

        // Enable DLAB (set baud rate divisor)
        //
        // DLAB(Divisor Latch Access Bit)
        // に1が設定されることで
        // offset +0と+1をbaud divisorを設定する際のための
        // 16bit の 下位8bitと上位8bitにそれぞれ割り当てる
        //
        // ここではDLABに1を設定している
        write_io_port_u8(self.base + 3, 0b1000_0000);

        // divisor(16bit) を8bitずつ2ポートに分けて設定
        const BAUD_DIVISOR: u16 = 0x0001;
        write_io_port_u8(self.base, (BAUD_DIVISOR & 0xff) as u8);
        write_io_port_u8(self.base + 1, (BAUD_DIVISOR >> 8) as u8);

        // 8 bits, no parity, one stop bit
        //
        // UART では
        // start bit(1bit), data(8bit), parity bit(0 or 1 bit), stop bit(2 or 1 bit)
        // という形式をとる
        // ここでは、
        // パリティなし、stop bitは1bitを設定している
        write_io_port_u8(self.base + 3, 0x03);

        // Enable FIFO, clear them, with 14-byte threshold
        // port offset が +2 にすると、
        // FIFOの設定ができる
        write_io_port_u8(self.base + 2, 0xc7);

        // IRQs enabled, RTS/DSR set
        write_io_port_u8(self.base + 4, 0x0b);
    }

    pub fn send_char(&self, c: char) {
        while (read_io_port_u8(self.base + 5) & 0x20) == 0 {
            busy_loop_hint();
        }
        write_io_port_u8(self.base, c as u8);
    }

    pub fn send_str(&self, s: &str) {
        let mut sc = s.chars();
        let slen = s.chars().count();
        for _ in 0..slen {
            self.send_char(sc.next().unwrap());
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let serial = Self::default();
        serial.send_str(s);

        Ok(())
    }
}

impl Default for SerialPort {
    fn default() -> Self {
        Self::new_for_com1()
    }
}
