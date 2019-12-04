use protocol::message::Message;
use protocol::block::{Block, BlockCode, Orientation, Chunk};

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

    #[test]
    fn chunk() {
        let msg_original = Message::Chunk(Chunk::new(54654, 564_564_565, [[Block::default();8];8]));
        let msg_serialized = msg_original.encode();
        println!("{}", msg_serialized.len());
        let msg_deserialized = Message::decode(msg_serialized).unwrap();
        assert_eq!(msg_original, msg_deserialized);

        let mut blocks = [[Block::default();8];8];
        blocks[2][3] = Block::new(BlockCode::SimpleWall, Orientation::Up);
        let msg_original = Message::Chunk(Chunk::new(65, 6_451_651_616, blocks));
        let msg_serialized = msg_original.encode();
        println!("{}", msg_serialized.len());
        let msg_deserialized = Message::decode(msg_serialized).unwrap();
        assert_eq!(msg_original, msg_deserialized);
    }