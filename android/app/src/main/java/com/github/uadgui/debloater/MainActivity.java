package com.github.uadgui.debloater;

import android.app.NativeActivity;
import android.os.Bundle;
import android.widget.Toast;

public class MainActivity extends NativeActivity {
    static {
        System.loadLibrary("uad_gui");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }
}
