#[derive(Debug, Clone)]
pub enum InterpretError {
    missingArgs,
    notRegister,
}

trait reg {
    fn is_reg(&self) -> bool;
}

impl reg for u16 {
    fn is_reg(&self) -> bool {
        if self >= &32768 && self <= &32775 {
            true
        } else {
            false
        }
    }
}

pub struct VM {
    registers: [u16; 8],
    memory: [u16; 0b0111111111111111 ^ 0b0],
    stack: Vec<u16>,
}

impl VM {
    pub fn New() -> Self {
        Self {
            memory: [21; 0b0111111111111111 ^ 0b0],
            stack: Vec::new(),
            registers: [0; 8],
        }
    }

    pub fn interpret(&mut self) {
        let mut ip = 0;
        loop {
            let (instr, mut a, mut b, mut c) = (
                self.get_mem(ip),
                self.get_mem(ip + 1),
                self.get_mem(ip + 2),
                self.get_mem(ip + 3),
            );
            // println!("IP: {}, INSTR: {}, A: {}, B: {}, C: {}", ip, instr, a, b, c);
            if instr == 21 {
                break;
            }
            match instr {
                0 => {      // HALT 
                    //TODO replace with something that exists more gracefully
                    return;
                },
                1 => {     // SET register a to b
                    if a.is_reg() {
                        self.registers[VM::get_reg(a)] = b;
                    }else {
                        panic!("register expected, instead got {}", a);
                    }
                    ip += 2;
                },
                2 => {     // PUSH a onto the stack
                    if a.is_reg() {
                        self.stack.push(self.registers[VM::get_reg(a)]);
                    } else {
                        self.stack.push(a);
                    }
                },
                3 => { // POP from the stack and store in register
                    if a.is_reg() {
                        self.registers[VM::get_reg(a)] = self.stack.pop().expect("cannot pop; empty stack");
                    } else {
                        panic!("{} is not a register!! >:O", a);
                    }
                },
                9 => {
                    if b.is_reg() {
                        b = self.registers[VM::get_reg(b)];
                    }
                    if c.is_reg() {
                        c = self.registers[VM::get_reg(c)];
                    }

                    self.registers[VM::get_reg(a)] = (b + c) % 32768;
                    ip += 3;
                },
                19 => {
                    if a.is_reg() {
                        a = self.registers[VM::get_reg(a)];
                    }
                    print!("{}", a as u8 as char);
                    ip += 1;
                },
                _ => (),
            }
            ip += 1;
        }
    }

    // take in a number, if said number is a register id, spit out the index of said register.
    fn get_reg(reg: u16) -> usize {
        if reg.is_reg() {
            (reg - 32768) as usize
        } else {
            0
        }
    }

    // take in a memory address, and return the value of the element in the memory address. if the
    // element is a register, return whatever is in the register instead?
    fn get_mem(&self, addr: usize) -> u16 {
        let val = self.memory[addr];
        // if val.is_reg() {
            // self.registers[VM::get_reg(val)]
        // } else {
            val
        // }
    }
}

fn disassemble(vm: &VM) {
    println!("\n\n=== MEMORY STUFFS LOL ===");
    println!("\n=== REGISTERS ===");
    for (index, reg) in vm.registers.iter().enumerate() {
        println!(" REG {:01} = {} ", index, reg);
    }
    println!("\n\n=== Instructions ===");
    for (index, inst) in vm.memory.iter().enumerate() {
        if inst != &21 {
            println!("{:#04x} : {}", index, inst);
        }
    }

    println!("\n=== Stack ===");
    for (index, val) in vm.stack.iter().enumerate() {
        println!("{:04} : {}", index, val);
    }
}

fn main() {
    let mut vm = VM::New();
    vm.memory[0] = 2; // Set ARG A to ARG B
    vm.memory[1] = 8008; // Register one, ARG A
    vm.memory[2] = 3; // value, ARG B. register one should now be 5
    vm.memory[3] = 32768; // ADD ARG B and ARG C together and store it in ARG A
    vm.memory[5] = 19;
    vm.memory[6] = 32768;

    vm.interpret();
    disassemble(&vm);
    // println!("Hello, world!");
}
