// constants are 15 bit unsigned integers.
// all math is modulo 32768

// pub enum OPCODE
// {
//     HALT    ,
//     SET     (u16, u16),
//     PUSH    (u16),
//     POP     (u16),
//     EQ      (u16, u16, u16),
//     GT      (u16, u16, u16),
//     JMP     (u16),
//     JT      (u16, u16),
//     JF      (u16, u16),
//     ADD     (u16, u16, u16),
//     MULT    (u16, u16, u16),
//     MOD     (u16, u16, u16),
//     AND     (u16, u16, u16),
//     OR      (u16, u16, u16),
//     NOT     (u16, u16, u16),
//     RMEM    (u16, u16),
//     WMEM    (u16, u16),
//     CALL    (u16),
//     RET     ,
//     OUT     (u16),
//     IN      (u16),
//     NOOP    ,
// }

pub enum InterpretError {
    NOT_ENOUGH_ARGS,
    TOO_MANY_ARGS,
    MISSING_ARGS
}

pub struct VM {
    registers: [u16; 8],
    memory: [ u16;0b0111111111111111^0b0 ],
    stack: Vec<u16>
}

impl VM {
    pub fn New() -> Self {
        Self {
            memory    : [ 21; 0b0111111111111111^0b0 ],
            stack     : Vec::new(),
            registers : [ 0; 8 ]
        }
    }

    fn get_reg (reg: u16) -> Result<u16, ()> {
        if reg >= 32768 && reg <= 32775 {
            Ok(reg - 32768)
        } else {
            Err(())
        }
    }

    pub fn interpret(&mut self) {
        let mut ip = 0;
        let cur_inst = self.memory[ip];
        while self.memory[ip] != 21 {
            match cur_inst
            {
                0 =>
                {
                    //TODO replace with something that exists more gracefully
                    return
                },
                1 => 
                {
                    let a = self.memory[ip+1];
                    ip+=1;
                    let b = self.memory[ip+1];
                    ip+=1;
                    if a < 32768 {
                        return
                    }
                    self.registers[VM::get_reg(a).unwrap() as usize] = b;
                }
                19 => 
                {
                    let a = self.memory[ip+1];
                    print!("{}", a as u8 as char);
                    ip+=1;
                },
                _ => { () }
            }
            ip+=1;
        }
    }
}

fn disassemble ( vm: &VM ) {
    println!("\n\n=== MEMORY STUFFS LOL ===");
    for (index, reg) in vm.registers.iter().enumerate() {
        print!(" REG {:01} = {} ", index, reg);
    }
    println!(" ");
    for (index, inst) in vm.memory.iter().enumerate() {
        if inst != &21 {
            println!("{:#04x} : {}", index, inst);
        }
    }
}

fn main() {
    let mut vm = VM::New();
    vm.memory[0] = 1;
    vm.memory[1] = 32769;
    vm.memory[2] = 34;
    vm.interpret();
    disassemble(&vm);
    // println!("Hello, world!");
}
