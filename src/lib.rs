#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn android_main(app: i_slint_backend_android_activity::android_activity::AndroidApp) {
    slint::android::init(app).unwrap();
    run_app();
}