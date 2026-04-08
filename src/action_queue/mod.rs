use bevy::prelude::Resource;

#[derive(Debug, Clone, PartialEq)]
pub enum QueuedAction {
    Travel { to_province: u32 },
    Idle,
}

#[derive(Resource)]
pub struct WeekQueue {
    pub days: [QueuedAction; 7],
    pub current_day: usize,
}

impl Default for WeekQueue {
    fn default() -> Self {
        Self {
            days: std::array::from_fn(|_| QueuedAction::Idle),
            current_day: 0,
        }
    }
}

impl WeekQueue {
    pub fn push_action(&mut self, action: QueuedAction) {
        if self.current_day >= 7 {
            return;
        }
        self.days[self.current_day] = action;
    }
}
