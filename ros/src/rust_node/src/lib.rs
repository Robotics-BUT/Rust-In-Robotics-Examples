use rosrust;
use rosrust::Message;

pub struct Controller<T>{
    subscriber: subscriber<T>,
    publisher: publisher<T>,
}


impl<T> Controller<T> where T: Message {

    pub fn new() -> Controller<T> {

        let publisher = rosrust::publish("rust_to_cpp", 0).unwrap();
        let subscriber = rosrust::subscribe("python_to_rust", 1, move |v: rosrust_msg::std_msgs::Header| {
            rosrust::ros_info!("Received: {}", v.seq);
            publisher.send(v).unwrap();
        });

        Controller {
            subscriber,
            publisher
        }
    }
}