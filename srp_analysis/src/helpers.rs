use crate::common::*;

// Custom types to make it easier to read
type C = u32;
type L = f32;
type LTot = f32;

/* 1. Total load factor */
// Calculate WCET
pub fn wcet(task: &Task) -> C {
    task.trace.end - task.trace.start
}

// Calculate load factor of a task
pub fn load_factor(task: &Task) -> L {
    let wcet = wcet(task) as f32;
    let inter_arrival = task.inter_arrival as f32;

    wcet / inter_arrival
}

// Calculate the total load factor of all tasks
pub fn total_load_factor(tasks: &[Task]) -> LTot {
    let mut sum: f32 = 0.0;
    for task in tasks {
        sum += load_factor(task);
    }

    sum
}
