use lazy_static::lazy_static;
use std::sync::Arc;

const STATIC_FRAME_STR: &str = include_str!("../frames/static/dororong-static.txt");
const PANGPANG_FRAME_STR: [&str; 3] = [
    include_str!("../frames/pangpang/dororong-pangpang-1.txt"),
    include_str!("../frames/pangpang/dororong-pangpang-2.txt"),
    include_str!("../frames/pangpang/dororong-pangpang-3.txt"),
];
const RUN_FRAMES_STR: [&str; 7] = [
    include_str!("../frames/run/dororong-run-1.txt"),
    include_str!("../frames/run/dororong-run-2.txt"),
    include_str!("../frames/run/dororong-run-3.txt"),
    include_str!("../frames/run/dororong-run-4.txt"),
    include_str!("../frames/run/dororong-run-5.txt"),
    include_str!("../frames/run/dororong-run-6.txt"),
    include_str!("../frames/run/dororong-run-7.txt"),
];
const BOKBOK_FRAME_STR: [&str; 4] = [
    include_str!("../frames/bokbok/dororong-bokbok-1.txt"),
    include_str!("../frames/bokbok/dororong-bokbok-2.txt"),
    include_str!("../frames/bokbok/dororong-bokbok-3.txt"),
    include_str!("../frames/bokbok/dororong-bokbok-4.txt"),
];

#[derive(Debug, Clone)]
pub struct Frame {
    pub lines: Arc<[&'static str]>,
}

#[derive(Debug, Clone)]
pub struct AnimatedFrames {
    pub frames: Arc<[Frame]>,
    pub interval_ms: Arc<[u64]>,
}

impl AnimatedFrames {
    pub fn iter(&self) -> AnimatedFramesIterator {
        AnimatedFramesIterator {
            frames: self.frames.clone(),
            interval_ms: self.interval_ms.clone(),
            current_frame: 0,
        }
    }
}

pub struct AnimatedFramesIterator {
    frames: Arc<[Frame]>,
    interval_ms: Arc<[u64]>,
    current_frame: usize,
}

impl Iterator for AnimatedFramesIterator {
    type Item = (Frame, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames.is_empty() || self.interval_ms.is_empty() {
            return None;
        }
        let max_index = self.frames.len().max(self.interval_ms.len()) - 1;
        if self.current_frame >= max_index {
            return None;
        }
        let frame = self.frames[self.current_frame].clone();
        let interval = self.interval_ms[self.current_frame];
        self.current_frame += 1;
        Some((frame, interval))
    }
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = Frame {
        lines: STATIC_FRAME_STR
            .lines()
            .map(|line| line.strip_suffix('\r').unwrap_or(line))
            .collect(),
    };
    
    // Bokbok animations (normal and fast)
    pub static ref BOKBOK_FRAMES: AnimatedFrames = {
        let frames = BOKBOK_FRAME_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[3].clone(),
                frames[2].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Arc::new([100, 75, 100, 75, 75, 100]),
        }
    };
    
    pub static ref BOKBOK_FRAMES_FAST: AnimatedFrames = {
        let frames = BOKBOK_FRAME_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[3].clone(),
                frames[2].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Arc::new([50, 40, 50, 40, 40, 50]),
        }
    };
    
    // Pangpang animations (normal and fast)
    pub static ref PANGPANG_FRAMES: AnimatedFrames = {
        let frames = PANGPANG_FRAME_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Arc::new([150, 100, 150, 100]),
        }
    };
    
    pub static ref PANGPANG_FRAMES_FAST: AnimatedFrames = {
        let frames = PANGPANG_FRAME_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[1].clone(),
            ]),
            interval_ms: Arc::new([75, 50, 75, 50]),
        }
    };
    
    // Run animations (normal and fast)
    pub static ref RUN_FRAMES: AnimatedFrames = {
        let frames = RUN_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[3].clone(),
                frames[4].clone(),
                frames[5].clone(),
                frames[6].clone(),
            ]),
            interval_ms: Arc::new([60, 60, 60, 60, 60, 60, 60]),
        }
    };
    
    pub static ref RUN_FRAMES_FAST: AnimatedFrames = {
        let frames = RUN_FRAMES_STR
            .iter()
            .map(|frame| Frame {
                lines: frame
                    .lines()
                    .map(|line| line.strip_suffix('\r').unwrap_or(line))
                    .collect(),
            })
            .collect::<Box<[Frame]>>();
        AnimatedFrames {
            frames: Arc::new([
                frames[0].clone(),
                frames[1].clone(),
                frames[2].clone(),
                frames[3].clone(),
                frames[4].clone(),
                frames[5].clone(),
                frames[6].clone(),
            ]),
            interval_ms: Arc::new([30, 30, 30, 30, 30, 30, 30]),
        }
    };
}

