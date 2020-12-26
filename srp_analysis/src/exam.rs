/* Home exam functions etc.*/
use crate::common::*;
use std::collections::HashSet;

/* 1. Total load factor */
// Calculate load factor of a task
pub fn load_factor(task: &Task) -> f32 {
    let deadline = task.deadline as f32;
    let inter_arrival = task.inter_arrival as f32;

    deadline / inter_arrival
}

// Calculate the total load factor of all tasks
pub fn total_load_factor(tasks: &[Task]) -> f32 {
    let mut sum: f32 = 0.0;
    for task in tasks {
        sum += load_factor(task);
    }

    sum
}

/* 2. Blocking */
// Blocking function. Calculates the largest amount of time a task (T1) may be blocked by
// another task (T2) using a resource (R). P(T2) < P(T1) and P(R) >= P(T1).
pub fn blocking_time(task: &Task, tasks: &[Task], ip: &IdPrio, tr: &TaskResources) -> u32 {
    let mut max_block_time = 0;
    let mut task_prio = 0;
    let mut resources = &HashSet::new();

    // Get the task's priority
    if let Some(prio) = ip.get(&task.id) {
        task_prio = *prio
    }
    // Get the resources used by the task
    if let Some(set) = tr.get(&task.id) {
        resources = set
    }

    for r in resources.iter() {
        for t in tasks {
            if let (Some(t_prio), Some(r_ceil)) = (ip.get(&t.id), ip.get(r)) {
                // Compare the priority and ceiling with the task prio
                if t_prio < &task_prio && r_ceil >= &task_prio {
                    let time = max_time_hold_resource(&t.trace, r);
                    if time > max_block_time {
                        max_block_time = time;
                    }
                }
            }
        }
    }

    max_block_time
}

// Get the maximum length of time for which the resource is hold in a trace
pub fn max_time_hold_resource(trace: &Trace, res_id: &str) -> u32 {
    let mut max_time = 0;

    if trace.id == res_id {
        max_time = trace.end - trace.start;
    }

    // Recursively calculate the max time
    if !trace.inner.is_empty() {
        for t in &trace.inner {
            let time = max_time_hold_resource(&t, res_id);
            if time > max_time {
                max_time = time;
            }
        }
    }

    max_time
}
