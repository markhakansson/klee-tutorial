use crate::common::*;
use crate::types::*;
use std::collections::HashSet;

/* 2. Blocking */
// Blocking function. Calculates the largest amount of time a task (T1) may be blocked by
// another task (T2) using a resource (R). P(T2) < P(T1) and P(R) >= P(T1).
pub fn blocking_time(task: &Task, tasks: &[Task], ip: &IdPrio, tr: &TaskResources) -> B {
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
fn max_time_hold_resource(trace: &Trace, res_id: &str) -> u32 {
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
