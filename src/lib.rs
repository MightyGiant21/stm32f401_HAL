use stm32f4::stm32f401::Peripherals;

pub fn init_gpio(gpio: &str, pin: u8, mode: &str) {
    let pers = Peripherals::take().unwrap();
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