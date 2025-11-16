//! This example provides demostrations of building a Cargo only CXX-Qt application

// Use this crate to test that missing_docs works with our generated code for a Cargo only build
#![deny(missing_docs)]

/// A module for our Rust defined QObject
pub mod cxxqt_object;

// use cxx_qt::casting::Upcast;
// use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQmlEngine, QUrl};
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
//use std::pin::Pin;

fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qml/main.qml"));
    }

    if let Some(engine) = engine.as_mut() {
        // let engine: Pin<&mut QQmlEngine> = engine.upcast_pin();
        // Listen to a signal from the QML Engine
        engine
            .as_qqmlengine()
            .on_quit(|_| {
                println!("QML Quit!");
            })
            .release();
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
