use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rand::prelude::*;

mod editor;
mod game;

const TIME_POINT: usize = 905;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    bpm: u16,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let game = serde_json::from_reader(fs::File::open(cli.input)?)?;
    let editor = convert(cli.bpm, game)?;
    serde_json::to_writer_pretty(
        fs::File::create(cli.output.unwrap_or_else(|| PathBuf::from("out.json")))?,
        &editor,
    )?;
    Ok(())
}

fn check_round(num: f64) -> Result<usize> {
    let round_num = num.round();
    if (round_num - num).abs() > 10f64.powi(-2) {
        return Err(anyhow!("cannot round. wrong value {}", num));
    }

    Ok(round_num as usize)
}

fn convert(bpm: u16, map: game::GameMap) -> Result<editor::EditorMap> {
    let delta = map
        .bar_line_list
        .get(1)
        .ok_or_else(|| anyhow!("music too short. Cannot calc"))?;
    let bpb = check_round(delta * bpm as f64 / 60.0).context("when parse bpb")? as u8;
    let mut rng = thread_rng();

    let mut next_id_map: HashMap<usize, editor::Slide> = HashMap::new();
    let mut slides = vec![];
    let mut notes = vec![];

    for (id, note_data) in map.note_data_list.iter().enumerate() {
        let timed_position = convert_to_timed_position(bpm, note_data)?;
        let note = match note_data.note_type {
            game::NoteType::Tap1 | game::NoteType::Tap2 => editor::Note::Single(timed_position),
            game::NoteType::ScratchRight | game::NoteType::ScratchLeft => {
                editor::Note::Flick(timed_position)
            }
            game::NoteType::Slide
            | game::NoteType::LongStart
            | game::NoteType::LongEnd
            | game::NoteType::StopStart
            | game::NoteType::StopEnd => {
                dbg!(note_data.note_type);
                let slide_id = if let Some(mut slide) = next_id_map.remove(&id) {
                    let id = slide.id;
                    slide.notes.push(timed_position.id);
                    if note_data.next_id != 0 {
                        next_id_map.insert(note_data.next_id, slide);
                    } else {
                        slides.push(slide)
                    }
                    id
                } else {
                    let id = rng.gen();
                    let slide = editor::Slide {
                        id,
                        notes: vec![timed_position.id],
                        flickend: false,
                    };
                    if note_data.next_id != 0 {
                        next_id_map.insert(note_data.next_id, slide);
                    } else {
                        slides.push(slide);
                    }
                    id
                };
                editor::Note::Slide(editor::SlideNote {
                    slide: slide_id,
                    timed: timed_position,
                })
            }
        };
        notes.push(note)
    }

    Ok(editor::EditorMap {
        timepoints: vec![editor::Timepoint {
            id: TIME_POINT,
            time: 0,
            bpm,
            bpb,
        }],
        slides,
        notes,
    })
}

fn convert_to_timed_position(
    bpm: u16,
    node_data: &game::NoteData,
) -> Result<editor::TimedPosition> {
    Ok(editor::TimedPosition {
        id: random(),
        timepoint: TIME_POINT,
        offset: time_to_offset(bpm, node_data.time)?,
        lane: node_data.lane_id,
    })
}

fn time_to_offset(bpm: u16, time: f64) -> Result<usize> {
    let tick = bpm as f64 * time * 48. / 60.;
    check_round(tick).context(format!("when parse time {}", time))
}
