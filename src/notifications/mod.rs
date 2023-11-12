use fcm::{Client, MessageBuilder, NotificationBuilder};

pub async fn notify() {
    let topic = "/topics/4c9171fb-1e34-4183-9936-7f71175b331c.526a1c83-aec5-41f3-b437-70940602c4a1".to_string();
    let api_key = "AAAAOnEFltY:APA91bEQPIUVPsPAPjp_a1-v6nDx5smYjglcKY37DinIf2SwA2cy3qaH4vBL6das0ejarkiaS-Ea31hUveMBixQOtnJIPZeibQ4K-ksk2VTI31eGOSPKVio7CV7nw02m5WcOMOlDUfwb";

    let client = Client::new();

    let mut builder = NotificationBuilder::new();
    builder.title("Hey!");
    builder.body("This is a test with topics");
    let notification = builder.finalize();

    let mut builder = MessageBuilder::new(api_key, &topic);
    builder.notification(notification);
    let message = builder.finalize();

    let response = client.send(message).await.unwrap();
    println!("Sent: {:?}", response);
}    