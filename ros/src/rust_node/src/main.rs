use rosrust;
extern crate rust_node;

fn main() {

    rosrust::init("rust_node");

    let controller = rust_node::Controller::new();

    rosrust::spin();
}