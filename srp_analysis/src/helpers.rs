use crate::common::*;

/* 1. Total load factor */
// Calculate WCET
pub fn wcet(task: &Task) -> u32 {
    task.trace.end - task.trace.start
}

// Calculate load factor of a task
pub fn load_factor(task: &Task) -> f32 {
    let wcet = wcet(task) as f32;
    let inter_arrival = task.inter_arrival as f32;

    wcet / inter_arrival
}

// Calculate the total load factor of all tasks
pub fn total_load_factor(tasks: &[Task]) -> f32 {
    let mut sum: f32 = 0.0;
    for task in tasks {
        sum += load_factor(task);
    }

    sum
}
