use rosrust;
use std::sync::Arc;

pub struct Controller{
    subscriber: Arc<rosrust::Subscriber>,
}


impl Controller  {

    pub fn new() -> Controller {

        let publisher = Arc::new(rosrust::publish("rust_to_cpp", 0).unwrap());
        let subscriber = Arc::new(rosrust::subscribe("python_to_rust", 1, move |v: rosrust_msg::std_msgs::Header| {
            publisher.send(v).unwrap();
        }).unwrap());

        Controller {
            subscriber,
        }
    }
}