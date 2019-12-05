use protocol::message::Message;
use protocol::block::{Block, BlockCode, Orientation, Chunk};
use protocol::entity::*;

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

#[test]
fn test() {
    let msg_original = Message::CreateEntity(Entity::new(9_223_372_036_854_775_808, 0, 0, String::new(), EntityType::You));
    let msg_serialized = msg_original.encode();
    println!("{:?}", msg_serialized);
    let msg_deserialized = Message::decode(msg_serialized).unwrap();
    assert_eq!(msg_original, msg_deserialized);
}