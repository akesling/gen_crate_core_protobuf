fn main() {
    let mut hello = schema::example::HelloProtocolBuffer::default();
    hello.greeting = "Hello".to_string();
    hello.object = "Generated Crate Core".to_string();

    println!(
        "Built protobuf as JSON: {}",
        serde_json::to_string(&hello).unwrap()
    );
}
