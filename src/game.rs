use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameMap {
    #[serde(rename = "BarLineList")]
    pub bar_line_list: Vec<f64>,

    #[serde(rename = "SoflanDataList")]
    pub soflan_data_list: Vec<SoflanData>,

    #[serde(rename = "NoteDataList")]
    pub note_data_list: Vec<NoteData>,
}

#[derive(Debug, Deserialize)]
pub struct SoflanData {
    #[serde(rename = "Time")]
    pub time: f64,

    #[serde(rename = "TimeScale")]
    pub time_scale: i8,

    #[serde(rename = "LeftRight")]
    pub left_right: u8,
}

#[derive(Debug, Deserialize)]
pub struct NoteData {
    #[serde(flatten)]
    pub note_type: NoteType,

    #[serde(rename = "LaneId")]
    pub lane_id: u8,

    #[serde(rename = "Time")]
    pub time: f64,

    #[serde(rename = "NextId")]
    pub next_id: usize,

    #[serde(rename = "Direction")]
    pub direction: i8,

    #[serde(rename = "EffectType")]
    pub effect_type: u8,

    #[serde(rename = "EffectParameter")]
    effect_parameter: u8,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(tag = "Type")]
pub enum NoteType {
    Tap1,

    Tap2,

    Slide,

    LongStart,

    LongEnd,

    ScratchRight,

    ScratchLeft,

    StopStart,

    StopEnd,
}
