#![no_std]

use stm32f4::stm32f401::Peripherals;

pub fn init_gpio(pers: &Peripherals, gpio: &str, pin: u8, mode: &str) {
    let rcc = &pers.RCC;
    let gpio_bank = gpio.trim();
    let gpio_mode = mode.trim();

    match gpio_bank {
        "a" => {
            rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
            match pin {
                0 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder0().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr0().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder0().input());
                        pers.GPIOA.idr.read().idr0().bit_is_set();
                    }
                    _ => {}
                },
                1 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder1().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr1().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder1().input());
                        pers.GPIOA.idr.read().idr1().bit_is_set();
                    }
                    _ => {}
                },
                2 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder2().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr2().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder2().input());
                        pers.GPIOA.idr.read().idr2().bit_is_set();
                    }
                    _ => {}
                },
                3 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder3().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr3().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder3().input());
                        pers.GPIOA.idr.read().idr3().bit_is_set();
                    }
                    _ => {}
                },
                4 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder4().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr4().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder4().input());
                        pers.GPIOA.idr.read().idr4().bit_is_set();
                    }
                    _ => {}
                },
                5 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder5().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr5().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder5().input());
                        pers.GPIOA.idr.read().idr5().bit_is_set();
                    }
                    _ => {}
                },
                6 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder6().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr6().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder6().input());
                        pers.GPIOA.idr.read().idr6().bit_is_set();
                    }
                    _ => {}
                },
                7 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder7().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr7().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder7().input());
                        pers.GPIOA.idr.read().idr7().bit_is_set();
                    }
                    _ => {}
                },
                8 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder8().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr8().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder8().input());
                        pers.GPIOA.idr.read().idr8().bit_is_set();
                    }
                    _ => {}
                },
                9 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder9().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr9().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder9().input());
                        pers.GPIOA.idr.read().idr9().bit_is_set();
                    }
                    _ => {}
                },
                10 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder10().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr10().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder10().input());
                        pers.GPIOA.idr.read().idr10().bit_is_set();
                    }
                    _ => {}
                },
                11 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder11().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr11().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder11().input());
                        pers.GPIOA.idr.read().idr11().bit_is_set();
                    }
                    _ => {}
                },
                12 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder12().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr12().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder12().input());
                        pers.GPIOA.idr.read().idr12().bit_is_set();
                    }
                    _ => {}
                },
                13 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder13().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr13().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder13().input());
                        pers.GPIOA.idr.read().idr13().bit_is_set();
                    }
                    _ => {}
                },
                14 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder14().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr14().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder14().input());
                        pers.GPIOA.idr.read().idr14().bit_is_set();
                    }
                    _ => {}
                },
                15 => match gpio_mode {
                    "output" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder15().output());
                        pers.GPIOA.odr.modify(|_, w| w.odr15().set_bit());
                    }
                    "input" => {
                        pers.GPIOA.moder.modify(|_, w| w.moder15().input());
                        pers.GPIOA.idr.read().idr15().bit_is_set();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        "b" => {
            rcc.ahb1enr.write(|w| w.gpioben().set_bit());
            match pin {
                0 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder0().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr0().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder0().input());
                        pers.GPIOB.idr.read().idr0().bit_is_set();
                    }
                    _ => {}
                },
                1 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder1().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr1().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder1().input());
                        pers.GPIOB.idr.read().idr1().bit_is_set();
                    }
                    _ => {}
                },
                2 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder2().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr2().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder2().input());
                        pers.GPIOB.idr.read().idr2().bit_is_set();
                    }
                    _ => {}
                },
                3 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder3().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr3().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder3().input());
                        pers.GPIOB.idr.read().idr3().bit_is_set();
                    }
                    _ => {}
                },
                4 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder4().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr4().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder4().input());
                        pers.GPIOB.idr.read().idr4().bit_is_set();
                    }
                    _ => {}
                },
                5 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder5().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr5().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder5().input());
                        pers.GPIOB.idr.read().idr5().bit_is_set();
                    }
                    _ => {}
                },
                6 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder6().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr6().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder6().input());
                        pers.GPIOB.idr.read().idr6().bit_is_set();
                    }
                    _ => {}
                },
                7 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder7().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr7().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder7().input());
                        pers.GPIOB.idr.read().idr7().bit_is_set();
                    }
                    _ => {}
                },
                8 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder8().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr8().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder8().input());
                        pers.GPIOB.idr.read().idr8().bit_is_set();
                    }
                    _ => {}
                },
                9 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder9().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr9().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder9().input());
                        pers.GPIOB.idr.read().idr9().bit_is_set();
                    }
                    _ => {}
                },
                10 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder10().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr10().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder10().input());
                        pers.GPIOB.idr.read().idr10().bit_is_set();
                    }
                    _ => {}
                },
                11 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder11().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr11().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder11().input());
                        pers.GPIOB.idr.read().idr11().bit_is_set();
                    }
                    _ => {}
                },
                12 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder12().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr12().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder12().input());
                        pers.GPIOB.idr.read().idr12().bit_is_set();
                    }
                    _ => {}
                },
                13 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder13().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr13().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder13().input());
                        pers.GPIOB.idr.read().idr13().bit_is_set();
                    }
                    _ => {}
                },
                14 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder14().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr14().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder14().input());
                        pers.GPIOB.idr.read().idr14().bit_is_set();
                    }
                    _ => {}
                },
                15 => match gpio_mode {
                    "output" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder15().output());
                        pers.GPIOB.odr.modify(|_, w| w.odr15().set_bit());
                    }
                    "input" => {
                        pers.GPIOB.moder.modify(|_, w| w.moder15().input());
                        pers.GPIOB.idr.read().idr15().bit_is_set();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        "c" => {
            rcc.ahb1enr.write(|w| w.gpiocen().set_bit());
            match pin {
                0 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder0().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr0().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder0().input());
                        pers.GPIOC.idr.read().idr0().bit_is_set();
                    }
                    _ => {}
                },
                1 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder1().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr1().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder1().input());
                        pers.GPIOC.idr.read().idr1().bit_is_set();
                    }
                    _ => {}
                },
                2 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder2().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr2().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder2().input());
                        pers.GPIOC.idr.read().idr2().bit_is_set();
                    }
                    _ => {}
                },
                3 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder3().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr3().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder3().input());
                        pers.GPIOC.idr.read().idr3().bit_is_set();
                    }
                    _ => {}
                },
                4 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder4().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr4().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder4().input());
                        pers.GPIOC.idr.read().idr4().bit_is_set();
                    }
                    _ => {}
                },
                5 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder5().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr5().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder5().input());
                        pers.GPIOC.idr.read().idr5().bit_is_set();
                    }
                    _ => {}
                },
                6 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder6().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr6().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder6().input());
                        pers.GPIOC.idr.read().idr6().bit_is_set();
                    }
                    _ => {}
                },
                7 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder7().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr7().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder7().input());
                        pers.GPIOC.idr.read().idr7().bit_is_set();
                    }
                    _ => {}
                },
                8 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder8().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr8().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder8().input());
                        pers.GPIOC.idr.read().idr8().bit_is_set();
                    }
                    _ => {}
                },
                9 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder9().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr9().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder9().input());
                        pers.GPIOC.idr.read().idr9().bit_is_set();
                    }
                    _ => {}
                },
                10 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder10().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr10().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder10().input());
                        pers.GPIOC.idr.read().idr10().bit_is_set();
                    }
                    _ => {}
                },
                11 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder11().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr11().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder11().input());
                        pers.GPIOC.idr.read().idr11().bit_is_set();
                    }
                    _ => {}
                },
                12 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder12().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr12().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder12().input());
                        pers.GPIOC.idr.read().idr12().bit_is_set();
                    }
                    _ => {}
                },
                13 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder13().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr13().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder13().input());
                        pers.GPIOC.idr.read().idr13().bit_is_set();
                    }
                    _ => {}
                },
                14 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder14().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr14().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder14().input());
                        pers.GPIOC.idr.read().idr14().bit_is_set();
                    }
                    _ => {}
                },
                15 => match gpio_mode {
                    "output" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder15().output());
                        pers.GPIOC.odr.modify(|_, w| w.odr15().set_bit());
                    }
                    "input" => {
                        pers.GPIOC.moder.modify(|_, w| w.moder15().input());
                        pers.GPIOC.idr.read().idr15().bit_is_set();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        _ => {}
    }
}

    pub fn pin_on(pers: &Peripherals, pin: u8) {
    pers.GPIOA.odr.modify(|_, w| match pin {
        0 => w.odr0().set_bit(),
        1 => w.odr1().set_bit(),
        2 => w.odr2().set_bit(),
        3 => w.odr3().set_bit(),
        4 => w.odr4().set_bit(),
        5 => w.odr5().set_bit(),
        6 => w.odr6().set_bit(),
        7 => w.odr7().set_bit(),
        8 => w.odr8().set_bit(),
        9 => w.odr9().set_bit(),
        10 => w.odr10().set_bit(),
        11 => w.odr11().set_bit(),
        12 => w.odr12().set_bit(),
        13 => w.odr13().set_bit(),
        14 => w.odr14().set_bit(),
        15 => w.odr15().set_bit(),
        _ => w.odr0().clear_bit(),
    });
}

pub fn pin_off(pers: &Peripherals, pin: u8) {
    pers.GPIOA.odr.modify(|_, w| match pin {
        0 => w.odr0().clear_bit(),
        1 => w.odr1().clear_bit(),
        2 => w.odr2().clear_bit(),
        3 => w.odr3().clear_bit(),
        4 => w.odr4().clear_bit(),
        5 => w.odr5().clear_bit(),
        6 => w.odr6().clear_bit(),
        7 => w.odr7().clear_bit(),
        8 => w.odr8().clear_bit(),
        9 => w.odr9().clear_bit(),
        10 => w.odr10().clear_bit(),
        11 => w.odr11().clear_bit(),
        12 => w.odr12().clear_bit(),
        13 => w.odr13().clear_bit(),
        14 => w.odr14().clear_bit(),
        15 => w.odr15().clear_bit(),
        _ => w.odr0().set_bit(),
    });
}

pub fn delay(pers: &Peripherals, ms: u32) {
    let tim2 = &pers.TIM2;

    // Initiate timer 2.
    pers.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());

    // OPM set to one pulse mode.
    // CEN kep disabled.
    tim2.cr1.write(|w| w.opm().set_bit().cen().clear_bit());

    // Config prescaler to have counter operate at 1 KHz.
    // TIM2 clock 8Mhz (No idea if this it true).
    // PSC = 7999.
    // 8Mhz / (7999 + 1) = 1Khz.
    // Counter (CNT) will increase every ms.
    tim2.psc.write(|w| w.psc().bits(7999));

    // Set timer to go off in ms.
    pers.TIM2.arr.write(|w| w.arr().bits(ms));

    // Enable counter CEN.
    pers.TIM2.cr1.write(|w| w.cen().set_bit());

    // Wait until alarm goes off.
    while !pers.TIM2.sr.read().uif().bit_is_set() {}

    // Clear the event flag.
    pers.TIM2.sr.modify(|_, w| w.uif().clear_bit());
}

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
