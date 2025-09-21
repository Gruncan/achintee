use jni::JNIEnv;
use jni::objects::JObject;

#[allow(non_snake_case)]
#[unsafe (no_mangle)]
pub extern "system" fn Java_dev_gruncan_Achintee_get_1os_1name(env: JNIEnv, obj: JObject) {
    println!("Hello from rust!")
}