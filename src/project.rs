// auto-generated (with some tweaks) thanks to https://github.com/lumeohq/xsd-parser-rs

use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Project {
    pub application: Application,
    pub transport: Option<Transport>,
    pub structure: Option<project::StructureType>,
    pub arrangement: Option<Arrangement>,
    pub scenes: Option<project::ScenesType>,

    #[serde(rename = "@version")]
    pub version: String,
}

pub mod project {
    use super::*;

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    pub struct StructureType {
        #[serde(rename = "$value")]
        choices: Vec<StructureChoice>,
    }

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum StructureChoice {
        Track(Track),
        Channel(Channel),
        __Unknown__(String),
    }

    impl Default for StructureChoice {
        fn default() -> StructureChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ScenesType {
        pub scene: Option<Vec<Scene>>,
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "@name")]
    pub name: String,

    #[serde(rename = "@version")]
    pub version: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transport {
    pub tempo: Option<RealParameter>,
    pub time_signature: Option<TimeSignatureParameter>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct RealParameter {
    #[serde(rename = "@max")]
    pub max: Option<String>,

    #[serde(rename = "@min")]
    pub min: Option<String>,

    #[serde(rename = "@unit")]
    pub unit: Unit,

    #[serde(rename = "@value")]
    pub value: Option<String>,

    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,

    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Referenceable {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Nameable {
    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct BoolParameter {
    #[serde(rename = "@value")]
    pub value: Option<bool>,

    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,

    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct IntegerParameter {
    #[serde(rename = "@max")]
    pub max: Option<i32>,

    #[serde(rename = "@min")]
    pub min: Option<i32>,

    #[serde(rename = "@value")]
    pub value: Option<i32>,

    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,

    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EnumParameter {
    #[serde(rename = "@count")]
    pub count: i32,

    #[serde(rename = "@labels")]
    pub labels: Option<String>,

    #[serde(rename = "@value")]
    pub value: Option<i32>,

    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,

    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimeSignatureParameter {
    #[serde(rename = "@denominator")]
    pub denominator: i32,

    #[serde(rename = "@numerator")]
    pub numerator: i32,

    #[serde(rename = "@parameterID")]
    pub parameter_id: Option<i32>,

    #[serde(rename = "@id")]
    pub id: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Lane {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Arrangement {
    pub lanes: Option<Lanes>,
    pub markers: Option<Markers>,
    pub tempo_automation: Option<Points>,
    pub time_signature_automation: Option<Points>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Lanes {
    #[serde(rename = "$value")]
    pub lanes_choice: Vec<lanes::LanesChoice>,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,
}

pub mod lanes {
    use super::*;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum LanesChoice {
        Timeline(Timeline),
        Lanes(Box<Lanes>),
        Notes(Notes),
        Clips(Clips),
        ClipSlot(Box<ClipSlot>),
        #[serde(rename = "markers")]
        Markers(Markers),
        Warps(Box<Warps>),
        Audio(Audio),
        Video(Video),
        Points(Points),
        __Unknown__(String),
    }

    impl Default for LanesChoice {
        fn default() -> LanesChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Timeline {
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Track {
    pub channel: Option<Channel>,
    pub track: Option<Vec<Track>>,

    #[serde(rename = "@contentType")]
    pub content_type: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Channel {
    pub devices: Option<channel::DevicesType>,
    pub mute: Option<BoolParameter>,
    pub pan: Option<RealParameter>,
    pub sends: Option<channel::SendsType>,
    pub volume: Option<RealParameter>,

    #[serde(rename = "@audioChannels")]
    pub audio_channels: Option<i32>,

    #[serde(rename = "@destination")]
    pub destination: Option<String>,

    #[serde(rename = "@role")]
    pub role: Option<MixerRole>,

    #[serde(rename = "@solo")]
    pub solo: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

pub mod channel {
    use super::*;

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    pub struct DevicesType {
        #[serde(rename = "$value")]
        pub devices_choice: Vec<devices_type::DevicesChoice>,
    }

    pub mod devices_type {
        use super::*;

        #[derive(PartialEq, Debug, Serialize, Deserialize)]
        pub enum DevicesChoice {
            Device(Device),
            Vst2Plugin(Vst2Plugin),
            Vst3Plugin(Vst3Plugin),
            ClapPlugin(ClapPlugin),
            BuiltinDevice(BuiltinDevice),
            Equalizer(Equalizer),
            Compressor(Compressor),
            NoiseGate(NoiseGate),
            Limiter(Limiter),
            AuPlugin(AuPlugin),
            __Unknown__(String),
        }

        impl Default for DevicesChoice {
            fn default() -> DevicesChoice {
                Self::__Unknown__("No valid variants".into())
            }
        }
    }

    #[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SendsType {
        pub send: Vec<Send>,
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Device {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct FileReference {
    #[serde(rename = "@path")]
    pub path: String,

    #[serde(rename = "@external")]
    pub external: Option<bool>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Vst2Plugin {
    #[serde(rename = "@pluginVersion")]
    pub plugin_version: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Plugin {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@pluginVersion")]
    pub plugin_version: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vst3Plugin {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@pluginVersion")]
    pub plugin_version: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClapPlugin {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@pluginVersion")]
    pub plugin_version: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuiltinDevice {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Equalizer {
    pub band: Vec<EqBand>,
    pub input_gain: Option<RealParameter>,
    pub output_gain: Option<RealParameter>,
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EqBand {
    pub freq: RealParameter,
    pub gain: Option<RealParameter>,
    pub q: Option<RealParameter>,
    pub enabled: Option<BoolParameter>,

    #[serde(rename = "@type")]
    pub _type: EqBandType,

    #[serde(rename = "@order")]
    pub order: Option<i32>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Compressor {
    pub attack: Option<RealParameter>,
    pub auto_makeup: Option<BoolParameter>,
    pub input_gain: Option<RealParameter>,
    pub output_gain: Option<RealParameter>,
    pub ratio: Option<RealParameter>,
    pub release: Option<RealParameter>,
    pub threshold: Option<RealParameter>,
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NoiseGate {
    pub attack: Option<RealParameter>,
    pub range: Option<RealParameter>,
    pub ratio: Option<RealParameter>,
    pub release: Option<RealParameter>,
    pub threshold: Option<RealParameter>,
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Limiter {
    pub attack: Option<RealParameter>,
    pub input_gain: Option<RealParameter>,
    pub output_gain: Option<RealParameter>,
    pub release: Option<RealParameter>,
    pub threshold: Option<RealParameter>,
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuPlugin {
    pub parameters: Option<ParametersType>,
    pub enabled: Option<BoolParameter>,
    pub state: Option<FileReference>,

    #[serde(rename = "@deviceID")]
    pub device_id: Option<String>,

    #[serde(rename = "@deviceName")]
    pub device_name: String,

    #[serde(rename = "@deviceRole")]
    pub device_role: DeviceRole,

    #[serde(rename = "@deviceVendor")]
    pub device_vendor: Option<String>,

    #[serde(rename = "@loaded")]
    pub loaded: Option<bool>,

    #[serde(rename = "@pluginVersion")]
    pub plugin_version: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Send {
    pub pan: Option<RealParameter>,
    pub volume: RealParameter,
    pub destination: Option<String>,

    #[serde(rename = "@type")]
    pub _type: Option<SendType>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "@time")]
    pub time: String,

    #[serde(rename = "@duration")]
    pub duration: String,

    #[serde(rename = "@channel")]
    pub channel: i32,

    #[serde(rename = "@key")]
    pub key: i32,

    #[serde(rename = "@vel")]
    pub vel: Option<String>,

    #[serde(rename = "@rel")]
    pub rel: Option<String>,

    #[serde(rename = "$value")]
    pub note_choice: Option<Vec<note::NoteChoice>>,
}

pub mod note {
    use super::*;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum NoteChoice {
        Timeline(Timeline),
        Lanes(Lanes),
        Notes(Notes),
        Clips(Clips),
        ClipSlot(ClipSlot),
        #[serde(rename = "markers")]
        Markers(Markers),
        Warps(Warps),
        Audio(Audio),
        Video(Video),
        Points(Points),
        __Unknown__(String),
    }

    impl Default for NoteChoice {
        fn default() -> NoteChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Notes {
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub note: Vec<Note>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Clip {
    #[serde(rename = "@time")]
    pub time: f64,

    #[serde(rename = "@duration")]
    pub duration: Option<f64>,

    #[serde(rename = "@contentTimeUnit")]
    pub content_time_unit: Option<TimeUnit>,

    #[serde(rename = "@playStart")]
    pub play_start: Option<f64>,

    #[serde(rename = "@playStop")]
    pub play_stop: Option<f64>,

    #[serde(rename = "@loopStart")]
    pub loop_start: Option<f64>,

    #[serde(rename = "@loopEnd")]
    pub loop_end: Option<f64>,

    #[serde(rename = "@fadeTimeUnit")]
    pub fade_time_unit: Option<TimeUnit>,

    #[serde(rename = "@fadeInTime")]
    pub fade_in_time: Option<f64>,

    #[serde(rename = "@fadeOutTime")]
    pub fade_out_time: Option<f64>,

    #[serde(rename = "@reference")]
    pub reference: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub clip_choice: Vec<clip::ClipChoice>,
}

pub mod clip {
    use super::*;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum ClipChoice {
        Timeline(Timeline),
        Lanes(Lanes),
        Notes(Notes),
        Clips(Clips),
        ClipSlot(Box<ClipSlot>),
        #[serde(rename = "markers")]
        Markers(Markers),
        Warps(Box<Warps>),
        Audio(Audio),
        Video(Video),
        Points(Points),
        __Unknown__(String),
    }

    impl Default for ClipChoice {
        fn default() -> ClipChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Clips {
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub clip: Option<Vec<Clip>>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ClipSlot {
    #[serde(rename = "@hasStop")]
    pub has_stop: Option<bool>,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub clip: Option<Clip>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Marker {
    #[serde(rename = "@time")]
    pub time: f64,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Markers {
    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub marker: Vec<Marker>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Warps {
    #[serde(rename = "@contentTimeUnit")]
    pub content_time_unit: TimeUnit,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub warps_choice: Vec<warps::WarpsChoice>,
}

pub mod warps {
    use super::*;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum WarpsChoice {
        Timeline(Timeline),
        Lanes(Lanes),
        Notes(Notes),
        Clips(Clips),
        ClipSlot(ClipSlot),
        #[serde(rename = "markers")]
        Markers(Markers),
        Warps(Box<Warps>),
        Warp(Warp),
        Audio(Audio),
        Video(Video),
        Points(Points),
        __Unknown__(String),
    }

    impl Default for WarpsChoice {
        fn default() -> WarpsChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Warp {
    #[serde(rename = "@time")]
    pub time: f64,

    #[serde(rename = "@contentTime")]
    pub content_time: f64,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Audio {
    pub file: FileReference,

    #[serde(rename = "@algorithm")]
    pub algorithm: Option<String>,

    #[serde(rename = "@channels")]
    pub channels: i32,

    #[serde(rename = "@sampleRate")]
    pub sample_rate: i32,

    #[serde(rename = "@duration")]
    pub duration: f64,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediaFile {
    #[serde(rename = "File")]
    pub file: FileReference,

    #[serde(rename = "@duration")]
    pub duration: f64,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Video {
    pub file: FileReference,

    #[serde(rename = "@algorithm")]
    pub algorithm: Option<String>,

    #[serde(rename = "@channels")]
    pub channels: i32,

    #[serde(rename = "@sampleRate")]
    pub sample_rate: i32,

    #[serde(rename = "@duration")]
    pub duration: f64,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct RealPoint {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(rename = "@interpolation")]
    pub interpolation: Option<Interpolation>,

    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct EnumPoint {
    #[serde(rename = "@value")]
    pub value: i32,

    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct BoolPoint {
    #[serde(rename = "@value")]
    pub value: bool,

    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct IntegerPoint {
    #[serde(rename = "@value")]
    pub value: i32,

    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimeSignaturePoint {
    #[serde(rename = "@numerator")]
    pub numerator: i32,

    #[serde(rename = "@denominator")]
    pub denominator: i32,

    #[serde(rename = "@time")]
    pub time: String,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Points {
    pub target: AutomationTarget,

    #[serde(rename = "@unit")]
    pub unit: Option<Unit>,

    #[serde(rename = "@timeUnit")]
    pub time_unit: Option<TimeUnit>,

    #[serde(rename = "@track")]
    pub track: Option<String>,

    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,

    #[serde(rename = "$value")]
    pub points_choice: Vec<points::PointsChoice>,
}

pub mod points {
    use super::*;

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum PointsChoice {
        Point(Point),
        RealPoint(RealPoint),
        EnumPoint(EnumPoint),
        BoolPoint(BoolPoint),
        IntegerPoint(IntegerPoint),
        TimeSignaturePoint(TimeSignaturePoint),
        __Unknown__(String),
    }

    impl Default for PointsChoice {
        fn default() -> PointsChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct AutomationTarget {
    #[serde(rename = "@parameter")]
    pub parameter: Option<String>,

    #[serde(rename = "@expression")]
    pub expression: Option<ExpressionType>,

    #[serde(rename = "@channel")]
    pub channel: Option<i32>,

    #[serde(rename = "@key")]
    pub key: Option<i32>,

    #[serde(rename = "@controller")]
    pub controller: Option<i32>,
}

#[derive(Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Scene {
    #[serde(rename = "@id")]
    pub id: Option<String>,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@color")]
    pub color: Option<String>,

    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Unit {
    Linear,
    Normalized,
    Percent,
    Decibel,
    Hertz,
    Semitones,
    Seconds,
    Beats,
    Bpm,
    __Unknown__(String),
}

impl Default for Unit {
    fn default() -> Unit {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeUnit {
    Beats,
    Seconds,
    __Unknown__(String),
}

impl Default for TimeUnit {
    fn default() -> TimeUnit {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceRole {
    Instrument,
    NoteFX,
    AudioFX,
    Analyzer,
    __Unknown__(String),
}

impl Default for DeviceRole {
    fn default() -> DeviceRole {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EqBandType {
    HighPass,
    LowPass,
    BandPass,
    HighShelf,
    LowShelf,
    Bell,
    Notch,
    __Unknown__(String),
}

impl Default for EqBandType {
    fn default() -> EqBandType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MixerRole {
    Regular,
    Master,
    Effect,
    Submix,
    Vca,
    __Unknown__(String),
}

impl Default for MixerRole {
    fn default() -> MixerRole {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SendType {
    Pre,
    Post,
    __Unknown__(String),
}

impl Default for SendType {
    fn default() -> SendType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    Audio,
    Automation,
    Notes,
    Video,
    Markers,
    Tracks,
    __Unknown__(String),
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Interpolation {
    Hold,
    Linear,
    __Unknown__(String),
}

impl Default for Interpolation {
    fn default() -> Interpolation {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionType {
    Gain,
    Pan,
    Transpose,
    Timbre,
    Formant,
    Pressure,
    ChannelController,
    ChannelPressure,
    PolyPressure,
    PitchBend,
    ProgramChange,
    __Unknown__(String),
}

impl Default for ExpressionType {
    fn default() -> ExpressionType {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct ParametersType {
    #[serde(rename = "$value")]
    pub parameters_choice: Option<Vec<ParametersChoice>>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]

pub enum ParametersChoice {
    #[serde(rename = "parameter")]
    Parameter(Parameter),
    RealParameter(RealParameter),
    BoolParameter(BoolParameter),
    IntegerParameter(IntegerParameter),
    EnumParameter(EnumParameter),
    TimeSignatureParameter(TimeSignatureParameter),
    __Unknown__(String),
}

impl Default for ParametersChoice {
    fn default() -> ParametersChoice {
        Self::__Unknown__("No valid variants".into())
    }
}
