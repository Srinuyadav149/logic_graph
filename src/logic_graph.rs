#[derive(Debug, Clone, Copy)]
pub enum LogicalOperation {
    And,
    Or,
    Not,
    Xor,
    LeftShift,
    RightShift
}

#[derive(Debug, Clone, Copy)]
pub enum HaltStatusCode {
    Done,
    Error
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Load(usize),
    Store(usize),
    Operate(LogicalOperation),
    Halt(HaltStatusCode)
}

#[derive(Debug, Clone, Copy)]
pub enum Bus {
    OneBit(u8),
    TwoBit(u8),
    FourBit(u8),
    EightBit(u8),
    SixteenBit(u16),
    ThirtyTwoBit(u32),
    SixtyFourBit(u64),
    OneTwentyEightBit([u64;2])
}

pub struct IoPin {
    index: usize,
    label: String
}

pub struct LogicGraphNode {
    operations: Vec<Opcode>,
    io_bus: Vec<Bus>
}

impl LogicGraphNode {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
            io_bus: Vec::new(),
        }
    }
    
    pub fn load_instructions(&mut self, opcodes: Vec<Opcode>) {
        self.operations = opcodes;
    }
    
    pub fn connect_io(&mut self, io_bus: Vec<Bus>) {
        self.io_bus = io_bus;
    }
    
    pub fn tick(&mut self) {
        let mut index: usize = 0;
        let mut operation_stack: Vec<Bus> = Vec::new();
        while index < self.operations.len() {
            match self.operations[index] {
                Opcode::Load(load_index) => {
                    operation_stack.push(self.io_bus[load_index]);
                }
                Opcode::Store(store_index) => {
                    let output = operation_stack.pop();
                    match output {
                        Some(bus) => {
                            self.io_bus[store_index] = bus;
                        }
                        None => {
                            println!("Error Storing Output");
                            break;
                        }
                    }
                }
                Opcode::Operate(operation) => {
                    match operation {
                        LogicalOperation::And => {
                            
                        }
                        LogicalOperation::Or => {
                            
                        }
                        LogicalOperation::Not => {
                            
                        }
                        LogicalOperation::Xor => {
                            
                        }
                        LogicalOperation::LeftShift => {
                            
                        }
                        LogicalOperation::RightShift => {
                            
                        }
                    }
                }
                Opcode::Halt(status_code) => {
                    
                }
            }
            index += 1;
        }
    }
}