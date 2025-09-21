project(achintee LANGUAGES C)

SET(SOURCES
        achintee.c
)

add_library(achintee SHARED ${SOURCES})


target_include_directories(achintee PUBLIC $ENV{JAVA_HOME}/include $ENV{JAVA_HOME}/include/win32 ${CMAKE_BINARY_DIR}/generated/jni)