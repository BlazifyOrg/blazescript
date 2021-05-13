/*
   Copyright 2021 BlazifyOrg
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
#[derive(Debug)]
pub enum OpCode {
    OpConstant(u16),
    OpPlus,
    OpMinus,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpPower,
    OpNot,
    OpAnd,
    OpOr,
    OpEquals,
    OpNotEquals,
    OpGreaterThan,
    OpGreaterThanEquals,
    OpLessThan,
    OpLessThanEquals,
    OpVarAssign(u16),
    OpVarAccess(u16),
    OpVarReassign(u16),
    OpJump(u16),
    OpJumpIfFalse(u16),
    OpCall,
    OpBlockStart,
    OpBlockEnd,
    OpIndexArray,
    OpPop,
}

impl OpCode {
    pub fn make_op(&self) -> Vec<u8> {
        match self {
            Self::OpConstant(arg) => make_three_byte_op(0x01, *arg),
            Self::OpPop => vec![0x02],
            Self::OpAdd => vec![0x03],
            Self::OpSubtract => vec![0x04],
            Self::OpMultiply => vec![0x05],
            Self::OpDivide => vec![0x06],
            Self::OpPower => vec![0x07],
            Self::OpJump(to) => make_three_byte_op(0x08, *to),
            Self::OpJumpIfFalse(to) => make_three_byte_op(0x09, *to),
            Self::OpPlus => vec![0x0A],
            Self::OpMinus => vec![0x0B],
            Self::OpNot => vec![0x0C],
            Self::OpAnd => vec![0x0D],
            Self::OpOr => vec![0x0E],
            Self::OpEquals => vec![0x0F],
            Self::OpNotEquals => vec![0x1A],
            Self::OpGreaterThan => vec![0x1B],
            Self::OpGreaterThanEquals => vec![0x1C],
            Self::OpLessThan => vec![0x1D],
            Self::OpLessThanEquals => vec![0x1E],
            Self::OpVarAssign(i) => make_three_byte_op(0x1F, *i),
            Self::OpVarAccess(i) => make_three_byte_op(0x2A, *i),
            Self::OpVarReassign(i) => make_three_byte_op(0x2B, *i),
            Self::OpBlockStart => vec![0x2C],
            Self::OpBlockEnd => vec![0x2D],
            Self::OpCall => vec![0x2E],
            Self::OpIndexArray => vec![0x2F],
        }
    }
}

fn convert_to_u8(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_to_u8(data));
    output
}
