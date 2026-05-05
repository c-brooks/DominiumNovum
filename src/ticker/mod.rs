pub mod clock;

use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::SystemSet,
    prelude::IntoScheduleConfigs,
};

use clock::{GameClock, advance_clock};

use crate::ticker::clock::{DayEnded, WeekEnded};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DailyTickSet {
    Clock,      // 1. advance time, fire events
    Production, // 2. buildings produce resources
}

pub struct TickerPlugin;

impl Plugin for TickerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameClock>()
            .add_message::<DayEnded>()
            .add_message::<WeekEnded>()
            .configure_sets(
                Update,
                (DailyTickSet::Clock, DailyTickSet::Production).chain(),
            )
            .add_systems(Update, advance_clock.in_set(DailyTickSet::Clock));
    }
}
