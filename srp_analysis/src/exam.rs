/* Home exam functions etc.*/
use crate::common::*;
use std::collections::{HashMap, HashSet};

/* 1. Total load factor */

// Calculate load factor of a task
pub fn load_factor(task: &Task) -> f32 {
    let deadline = task.deadline as f32;
    let inter_arrival = task.inter_arrival as f32;

    deadline/inter_arrival
}

// Calculate the total load factor of all tasks
pub fn total_load_factor(tasks: &Vec<Task>) -> f32 {
    let mut sum: f32 = 0.0;
    for task in tasks {
        sum += load_factor(task);
    }
    
    sum
}

/* 2. Blocking */
pub fn blocking_time(task: &Task, tasks: &Vec<Task>, ip: &IdPrio, tr: &TaskResources) -> u32 {
    let mut max_block_time = 0;
    let mut task_prio = 0;
    let mut resources  = &HashSet::new(); 
    
    // Get the task's priority
    match ip.get(&task.id) {
        Some(prio) => task_prio = *prio,
        None => (),
    }

    // Get the resources used by the task
    match tr.get(&task.id) {
        Some(set) => resources = set,
        None => (),
    }
    
    for r in resources.iter() {
        for t in tasks {
            // Compare the priority and ceiling
            if let (Some(t_prio), Some(r_ceil)) = (ip.get(&t.id), ip.get(r)) {
                if t_prio < &task_prio && r_ceil >= &task_prio {
                    let _time = max_time_hold_resource(&t.trace, r);
                    if _time > max_block_time {
                        max_block_time = _time;
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
    
    if trace.inner.len() > 0 {
        for t in &trace.inner {
            let _time = max_time_hold_resource(&t, res_id);
            if _time > max_time {
                max_time = _time;
            }
        }
    }

    max_time
}

