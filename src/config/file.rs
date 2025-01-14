use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Capturer {
    Wlroots,
    None,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Processor {
    OpenGL,
    Vulkan,
}

impl Default for Processor {
    fn default() -> Self {
        Self::Vulkan
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Als {
    Iio {
        path: String,
        thresholds: HashMap<String, String>,
    },
    Time {
        thresholds: HashMap<String, String>,
    },
    Webcam {
        video: usize,
        thresholds: HashMap<String, String>,
    },
    None,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct OutputByType {
    pub backlight: Vec<BacklightOutput>,
    pub ddcutil: Vec<DdcUtilOutput>,
}

#[derive(Deserialize, Debug)]
pub struct BacklightOutput {
    pub name: String,
    pub path: String,
    pub capturer: Capturer,
}

#[derive(Deserialize, Debug)]
pub struct DdcUtilOutput {
    pub name: String,
    pub capturer: Capturer,
}

#[derive(Deserialize, Debug)]
pub struct Keyboard {
    pub name: String,
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub als: Als,
    #[serde(default)]
    pub output: OutputByType,
    #[serde(default)]
    pub keyboard: Vec<Keyboard>,
    #[serde(default)]
    pub processor: Processor,
}
