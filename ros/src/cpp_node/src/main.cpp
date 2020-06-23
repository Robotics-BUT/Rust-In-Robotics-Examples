#include <ros/ros.h>
#include <std_msgs/Header.h>

#include <sstream>

class Controller {

public:

    Controller(ros::NodeHandle& node)
    : node_{node} {
        publisher_ = node_.advertise<std_msgs::Header>("cpp_to_python", 0);
        subscriber_ = node_.subscribe("rust_to_cpp", 1 , &Controller::subscriber_callback, this);
        timer_ = node_.createTimer(ros::Duration(0.1), &Controller::timer_callback, this);
    }

private:

    void subscriber_callback (const std_msgs::Header& msg) const {

        auto delay = ros::Time::now() - msg.stamp;
        std::cout << "Message delay: " << delay << " sec" << std::endl;
    }

    void timer_callback (const ros::TimerEvent& event) const {

        std_msgs::Header msg;
        msg.frame_id = "map";
        msg.stamp = ros::Time::now();
        publisher_.publish(msg);
    }

    ros::NodeHandle& node_;

    ros::Publisher publisher_;
    ros::Subscriber subscriber_;
    ros::Timer timer_;
};

int main(int argc, char **argv) {

    ros::init(argc, argv, "cpp_node");
    ros::NodeHandle n;

    auto controller = Controller(n);

    ros::spin();
    return 0;
}