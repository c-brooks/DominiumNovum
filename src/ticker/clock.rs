use bevy::ecs::{
    message::{Message, MessageWriter},
    resource::Resource,
    system::ResMut,
};

#[derive(Message, Debug, Clone)]
pub struct DayEnded {
    pub day: u32,
    pub week: u32,
    pub day_of_week: u8, // 0-6, useful for weekly triggers
}

#[derive(Message, Debug, Clone)]
pub struct WeekEnded {
    pub week: u32,
}

#[derive(Resource, Default)]
pub struct GameClock {
    pub day: u32,
    pub week: u32,
    pub day_of_week: u8, // 0-6
    pub paused: bool,
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, PartialEq, Default)]
pub enum ExecutionMode {
    #[default]
    Paused,
    Stepping, // advance one day at a time (player pressed Step)
    Playing,  // running through the week automatically
}

impl GameClock {
    pub fn year(&self) -> u32 {
        2080 + self.day / 365
    }
    pub fn month(&self) -> u32 {
        (self.day % 365) / 30
    }
}

pub fn advance_clock(
    mut clock: ResMut<GameClock>,
    mut day_events: MessageWriter<DayEnded>,
    mut week_events: MessageWriter<WeekEnded>,
) {
    match clock.execution_mode {
        ExecutionMode::Paused => return,
        ExecutionMode::Stepping => {
            // Advance exactly one day then return to planning
            clock.execution_mode = ExecutionMode::Paused;
        }
        ExecutionMode::Playing => {}
    }

    // Capture before incrementing so DayEnded refers to the day that just ran
    let current_day_of_week = clock.day_of_week;
    clock.day += 1;
    clock.day_of_week = (current_day_of_week + 1) % 7;

    if clock.day_of_week == 0 {
        clock.week += 1;
        week_events.write(WeekEnded { week: clock.week });
        clock.execution_mode = ExecutionMode::Paused;
    }

    day_events.write(DayEnded {
        day: clock.day,
        week: clock.week,
        day_of_week: current_day_of_week,
    });
}
