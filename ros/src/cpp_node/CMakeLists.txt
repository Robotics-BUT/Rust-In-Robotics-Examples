cmake_minimum_required(VERSION 3.0.2)
project(cpp_node)

find_package(catkin REQUIRED COMPONENTS
  roscpp
  std_msgs
)

catkin_package(
#  INCLUDE_DIRS include
#  LIBRARIES cpp_node
#  CATKIN_DEPENDS roscpp std_msgs
#  DEPENDS system_lib
)

include_directories( ${catkin_INCLUDE_DIRS} )

add_executable(${PROJECT_NAME} src/main.cpp)
target_link_libraries(${PROJECT_NAME} ${catkin_LIBRARIES} )
