use std::{fs, path::Path, time::SystemTime};

pub fn log_init() {
    // Log files for log rotation
    let log_path_0 = Path::new("game-0.log");
    let log_path_1 = Path::new("game-1.log");
    let log_path_2 = Path::new("game-2.log");
    let log_path_3 = Path::new("game-3.log");
    let log_path_4 = Path::new("game-4.log");
    if log_path_4.exists() {
        fs::remove_file(log_path_4).expect("failed removing game-4.log");
    }
    if log_path_3.exists() {
        fs::rename(log_path_3, log_path_4).expect("failed renaming game-3.log to game-4.log");
    }
    if log_path_2.exists() {
        fs::rename(log_path_2, log_path_3).expect("failed renaming game-2.log to game-3.log");
    }
    if log_path_1.exists() {
        fs::rename(log_path_1, log_path_2).expect("failed renaming game-1.log to game-2.log");
    }
    if log_path_0.exists() {
        fs::rename(log_path_0, log_path_1).expect("failed renaming game-0.log to game-1.log");
    }

    let default_log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };

    fern::Dispatch::new()
        // Setup formation
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(
            fern::Dispatch::new()
                // Default level to accept
                .level(default_log_level)
                // WGPU Overwrite
                .level_for("wgpu_core", log::LevelFilter::Info)
                .level_for("wgpu_hal", log::LevelFilter::Info)
                // Write to StdOut
                .chain(std::io::stdout())
                .chain(fern::log_file(log_path_0).expect("failed building file log")),
        )
        // Apply as global logger!
        .apply()
        .expect("failed building logger");

    log::info!(
        "Logger initialized at max level set to {}",
        log::max_level()
    );
}
