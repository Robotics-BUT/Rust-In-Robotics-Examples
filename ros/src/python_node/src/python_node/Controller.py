import rospy
from std_msgs.msg import Header


class Controller:

    def __init__(self):
        self.pub = rospy.Publisher('python_to_rust', Header, queue_size=0)
        self.sub = rospy.Subscriber('cpp_to_python', Header, self.subscriber_callback)

    def subscriber_callback(self, msg):
        self.pub.publish(msg)
