use lapin::options::*;
use lapin::types::FieldTable;
use lapin::BasicProperties;
use lapin::Channel;
use lapin::Connection;
use lapin::ConnectionProperties;

#[tokio::test]
pub async fn test_v1() {
    println!("00000")
}


pub struct RabbitMqClient(Channel);
impl std::ops::Deref for RabbitMqClient {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}




