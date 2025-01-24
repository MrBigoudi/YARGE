use tracing::field::Visit;

pub struct LoggerConsoleVisitor;

impl Visit for LoggerConsoleVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.record_debug(field, &value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.record_debug(field, &value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.record_debug(field, &value)
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.record_debug(field, &value)
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.record_debug(field, &value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.record_debug(field, &value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.record_debug(field, &value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        // If no field name given
        if field.name() == "message" {
            println!("{}", value)
        } else {
            println!("{}: {}", field.name(), value)
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        // If no field name given
        if field.name() == "message" {
            println!("{:?}", value)
        } else {
            println!("{:?}: {:?}", field.name(), value)
        }
    }
}
