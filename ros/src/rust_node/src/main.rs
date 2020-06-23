use rosrust;
extern crate rust_node;

fn main() {

    rosrust::init("rust_node");

    let publisher = rosrust::publish("rust_to_cpp", 0).unwrap();
    let subscriber_info = rosrust::subscribe("python_to_rust", 1, move |v: rosrust_msg::std_msgs::Header| {
        rosrust::ros_info!("Received: {}", v.seq);
        publisher.send(v).unwrap();
    })
        .unwrap();

    let controller = rust_node::Controller::new();

    let a = publisher;

    rosrust::spin();
}