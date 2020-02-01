use flutter_qrcode_plugin::QrCodePlugin;
use flutter_winit::FlutterWindow;
use glutin::window::WindowBuilder;
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();

    let assets_dir = std::env::var("FLUTTER_ASSET_DIR").expect("FLUTTER_ASSET_DIR");

    let mut args = Vec::with_capacity(3);

    if let Ok(observatory_port) = std::env::var("DART_OBSERVATORY_PORT") {
        args.push("--disable-service-auth-codes".to_string());
        args.push(format!("--observatory-port={}", observatory_port));
    }

    if let Ok(snapshot) = std::env::var("FLUTTER_AOT_SNAPSHOT") {
        if Path::new(&snapshot).exists() {
            args.push(format!("--aot-shared-library-name={}", snapshot));
        }
    }

    let window = WindowBuilder::new().with_title("Flutter App Demo");
    let flutter = FlutterWindow::new(window, PathBuf::from(assets_dir)).unwrap();
    let flutter = flutter.with_resource_context().unwrap();

    flutter.add_plugin(QrCodePlugin::default());

    flutter.start_engine(&args).unwrap();

    flutter.run();
}
