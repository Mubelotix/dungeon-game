#![warn(clippy::all)]

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
	Connect = 1, // Ask a username
}

#[derive(Debug, PartialEq)]
pub enum Message {
	Connect(String)
}

impl Message {
	pub fn decode(data: String) -> Result<Self, &'static str> {
		match data.as_bytes()[0] {
			1 => {
                if let Ok(username) = String::from_utf8(data.as_bytes()[1..].to_vec()) {
                    Ok(Message::Connect(username))
                } else {
                    Err("invalid username")
                }
			},
			_ => {
				Err("invalid opcode")
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
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect() {
        let msg_original = Message::Connect(String::from("jean miche muche"));
        let msg_serialized = msg_original.encode();
        let msg_deserialized = Message::decode(msg_serialized).unwrap();
        assert_eq!(msg_original, msg_deserialized);

        let msg_original = Message::Connect(String::new());
        let msg_serialized = msg_original.encode();
        let msg_deserialized = Message::decode(msg_serialized).unwrap();
        assert_eq!(msg_original, msg_deserialized);
    }
}
