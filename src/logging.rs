/* standard library imports */
use std::io::Write;

pub fn get_log_builder(format: Option<&'static str>) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();
    let log_format = format.unwrap_or("%Y-%m-%dT%H:%M:%S");
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format(log_format),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info);
    builder
}
