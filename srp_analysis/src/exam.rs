/* Home exam functions etc.*/
use crate::common::Task;

// 1. Total load factor
pub fn load_factor(task: &Task) -> f32 {
    let deadline = task.deadline as f32;
    let inter_arrival = task.inter_arrival as f32;

    deadline/inter_arrival
}

pub fn total_load_factor(tasks: &Vec<Task>) -> f32 {
    let mut sum: f32 = 0.0;
    for task in tasks {
        sum += load_factor(task);
    }
    
    sum
}
