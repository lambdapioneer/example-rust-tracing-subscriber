use std::env;
use std::fmt::Debug;

use tracing::field::Field;
use tracing::{Event, Subscriber};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Layer};

/// A simple visitor that collects the message field of log messages.
struct SimpleMessageVisitor {
    message: Option<String>,
}

impl SimpleMessageVisitor {
    fn new() -> Self {
        SimpleMessageVisitor { message: None }
    }
}

impl tracing::field::Visit for SimpleMessageVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{:?}", value));
        }
    }
}

/// A simple subscriber layer that leverages `SimpleMessageVisitor` to collect the message fields
/// of the tracing messages.
pub struct CustomSubscriberLayer {}

impl<S> Layer<S> for CustomSubscriberLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
        // collect the message field using the visitor pattern (other metadata is available directly
        // from the `Event` type)
        let mut simple_message_visitor = SimpleMessageVisitor::new();
        event.record(&mut simple_message_visitor);

        // then process it further (e.g. forwarding to the platform specific log interface); here
        // we just print it to stdout
        println!(
            "The source '{:}' says on the '{:}' level: '{:}'",
            event.metadata().target(),
            event.metadata().level(),
            simple_message_visitor
                .message
                .unwrap_or_else(|| { "<empty>".to_string() })
        );
    }
}

#[tokio::main]
async fn main() {
    init_tracing();

    tracing::error!("Hic forum est.");
    tracing::info!("Populus properat.");
    tracing::debug!("In vino veritas.");
}

fn init_tracing() {
    // follow environment variable instructions if present
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "example_rust_tracing_subscriber=info");
    }

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        // re-create the default output to stderr
        .with(fmt::Layer::new().with_writer(std::io::stderr))
        // and then add our custom subscriber
        .with(CustomSubscriberLayer {});

    tracing::subscriber::set_global_default(subscriber).expect("failed setting global subscriber");
}
