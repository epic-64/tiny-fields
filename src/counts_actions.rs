use crate::game::Progress;

pub struct CountsActions {
    pub level: i64,
    pub actions_done_current_level: i64,
    pub actions_done_total: i64,
    pub level_up_progress: Progress,
    pub actions_to_reach_level: fn (i64) -> i64,
    pub actions_flat: i64,
}

impl CountsActions {
    pub fn new(actions_to_reach_level: fn (i64) -> i64, actions_flat: i64) -> Self {
        Self {
            level: 1,
            actions_done_current_level: 0,
            actions_done_total: 0,
            level_up_progress: Progress::new(),
            actions_to_reach_level,
            actions_flat,
        }
    }

    pub fn actions_to_next_level(&self) -> i64 {
        self.actions_flat
            + (self.actions_to_reach_level)(self.level + 1)
            - (self.actions_to_reach_level)(self.level)
    }

    pub fn increment_actions(&mut self) {
        self.actions_done_total += 1;
        self.actions_done_current_level += 1;

        self.level_up_progress.set(
            self.actions_done_current_level as f64 / self.actions_to_next_level() as f64
        );

        if self.actions_done_current_level as f64 >= self.actions_to_next_level() as f64 {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.actions_done_current_level = 0;
        self.level_up_progress.reset();
    }
}