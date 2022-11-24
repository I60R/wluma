use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

mod app;
mod file;
pub use app::*;

pub fn load() -> Result<app::Config, Box<dyn Error>> {
    validate(parse()?)
}

fn parse() -> Result<app::Config, toml::de::Error> {
    let file_config = dirs::config_dir()
        .and_then(|config_dir| fs::read_to_string(&config_dir.join("wluma/config.toml")).ok())
        .unwrap_or_else(|| include_str!("../../config.toml").to_string());

    let parse_als_thresholds = |t: HashMap<String, String>| -> HashMap<u64, String> {
        t.into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v))
            .collect()
    };

    toml::from_str(&file_config).map(|file_config: file::Config| app::Config {
        output: file_config
            .output
            .backlight
            .into_iter()
            .map(|o| {
                app::Output::Backlight(app::BacklightOutput {
                    name: o.name,
                    path: o.path,
                    min_brightness: 1,
                    capturer: match o.capturer {
                        file::Capturer::None => app::Capturer::None,
                        file::Capturer::Wlroots => app::Capturer::Wlroots,
                    },
                })
            })
            .chain(file_config.output.ddcutil.into_iter().map(|o| {
                app::Output::DdcUtil(app::DdcUtilOutput {
                    name: o.name,
                    min_brightness: 1,
                    capturer: match o.capturer {
                        file::Capturer::None => app::Capturer::None,
                        file::Capturer::Wlroots => app::Capturer::Wlroots,
                    },
                })
            }))
            .chain(file_config.keyboard.into_iter().map(|k| {
                app::Output::Backlight(app::BacklightOutput {
                    name: k.name,
                    path: k.path,
                    min_brightness: 0,
                    capturer: Capturer::None,
                })
            }))
            .collect(),

        als: match file_config.als {
            file::Als::Iio { path, thresholds } => app::Als::Iio {
                path,
                thresholds: parse_als_thresholds(thresholds),
            },
            file::Als::Webcam { video, thresholds } => app::Als::Webcam {
                video,
                thresholds: parse_als_thresholds(thresholds),
            },
            file::Als::Time { thresholds } => app::Als::Time {
                thresholds: parse_als_thresholds(thresholds),
            },
            file::Als::None => app::Als::None,
        },

        processor: match file_config.processor {
            file::Processor::OpenGL => Processor::OpenGL,
            file::Processor::Vulkan => Processor::Vulkan,
        },
    })
}

fn validate(config: app::Config) -> Result<app::Config, Box<dyn Error>> {
    let names = config
        .output
        .iter()
        .map(|output| match output {
            app::Output::Backlight(app::BacklightOutput { name, .. }) => name,
            app::Output::DdcUtil(DdcUtilOutput { name, .. }) => name,
        })
        .collect::<HashSet<_>>();

    match (names.len(), names.len() == config.output.len()) {
        (0, _) => Err("No output or keyboard configured".into()),
        (_, false) => Err("Names of all outputs and keyboards are not unique".into()),
        _ => Ok(config),
    }
}
