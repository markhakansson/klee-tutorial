/* Home exam functions etc.*/
use crate::common::*;
use std::collections::HashSet;

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

/* 3. Preemption */
// Calculates the preemption time for a task.
pub fn preemption(task: &Task, tasks: &[Task], ip: &IdPrio) -> u32 {
    let mut sum = 0; 
    let mut task_prio = 0;

    if let Some(prio) = ip.get(&task.id) {
        task_prio = *prio
    }
    
    for t in tasks {
        if let Some(t_prio) = ip.get(&t.id) {
            if t_prio > &task_prio {
                let bp = busy_period(task) as f32;
                let a = t.inter_arrival as f32;
                let calc = wcet(t) * (bp / a).ceil() as u32; 
                sum += calc;
            }
        }
    }

    sum
}

pub fn busy_period(task: &Task) -> u32 {
    task.deadline
}

// Calculate the response time of a task. R = B + C + I.
pub fn response_time(task: &Task, tasks: &[Task], ip: &IdPrio, tr: &TaskResources) -> u32 {
    let blocking = blocking_time(task, tasks, ip, tr);
    let wcet = wcet(task);
    let interference = preemption(task, tasks, ip);

    blocking + wcet + interference
}

pub fn calc_response_times(tasks: &[Task], ip: &IdPrio, tr: &TaskResources) -> Vec<(String, u32, u32, u32, u32)> {
    let mut res = Vec::new();
    
    for task in tasks {
        let c = wcet(task);
        let b = blocking_time(task, tasks, ip, tr);
        let i = preemption(task, tasks, ip);
        let r = c + b + i;
        res.push((task.id.to_string(), r, c, b, i));
    }

    res
}
