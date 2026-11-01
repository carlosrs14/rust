/*
Let's model a real-time chat system where users can
share audio and video files.
 
Define a DigitalContent enum with two variants:
AudioFile and VideoFile. Derive a Debug implementation.
 
Define a ChatMessage struct with two fields: `content`
and `time`. The struct should define one generic type, T,
which will be the type of the `content` field.
The `time` field should always be a String.
Derive a Debug implementation.
 
Add an impl block for ChatMessage structs whose T type
is a DigitalContent enum. Define a `consume_entertainment`
method that prints out the value of the `content` field in
Debug format. For example, "Watching the AudioFile".
 
Add an impl block for ChatMessage structs with any type T.
Define a `retrieve_time` method that returns a String.
It should return a clone of the `time` field from
the struct.
 
In `main`, create a ChatMessage with `content` set to a
string slice.
 
Create another ChatMessage with `content` set to a String.
 
Create another ChatMessage with `content' set to a
DigitalContent variant.
 
Invoke the `consume_entertainment` method on the
ChatMessage storing a DigitalContent enum.
 
Invoke the `retrieve_time` method on all 3 ChatMessage
instances and print out each String's content.
*/

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String{
        return String::clone(&self.time);
    }
}

fn main() {
    let chat01: ChatMessage<&str> = ChatMessage { content: "chat message 01", time: String::from("time 01") };
    let chat02: ChatMessage<String> = ChatMessage { content: String::from("chat message 02"), time: String::from("time 02") };
    let chat03 = ChatMessage {content: DigitalContent::AudioFile, time: String::from("time 03")};
    let chat04 = ChatMessage {content: DigitalContent::VideoFile, time: String::from("time 04")};

    chat03.consume_entertainment();
    chat04.consume_entertainment();

    println!("message 1: {}", chat01.retrieve_time());
    println!("message 2: {}", chat02.retrieve_time());
    println!("message 3: {}", chat03.retrieve_time());
    println!("message 4: {}", chat04.retrieve_time());
}
