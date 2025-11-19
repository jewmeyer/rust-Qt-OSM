// Derived from https://github.com/KDAB/cxx-qt/blob/main/examples/qml_minimal/rust/src/cxxqt_object.rs

/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }

    extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        #[namespace = "my_object"]
        type MyObject = super::MyObjectRust;

        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        #[cxx_name = "incrementNumber"]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        #[cxx_name = "sayHi"]
        fn say_hi(self: &MyObject, string: &QString, number: i32);

        #[qinvokable]
        #[cxx_name = "clickedInMap"]
        fn clicked_in_map(self: &MyObject, longitude: f32, latitude: f32);

        #[qinvokable]
        #[cxx_name = "onZoomLevelChanged"]
        fn on_zoom_level_changed(self: &MyObject, zoom_level: f32);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;

/// The Rust struct for the QObject
#[derive(Default)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}

impl qobject::MyObject {
    /// Increment the number Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }

    /// React to mouse click in map
    pub fn clicked_in_map(&self, longitude: f32, latitude: f32) {
        println!("Mouse click in map: longitude is {longitude}, latitude is {latitude}");
    }

    /// React to change of zoom level via mouse wheel or keyboard shortcuts
    pub fn on_zoom_level_changed(&self, zoom_level: f32) {
        println!("Zoom level changed to {zoom_level}");
    }
}
