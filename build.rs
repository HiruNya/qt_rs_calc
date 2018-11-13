extern crate rust_qt_binding_generator;

fn main() {
    let out_dir = ::std::env::var("OUT_DIR").unwrap();
    rust_qt_binding_generator::build::Build::new(&out_dir)
        .bindings("qt/bindings.json")
        .qrc("qt/qml.qrc")
        .cpp("qt/main.cpp")
        .compile("qt_rs_calc");
}
