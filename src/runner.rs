use crate::instructions::{Instruction, Label};
use crate::syscall::syscall;

pub struct Runner {
    instructions: Vec<Instruction>,
    registers: [i64; 4096],
    stack: Vec<usize>,
}

impl Runner {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            registers: [0; 4096],
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let mut pc = 0;
        let max_pc = self.instructions.len();

        loop {
            if pc >= max_pc {
                break;
            }

            let instruction = &self.instructions[pc];

            match instruction {
                Instruction::Set(value, destination) => {
                    let _value = self.read_value(value);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _value;
                }
                Instruction::Input(destination) => {
                    let _destination = self.read_address(destination);

                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim().parse::<i64>().unwrap();

                    self.registers[_destination] = input;
                }
                Instruction::Output(value) => {
                    let _value = self.read_value(value);

                    println!("{}", _value);
                }
                Instruction::Add(addend1, addend2, destination) => {
                    let _addend1 = self.read_value(addend1);
                    let _addend2 = self.read_value(addend2);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _addend1 + _addend2;
                }
                Instruction::Subtract(minuend, subtrahend, destination) => {
                    let _minuend = self.read_value(minuend);
                    let _subtrahend = self.read_value(subtrahend);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _minuend - _subtrahend;
                }
                Instruction::Multiply(factor1, factor2, destination) => {
                    let _factor1 = self.read_value(factor1);
                    let _factor2 = self.read_value(factor2);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _factor1 * _factor2;
                }
                Instruction::Divide(dividend, divisor, destination) => {
                    let _dividend = self.read_value(dividend);
                    let _divisor = self.read_value(divisor);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _dividend / _divisor;
                }
                Instruction::Modulo(dividend, divisor, destination) => {
                    let _dividend = self.read_value(dividend);
                    let _divisor = self.read_value(divisor);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _dividend % _divisor;
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
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
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
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
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
                    let _a = self.read_value(a);
                    let _b = self.read_value(b);
                    let _label = match label {
                        Label::Label(_) => panic!("Invalid label"),
                        Label::Instruction(value) => *value,
                    };

                    if _a < _b {
                        pc = _label;
                        continue;
                    }
                }
                Instruction::Move(source, destination) => {
                    let _source = self.read_value(source);
                    let _destination = self.read_address(destination);

                    self.registers[_destination] = _source;
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
                Instruction::Return => {
                    pc = self.stack.pop().unwrap();
                    continue;
                }
                Instruction::Time(destination) => {
                    let _destination = self.read_address(destination);

                    let time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    self.registers[_destination] = time as i64;
                }
                Instruction::Syscall(destination, sysno, a1, a2, a3, a4, a5, a6) => {
                    let _destination = self.read_address(destination);
                    let _sysno = self.read_value(sysno);

                    let _a1 = self.read_optional_value(a1);
                    let _a2 = self.read_optional_value(a2);
                    let _a3 = self.read_optional_value(a3);
                    let _a4 = self.read_optional_value(a4);
                    let _a5 = self.read_optional_value(a5);
                    let _a6 = self.read_optional_value(a6);

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
                        self.registers[_destination as usize] = ret as i64;
                    }
                }
                Instruction::Terminate => return,
            }

            pc += 1;
        }
    }

    fn read_optional_value(&self, param: &Option<crate::instructions::Param>) -> Option<i64> {
        match param {
            Some(value) => Some(self.read_value(value)),
            None => None,
        }
    }

    fn read_value(&self, param: &crate::instructions::Param) -> i64 {
        match param {
            crate::instructions::Param::Data(value) => *value,
            crate::instructions::Param::Address(value) => self.registers[*value],
            crate::instructions::Param::Reference(value) => {
                let referenced_reg = self.registers[*value];
                self.registers[referenced_reg as usize]
            }
        }
    }

    fn read_address(&self, param: &crate::instructions::Param) -> usize {
        match param {
            crate::instructions::Param::Address(value) => *value,
            crate::instructions::Param::Reference(value) => self.registers[*value] as usize,
            _ => panic!("Invalid address"),
        }
    }
}

fn optional_int_to_usize(value: Option<i64>) -> Option<usize> {
    match value {
        Some(value) => Some(value as usize),
        None => None,
    }
}
