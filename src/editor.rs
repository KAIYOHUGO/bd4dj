use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EditorMap {
    pub timepoints: Vec<Timepoint>,
    pub slides: Vec<Slide>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize)]
pub struct Timepoint {
    pub id: usize,
    pub time: usize,
    pub bpm: u16,
    pub bpb: u8,
}

#[derive(Debug, Serialize)]
pub struct Slide {
    pub id: usize,
    pub notes: Vec<usize>,
    pub flickend: bool,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Note {
    Single(TimedPosition),
    Slide(SlideNote),
    Flick(TimedPosition),
}

#[derive(Debug, Serialize)]
pub struct SlideNote {
    pub slide: usize,

    #[serde(flatten)]
    pub timed: TimedPosition,
}

#[derive(Debug, Serialize)]
pub struct TimedPosition {
    pub id: usize,
    pub timepoint: usize,
    pub offset: usize,
    pub lane: u8,
}
