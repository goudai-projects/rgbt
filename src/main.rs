use crate::options::BuildOptions;
use structopt::StructOpt;

pub mod options;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = options::Options::from_args();

    init_logger()?;

    match opts.build {
        BuildOptions::GoOpts(go) => go.run().await?,
        BuildOptions::DubboOpts(dubbo) => dubbo.run().await?,
        BuildOptions::SpringBootOpts(springboot) => springboot.run().await?,
        BuildOptions::TomcatOpts(tomcat) => tomcat.run().await?,
    }

    Ok(())
}

fn init_logger() -> anyhow::Result<()> {
    if std::env::var("RUST_BACKTRACE").is_ok() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    let level = if crate::options::OPTIONS.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        .format(|out, message, record| {
            let color_config = fern::colors::ColoredLevelConfig::new()
                .debug(fern::colors::Color::Magenta)
                .info(fern::colors::Color::Green)
                .warn(fern::colors::Color::Cyan);
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%F %H:%M:%S %:z"),
                color_config.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for(module_path!().splitn(2, "::").next().unwrap(), level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
