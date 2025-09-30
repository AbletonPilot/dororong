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
const DANCE_FRAME_STR: [&str; 13] = [
    include_str!("../frames/dance/dororong-dance-1.txt"),
    include_str!("../frames/dance/dororong-dance-2.txt"),
    include_str!("../frames/dance/dororong-dance-3.txt"),
    include_str!("../frames/dance/dororong-dance-4.txt"),
    include_str!("../frames/dance/dororong-dance-5.txt"),
    include_str!("../frames/dance/dororong-dance-6.txt"),
    include_str!("../frames/dance/dororong-dance-7.txt"),
    include_str!("../frames/dance/dororong-dance-8.txt"),
    include_str!("../frames/dance/dororong-dance-9.txt"),
    include_str!("../frames/dance/dororong-dance-10.txt"),
    include_str!("../frames/dance/dororong-dance-11.txt"),
    include_str!("../frames/dance/dororong-dance-12.txt"),
    include_str!("../frames/dance/dororong-dance-13.txt"),
];
const FRONTBACK_FRAME_STR: [&str; 4] = [
    include_str!("../frames/frontback/dororong-frontback-1.txt"),
    include_str!("../frames/frontback/dororong-frontback-2.txt"),
    include_str!("../frames/frontback/dororong-frontback-3.txt"),
    include_str!("../frames/frontback/dororong-frontback-4.txt"),
];
const UPDOWN_FRAME_STR: [&str; 15] = [
    include_str!("../frames/updown/dororong-updown-1.txt"),
    include_str!("../frames/updown/dororong-updown-2.txt"),
    include_str!("../frames/updown/dororong-updown-3.txt"),
    include_str!("../frames/updown/dororong-updown-4.txt"),
    include_str!("../frames/updown/dororong-updown-5.txt"),
    include_str!("../frames/updown/dororong-updown-6.txt"),
    include_str!("../frames/updown/dororong-updown-7.txt"),
    include_str!("../frames/updown/dororong-updown-8.txt"),
    include_str!("../frames/updown/dororong-updown-9.txt"),
    include_str!("../frames/updown/dororong-updown-10.txt"),
    include_str!("../frames/updown/dororong-updown-11.txt"),
    include_str!("../frames/updown/dororong-updown-12.txt"),
    include_str!("../frames/updown/dororong-updown-13.txt"),
    include_str!("../frames/updown/dororong-updown-14.txt"),
    include_str!("../frames/updown/dororong-updown-15.txt"),
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
        if self.current_frame > max_index {
            return None;
        }
        let frame = self.frames[self.current_frame].clone();
        let interval = self.interval_ms[self.current_frame];
        self.current_frame += 1;
        Some((frame, interval))
    }
}

fn create_frame_from_str(frame_str: &'static str) -> Frame {
    Frame {
        lines: frame_str
            .lines()
            .map(|line| line.strip_suffix('\r').unwrap_or(line))
            .collect(),
    }
}

fn create_animated_frames(
    frame_strs: &[&'static str],
    frame_order: &[usize],
    intervals: &[u64],
) -> AnimatedFrames {
    let frames: Vec<Frame> = frame_strs
        .iter()
        .map(|s| create_frame_from_str(s))
        .collect();
    let ordered_frames: Vec<Frame> = frame_order.iter().map(|&i| frames[i].clone()).collect();
    AnimatedFrames {
        frames: Arc::from(ordered_frames.into_boxed_slice()),
        interval_ms: Arc::from(intervals.to_vec().into_boxed_slice()),
    }
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = create_frame_from_str(STATIC_FRAME_STR);

    // Bokbok animations (normal and fast)
    pub static ref BOKBOK_FRAMES: AnimatedFrames = create_animated_frames(
        &BOKBOK_FRAME_STR,
        &[0, 1, 2, 3],
        &[60, 60, 60, 60],
    );

    pub static ref BOKBOK_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &BOKBOK_FRAME_STR,
        &[0, 1, 2, 3],
        &[30, 30, 30, 30],
    );

    // Pangpang animations (normal and fast)
    pub static ref PANGPANG_FRAMES: AnimatedFrames = create_animated_frames(
        &PANGPANG_FRAME_STR,
        &[0, 1, 2],
        &[60, 60, 60],
    );

    pub static ref PANGPANG_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &PANGPANG_FRAME_STR,
        &[0, 1, 2],
        &[30, 30, 30],
    );

    // Run animations (normal and fast)
    pub static ref RUN_FRAMES: AnimatedFrames = create_animated_frames(
        &RUN_FRAMES_STR,
        &[0, 1, 2, 3, 4, 5, 6],
        &[60, 60, 60, 60, 60, 60, 60],
    );

    pub static ref RUN_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &RUN_FRAMES_STR,
        &[0, 1, 2, 3, 4, 5, 6],
        &[30, 30, 30, 30, 30, 30, 30],
    );

    // Dance animations (normal and fast)
    pub static ref DANCE_FRAMES: AnimatedFrames = create_animated_frames(
        &DANCE_FRAME_STR,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100],
    );

    pub static ref DANCE_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &DANCE_FRAME_STR,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50],
    );

    // Frontback animations (normal and fast)
    pub static ref FRONTBACK_FRAMES: AnimatedFrames = create_animated_frames(
        &FRONTBACK_FRAME_STR,
        &[0, 1, 2, 3],
        &[60, 60, 60, 60],
    );

    pub static ref FRONTBACK_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &FRONTBACK_FRAME_STR,
        &[0, 1, 2, 3],
        &[30, 30, 30, 30],
    );

    // Updown animations (normal and fast)
    pub static ref UPDOWN_FRAMES: AnimatedFrames = create_animated_frames(
        &UPDOWN_FRAME_STR,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        &[60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60],
    );

    pub static ref UPDOWN_FRAMES_FAST: AnimatedFrames = create_animated_frames(
        &UPDOWN_FRAME_STR,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        &[30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30],
    );
}
