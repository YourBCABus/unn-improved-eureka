use fcm::{Client, MessageBuilder, NotificationBuilder};

pub async fn notify() {
    let topic = "/topics/1fa55c29-9a9a-47f5-b2b1-690937364192.01269804-5d32-4040-aa9a-2f0c1a80be8c".to_string();

    let client = Client::new();

    let mut builder = NotificationBuilder::new();
    builder.title("Hey!");
    builder.body("This is a test with topics");
    let notification = builder.finalize();

    let mut builder = MessageBuilder::new(crate::env::notifications::fcm_api_key(), &topic);
    builder.notification(notification);
    let message = builder.finalize();

    let response = client.send(message).await.unwrap();
    println!("Sent: {:?}", response);
}    