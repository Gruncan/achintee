#include <jni.h>
#include <stdio.h>
#include "dev_gruncan_Achintee.h"

JNIEXPORT void JNICALL Java_dev_gruncan_Achintee_get_1os_1name(JNIEnv* env, jobject thisObj) {
   printf("Hello World!\n");
   return;
}