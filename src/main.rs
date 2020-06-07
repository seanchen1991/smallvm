

type Register = usize;
type Address = usize;

#[derive(Debug, Clone, Copy)]
enum Immediate {
    None(),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
}

enum Instruction {
    /// Do nothing
    NOP(),
    /// Move Immediate to Register
    MOV(Register, Immediate),
    /// Move Register contents to another Register
    MOVR(Register, Register),
    /// Jump to the Register
    JMP(Register),
    /// Jump if equal to location
    JE(Register),
    /// Jump if not equal to location
    JNE(Register),
    /// Jump if greater than
    JG(Register),
    /// Jump if less than
    JL(Register),
    /// Compare two Registers
    CMP(Register, Register),
    /// Print contents of Register
    PRINTR(Register),
    /// Print contents of Immediate Address
    PRINTV(Address),
    /// Store Immediate into heap at specific Address from stack
    VSTORE(Address, Immediate),
    /// Load Immediate from heap and pushes value to stack
    VLOAD(Address),
    /// Store Immediate into heap from Register contents
    VSTORER(Address, Register),
    /// Load an Immediate from heap to Register
    VLOADR(Register, Address),
    /// Add two Registers and pushes result to stack
    ADD(Register, Register),
    /// Subtract two Registers and pushes result to stack
    SUB(Register, Register),
    /// Multiples two Registers and pushes result to stack
    MUL(Register, Register),
    /// Divides two Registers and pushes result to stack 
    DIV(Register, Register),
    /// Bitwise AND of two Registers and pushes result to stack
    AND(Register, Register),
    /// Bitwise OR of two Registers and pushes result to stack
    OR(Register, Register),
    /// Bitwise XOR of two Registers and pushes result to stack
    XOR(Register, Register),
    /// Shifts the Register to the right by Immediate
    SHR(Register, Immediate),
    /// Shifts the Register to the left by Immediate
    SHL(Register, Immediate),
    /// Push Immediate onto the stack
    VPUSH(Immediate),
    /// Push Register contents onto the stack
    VPUSHR(Immediate),
    /// Pops an Immediate from the stack to the Register
    VPOP(Register),
    /// Call function at address in Register
    CALL(Register),
    /// Return from a function
    RET(),
    /// Stop execution
    HALT(),
}

struct VM {
    stack: Vec<Immediate>,
    data: Vec<Immediate>,
    code: Vec<u8>,
    ip: Address,
    flag_eq: bool,
    flag_gt: bool,
    registers: [Immediate; 8],
    is_executing: bool,
}

impl VM {
    fn new(code: Vec<u8>, heap_size: usize) -> Self {
        VM {
            ip: 0,
            flag_eq: false,
            flag_gt: false, 
            registers: [Immediate::U8(0); 8],
            code,
            stack: Vec::new(),
            data: vec![Immediate::U8(0); heap_size],
            is_executing: false
        }
    }

    fn cpu(&mut self) {
        // machine is already running
        if self.is_executing { return; }

        self.is_executing = true;

        while self.ip < self.code.len() && self.is_executing {
            // decode current instruction
            let inst = self.decode();

            // execute decoded instruction
            self.execute(inst).expect("Failed to execute instruction at ip: {:?}", self.ip);
        }

        // increment instruction pointer
        self.ip += 1;
    }
}

fn main() {
    //Example Program:
    // 1 0 0 10     MOV(R0, 10)
    // 1 1 0 8      MOV(R1, 8)
    // 1 2 0 22     MOV(R2, ?) Location to jump to if R0 is greater than R1
    // 1 3 0 25     MOV(R3, ?) Location to jump to otherwise
    // 6 0 1        CMP(R0, R1)
    // 23 2         JG(R2) Jump if R0 is greater than R1
    // 3 3          JMP(R3)
    // 7 0          PRINTR(R0)
    // 22           HALT()
    // 7 1          PRINTR(R1)
    // 22           HALT()
    let mut vm = VM::new(
        vec![1, 0, 0, 10, 1, 1, 0, 8, 1, 2, 0, 22, 1, 3, 0, 25, 6, 0, 1, 23, 2, 3, 3, 7, 0, 22, 7, 1, 22], 
        1024
    );
    vm.cpu();
}
