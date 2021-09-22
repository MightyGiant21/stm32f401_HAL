use stm32f4::stm32f401::Peripherals;
use super::*;


pub fn init_glcd(pers: &Peripherals) {
    // 8 bit mode.
    send_cmd(pers, 0x30);

    // 8 bit mode.
    send_cmd(pers, 0x30);

    send_cmd(pers, 0x08);

    // Clear screen.
    send_cmd(pers, 0x1);

    // Cursor set.
    send_cmd(pers, 0x6);

    send_cmd(pers, 0xC);

    // Return home.
    send_cmd(pers, 0x2);
}

pub fn set_graphic_mode(pers: &Peripherals) {
    send_cmd(pers, 0x30);
    send_cmd(pers, 0x34);
    send_cmd(pers, 0x36);
}

pub fn set_char_mode(pers: &Peripherals) {
    send_cmd(pers, 0x30);
}

pub fn send_cmd(pers: &Peripherals, byte: u8) {
    pin_on(pers, 2);

    // Send 11111.
    pin_on(pers, 1);
    for i in 0..5 {
        tick(pers);
    }

    // Send 000.
    for i in 0..3 {
        pin_off(pers, 1);
        tick(pers);
    }

    set_bit(pers, byte);

    pin_off(pers, 2);
}

pub fn send_data(pers: &Peripherals, byte: u8) {
    pin_on(pers, 2);

    // Send 11111.
    pin_on(pers, 1);
    for i in 0..5 {
        tick(pers);
    }

    // Send 010.
    pin_off(pers, 1);
    tick(pers);
    pin_on(pers, 1);
    tick(pers);
    pin_off(pers, 1);
    tick(pers);

    set_bit(pers, byte);

    pin_off(pers, 2);
}

fn tick(pers: &Peripherals) {
    let ms = 0;
    pin_on(pers, 0);
    pin_off(pers, 0);
}

fn set_bit(pers: &Peripherals, byte: u8) {
    let hi = byte >> 4;
    let lo = byte & 0xF;

    // Set high nibble.
    for i in 0..4 {
        if hi >> (3 - i) & 0x1 == 1 {
            pin_on(pers, 1);
            tick(pers);
        } else if hi >> (3 - i) & 0x1 == 0 {
            pin_off(pers, 1);
            tick(pers);
        }
    }

    // Send 0000.
    for i in 0..4 {
        pin_off(pers, 1);
        tick(pers);
    }

    // Set low nibble.
    for i in 0..4 {
        if lo >> (3 - i) & 0x1 == 1 {
            pin_on(pers, 1);
            tick(pers);
        } else if lo >> (3 - i) & 0x1 == 0 {
            pin_off(pers, 1);
            tick(pers);
        }
    }

    // Send 0000.
    for i in 0..4 {
        pin_off(pers, 1);
        tick(pers);
    }
}

pub fn draw_bitmap(pers: &Peripherals, bitmap: &[u8]) {
    for y in 0..64 {
        if y < 32 {
            for x in 0..8 {
                send_cmd(pers, 0x80 | y);
                send_cmd(pers, 0x80 | x);
                send_data(pers, bitmap[2 * x as usize + 16 * y as usize]);
                send_data(pers, bitmap[2 * x as usize + 1 + 16 * y as usize]);
            }
        } else {
            for x in 0..8 {
                send_cmd(pers, 0x80 | (y + 32));
                send_cmd(pers, 0x88 | x);
                send_data(pers, bitmap[2 * x as usize + 16 * y as usize]);
                send_data(pers, bitmap[2 * x as usize + 1 + 16 * y as usize]);
            }
        }
    }
}

pub fn draw_font(pers: &Peripherals, font: &[u8]) {
    for y in 0..64 {
        if y < 32 {
            for x in 0..8 {
                send_cmd(pers, 0x80 | y);
                send_cmd(pers, 0x80 | x);
                send_data(pers, font[2 * x as usize + 16 * y as usize]);
                send_data(pers, font[2 * x as usize + 1 + 16 * y as usize]);
            }
        } else {
            for x in 0..8 {
                send_cmd(pers, 0x80 | (y + 32));
                send_cmd(pers, 0x88 | x);
                send_data(pers, font[2 * x as usize + 16 * y as usize]);
                send_data(pers, font[2 * x as usize + 1 + 16 * y as usize]);
            }
        }
    }
}
