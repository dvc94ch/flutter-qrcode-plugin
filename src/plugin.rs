use crate::qrcode::QrCodeScanner;
use flutter_plugins::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const PLUGIN_NAME: &str = module_path!();
const CHANNEL_NAME: &str = "rust/qrcode";

#[derive(Default)]
pub struct QrCodePlugin {
    handler: Arc<RwLock<Handler>>,
}

#[derive(Default)]
struct Handler {
    handle: Option<Handle>,
    stop_trigger: Arc<AtomicBool>,
}

impl Plugin for QrCodePlugin {
    fn plugin_name() -> &'static str {
        PLUGIN_NAME
    }

    fn init_channels(&mut self, registrar: &mut ChannelRegistrar) {
        let event_handler = Arc::downgrade(&self.handler);
        registrar.register_channel(EventChannel::new(CHANNEL_NAME, event_handler));
    }
}

impl EventHandler for Handler {
    fn on_listen(
        &mut self,
        _value: Value,
        engine: FlutterEngine,
    ) -> Result<Value, MethodCallError> {
        if let Some(handle) = &self.handle {
            send_event(engine, Event::Initialized(handle.clone()))?;
            return Ok(Value::Null);
        }

        // create texture
        let texture = engine.create_texture();
        let texture_id = texture.id();

        // create scanner
        let mut scanner = QrCodeScanner::new(texture)?;
        let handle = Handle {
            texture_id,
            width: scanner.width() as _,
            height: scanner.height() as _,
        };
        self.handle = Some(handle.clone());
        send_event(engine.clone(), Event::Initialized(handle))?;

        let stop_trigger = Arc::new(AtomicBool::new(false));
        self.stop_trigger = stop_trigger.clone();
        engine.clone().run_in_background(async move {
            while !stop_trigger.load(Ordering::Relaxed) {
                match scanner.frame() {
                    Ok(Some(code)) => send_event(engine.clone(), Event::QrCode(code)).unwrap(),
                    Err(err) => send_error(engine.clone(), &err),
                    _ => {}
                }
            }
            drop(scanner);
            send_event(engine, Event::Disposed).unwrap();
        });
        Ok(Value::Null)
    }

    fn on_cancel(&mut self, _engine: FlutterEngine) -> Result<Value, MethodCallError> {
        self.stop_trigger.store(true, Ordering::Relaxed);
        self.handle = None;
        Ok(Value::Null)
    }
}

fn send_event(engine: FlutterEngine, event: Event) -> Result<(), MethodCallError> {
    log::debug!("event: {:?}", event);
    let value = to_value(event)?;
    engine.run_on_platform_thread(move |engine| {
        engine.with_channel(CHANNEL_NAME, |channel| {
            if let Some(channel) = channel.try_as_method_channel() {
                channel.send_success_event(&value);
            }
        });
    });
    Ok(())
}

fn send_error(engine: FlutterEngine, error: &dyn std::error::Error) {
    let message = format!("{}", error);
    log::error!("{}", &message);
    engine.run_on_platform_thread(move |engine| {
        engine.with_channel(CHANNEL_NAME, move |channel| {
            if let Some(channel) = channel.try_as_method_channel() {
                channel.send_error_event("", &message, &Value::Null);
            }
        });
    });
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Handle {
    texture_id: i64,
    width: i64,
    height: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum Event {
    Initialized(Handle),
    QrCode(String),
    Disposed,
}

#[derive(Debug)]
struct UninitializedError;

impl std::fmt::Display for UninitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "scanner was not initialized")
    }
}

impl std::error::Error for UninitializedError {}

impl From<UninitializedError> for MethodCallError {
    fn from(error: UninitializedError) -> Self {
        Self::from_error(error)
    }
}
