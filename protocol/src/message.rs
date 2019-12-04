use std::mem::transmute;
use crate::block::{Block, BlockCode, Orientation};

fn u64_to_8_u8(x: u64) -> [u8;8] {
    unsafe { transmute(x.to_be()) }
}

fn u16_to_2_u8(x: u16) -> [u8; 2] {
    unsafe { transmute(x.to_be()) }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
	Connect = 1, // Ask a username
	ChatMessage, // Chat message
	Chunk, // 8*8 blocks
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Connect(String),
    ChatMessage(String, String, String),
    Chunk(u64, u64, [[Block;8];8])
}

impl Message {
	pub fn decode(data: String) -> Result<Self, &'static str> {
        if data.is_empty() {
            return Err("empty message");
        }
        let data = data.as_bytes();
		match data[0] {
			1 => {
                if let Ok(username) = String::from_utf8(data[1..].to_vec()) {
                    Ok(Message::Connect(username))
                } else {
                    Err("invalid username")
                }
            },
            2 => {
                if data.len() < 2 {
                    return Err("too short message");
                }
                let from_lenght = data[1] as usize;
                if data.len() < 3 + from_lenght {
                    return Err("too short message");
                }
                let to_lenght = data[2+from_lenght] as usize;
                if data.len() < 4 + from_lenght + to_lenght {
                    return Err("too short message");
                }
                let message_lenght = data[3+from_lenght+to_lenght] as usize;
                if data.len() < 4 + from_lenght + to_lenght + message_lenght {
                    return Err("too short message");
                }
                if let Ok(from) = String::from_utf8(data[2..2+from_lenght].to_vec()) {
                    if let Ok(to) = String::from_utf8(data[3+from_lenght..3+from_lenght+to_lenght].to_vec()) {
                        if let Ok(message) = String::from_utf8(data[4+from_lenght+to_lenght..4+from_lenght+to_lenght+message_lenght].to_vec()) {
                            Ok(Message::ChatMessage(from, to, message))
                        } else {
                            Err("invalid recipient username")
                        }
                    } else {
                        Err("invalid recipient username")
                    }
                } else {
                    Err("invalid sender username")
                }
            },
            3 => {
                if data.len() != 209 {
                    return Err("wrong message lenght");
                }
                let x = u64::from_be_bytes([data[1],data[2],data[3],data[4],data[5],data[6],data[7],data[8]]);
                let y = u64::from_be_bytes([data[9],data[10],data[11],data[12],data[13],data[14],data[15],data[16]]);
                let mut blocks: [[Block; 8]; 8] = [[Block::default(); 8]; 8];
                for i in 0..8 {
                    for j in 0..8 {
                        let block_code: BlockCode = u16::from_be_bytes([data[17+i*8*3+j*3], data[18+i*8*3+j*3]]).into();
                        let orientation: Orientation = data[19+i*8*3+j*3].into();
                        blocks[i][j] = Block::new(block_code, orientation);
                    }
                }
                Ok(Message::Chunk(x, y, blocks))
            }
			i => {
                panic!("invalid opcode {}", i);
			}
		}
	}

	pub fn encode(&self) -> String {
		match self {
			Message::Connect(username) => {
				let mut data = vec![OpCode::Connect as u8];
				data.append(&mut username.as_bytes().to_vec());

				unsafe {
					String::from_utf8_unchecked(data)
				}
            },
            Message::ChatMessage(from, to, message) => {
                // TODO assert 256 max size
                let mut data = vec![OpCode::ChatMessage as u8];
                data.push(from.len() as u8);
                data.append(&mut from.as_bytes().to_vec());
                data.push(to.len() as u8);
                data.append(&mut to.as_bytes().to_vec());
                data.push(message.len() as u8);
                data.append(&mut message.as_bytes().to_vec());

                unsafe {
					String::from_utf8_unchecked(data)
				}
            },
            Message::Chunk(x, y, blocks) => {
                let mut data = vec![OpCode::Chunk as u8];
                data.append(&mut u64_to_8_u8(*x).to_vec());
                data.append(&mut u64_to_8_u8(*y).to_vec());
                for i in 0..8 {
                    for j in 0..8 {
                        println!("b: {:?}", u16_to_2_u8(blocks[i][j].get_block_code() as u16));
                        data.append(&mut u16_to_2_u8(blocks[i][j].get_block_code() as u16).to_vec());
                        data.push(blocks[i][j].get_orientation() as u8);
                    }
                }

                unsafe {
					String::from_utf8_unchecked(data)
				}
            }
		}
	}
}