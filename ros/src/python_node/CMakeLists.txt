cmake_minimum_required(VERSION 3.0.2)
project(python_node)

find_package(
  catkin REQUIRED COMPONENTS rospy
)
catkin_python_setup()
catkin_package()

catkin_install_python(PROGRAMS bin/main DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION})