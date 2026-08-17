use crate::instructions::{Instruction, Label};
use crate::syscall::syscall;

use std::io::{self, Read};

pub struct Runner {
    registers: Vec<i64>,
    stack: Vec<usize>,
}

impl Runner {
    pub fn new(register_count: usize) -> Self {
        Self {
            registers: vec![0; register_count],
            stack: Vec::new(),
        }
    }

    pub fn run<const FAST: bool>(&mut self, instructions: &[Instruction]) {
        self.registers.fill(0);
        self.stack.clear();

        let mut pc = 0;
        let max_pc = instructions.len();

        loop {
            if pc >= max_pc {
                break;
            }

            let instruction = &instructions[pc];

            match instruction {
                Instruction::Return => {
                    if self.stack.is_empty() {
                        // Returning from main works as program exit.
                        return;
                    }

                    pc = self.stack.pop().unwrap();
                    continue;
                }
                Instruction::Set(value, destination) => {
                    let _value = self.read_source::<FAST>(value);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _value);
                }
                Instruction::Input(destination) => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<i64>().unwrap();

                    let _destination = self.read_destination::<FAST>(destination);
                    self.write_reg::<FAST>(_destination, input);
                }
                Instruction::CharInput(destination, size) => {
                    let _size = self.read_source::<FAST>(size);
                    if _size < 0 {
                        panic!("Cin size must be positive");
                    }

                    let stdin = io::stdin();
                    let mut buffer = vec![0; _size as usize];
                    let bytes_read = stdin.lock().read(&mut buffer).unwrap();

                    buffer.truncate(bytes_read); // In case less than x bytes were read
                    let result = String::from_utf8(buffer).expect("Found invalid UTF-8");

                    let _destination = self.read_destination::<FAST>(destination);
                    for (i, c) in result.chars().enumerate() {
                        self.write_reg::<FAST>(_destination + i, c as i64);
                    }
                }
                Instruction::Output(value) => {
                    let _value = self.read_source::<FAST>(value);

                    println!("{}", _value);
                }
                Instruction::CharOutput(value) => {
                    let _value = self.read_source::<FAST>(value);

                    print!("{}", _value as u8 as char);
                }
                Instruction::Add(addend1, addend2, destination) => {
                    let _addend1 = self.read_source::<FAST>(addend1);
                    let _addend2 = self.read_source::<FAST>(addend2);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _addend1 + _addend2);
                }
                Instruction::Subtract(minuend, subtrahend, destination) => {
                    let _minuend = self.read_source::<FAST>(minuend);
                    let _subtrahend = self.read_source::<FAST>(subtrahend);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _minuend - _subtrahend);
                }
                Instruction::Multiply(factor1, factor2, destination) => {
                    let _factor1 = self.read_source::<FAST>(factor1);
                    let _factor2 = self.read_source::<FAST>(factor2);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _factor1 * _factor2);
                }
                Instruction::Divide(dividend, divisor, destination) => {
                    let _dividend = self.read_source::<FAST>(dividend);
                    let _divisor = self.read_source::<FAST>(divisor);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _dividend / _divisor);
                }
                Instruction::Modulo(dividend, divisor, destination) => {
                    let _dividend = self.read_source::<FAST>(dividend);
                    let _divisor = self.read_source::<FAST>(divisor);
                    let _destination = self.read_destination::<FAST>(destination);

                    self.write_reg::<FAST>(_destination, _dividend % _divisor);
                }
                Instruction::Jump(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    pc = _label;
                    continue;
                }
                Instruction::JumpGreaterThan(a, b, label) => {
                    let _a = self.read_source::<FAST>(a);
                    let _b = self.read_source::<FAST>(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a > _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::JumpEqual(a, b, label) => {
                    let _a = self.read_source::<FAST>(a);
                    let _b = self.read_source::<FAST>(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a == _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::JumpLessThan(a, b, label) => {
                    let _a = self.read_source::<FAST>(a);
                    let _b = self.read_source::<FAST>(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a < _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::Call(label) => {
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    self.stack.push(pc + 1);
                    pc = _label;
                    continue;
                }
                Instruction::Time(destination) => {
                    let _destination = self.read_destination::<FAST>(destination);

                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    self.write_reg::<FAST>(_destination, time as i64);
                }
                Instruction::Syscall(destination, sysno, a1, a2, a3, a4, a5, a6) => {
                    let _destination = self.read_destination::<FAST>(destination);
                    let _sysno = self.read_source::<FAST>(sysno);

                    let _a1 = self.read_optional_source::<FAST>(a1);
                    let _a2 = self.read_optional_source::<FAST>(a2);
                    let _a3 = self.read_optional_source::<FAST>(a3);
                    let _a4 = self.read_optional_source::<FAST>(a4);
                    let _a5 = self.read_optional_source::<FAST>(a5);
                    let _a6 = self.read_optional_source::<FAST>(a6);

                    let ret = unsafe {
                        syscall(
                            _sysno as usize,
                            optional_int_to_usize(_a1),
                            optional_int_to_usize(_a2),
                            optional_int_to_usize(_a3),
                            optional_int_to_usize(_a4),
                            optional_int_to_usize(_a5),
                            optional_int_to_usize(_a6),
                        )
                    };

                    // https://git.musl-libc.org/cgit/musl/tree/src/internal/syscall_ret.c?h=v1.1.15
                    if ret > -4096isize as usize {
                        let errno = -(ret as i32);
                        panic!("syscall failed: {}", errno)
                    } else {
                        self.write_reg::<FAST>(_destination, ret as i64);
                    }
                }
                Instruction::Fault(msg) => {
                    panic!("Program fault: {}", msg);
                }
            }

            pc += 1;
        }
    }

    fn read_optional_source<const FAST: bool>(
        &self,
        param: &Option<crate::instructions::Source>,
    ) -> Option<i64> {
        param.as_ref().map(|value| self.read_source::<FAST>(value))
    }

    fn read_source<const FAST: bool>(&self, param: &crate::instructions::Source) -> i64 {
        match param {
            crate::instructions::Source::Data(value) => *value,
            crate::instructions::Source::Address(value) => self.read_reg::<FAST>(*value),
            crate::instructions::Source::Reference(value) => {
                let referenced_reg = self.read_reg::<FAST>(*value);
                self.read_reg::<FAST>(referenced_reg as usize)
            }
        }
    }

    fn read_destination<const FAST: bool>(
        &self,
        param: &crate::instructions::Destination,
    ) -> usize {
        match param {
            crate::instructions::Destination::Address(value) => *value,
            crate::instructions::Destination::Reference(value) => {
                self.read_reg::<FAST>(*value) as usize
            }
        }
    }

    fn read_reg<const FAST: bool>(&self, i: usize) -> i64 {
        if FAST {
            unsafe { *self.registers.get_unchecked(i) }
        } else {
            self.registers[i]
        }
    }

    fn write_reg<const FAST: bool>(&mut self, i: usize, value: i64) {
        if FAST {
            unsafe { *self.registers.get_unchecked_mut(i) = value }
        } else {
            self.registers[i] = value;
        }
    }
}

fn optional_int_to_usize(value: Option<i64>) -> Option<usize> {
    value.map(|value| value as usize)
}
