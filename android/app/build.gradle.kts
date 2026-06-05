plugins {
    id("com.android.application")
}

android {
    namespace = "com.github.uadgui.debloater"
    compileSdk = 34

    defaultConfig {
        applicationId = "com.github.uadgui.debloater"
        minSdk = 26
        targetSdk = 34
        versionCode = 1
        versionName = "0.5.1"

        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64", "x86")
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation("androidx.appcompat:appcompat:1.6.1")
}
