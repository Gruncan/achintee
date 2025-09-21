package dev.gruncan;

public class Achintee {


    static {
        System.loadLibrary("achintee");
    }


    public Achintee() {

    }

    public native void get_os_name();
}
