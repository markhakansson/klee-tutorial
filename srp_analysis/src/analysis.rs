/* Home exam functions etc.*/
use crate::common::*;
use crate::helpers::*;

/* 3. Preemption */
// Calculate the response time of a task. R = B + C + I.
pub fn response_time(
    task: &Task,
    tasks: &[Task],
    ip: &IdPrio,
    tr: &TaskResources,
    approx: bool,
) -> u32 {
    let blocking = blocking_time(task, tasks, ip, tr);
    let wcet = wcet(task);
    let interference = preemption(task, tasks, ip, tr, approx);

    blocking + wcet + interference
}

pub fn calc_response_times(
    tasks: &[Task],
    ip: &IdPrio,
    tr: &TaskResources,
    approx: bool,
) -> Vec<(String, u32, u32, u32, u32)> {
    let mut res = Vec::new();

    for task in tasks {
        let c = wcet(task);
        let b = blocking_time(task, tasks, ip, tr);
        let i = preemption(task, tasks, ip, tr, approx);
        let r = c + b + i;
        res.push((task.id.to_string(), r, c, b, i));
    }

    res
}

fn preemption(task: &Task, tasks: &[Task], ip: &IdPrio, tr: &TaskResources, approx: bool) -> u32 {
    if approx {
        preemption_approx(task, tasks, ip)
    } else {
        preemption_rec(task, tasks, ip, tr, 0)
    }
}

// Calculates the approximate preemption time for a task.
fn preemption_approx(task: &Task, tasks: &[Task], ip: &IdPrio) -> u32 {
    let mut sum = 0;
    let mut task_prio = 0;

    if let Some(prio) = ip.get(&task.id) {
        task_prio = *prio
    }

    for t in tasks {
        if let Some(t_prio) = ip.get(&t.id) {
            if t_prio > &task_prio {
                let bp = task.deadline as f32;
                let a = t.inter_arrival as f32;
                let calc = wcet(t) * (bp / a).ceil() as u32;
                sum += calc;
            }
        }
    }

    sum
}

// Response time analysis recurrence relation
fn preemption_rec(
    task: &Task,
    tasks: &[Task],
    ip: &IdPrio,
    tr: &TaskResources,
    prev_rt: u32,
) -> u32 {
    let mut current_rt: u32;
    let mut task_prio = 0;

    if let Some(prio) = ip.get(&task.id) {
        task_prio = *prio
    }

    current_rt = wcet(task) + blocking_time(task, tasks, ip, tr);

    if current_rt != prev_rt {
        let rt_next = preemption_rec(task, tasks, ip, tr, current_rt) as f32;
        let mut sum = 0;

        for t in tasks {
            if let Some(t_prio) = ip.get(&t.id) {
                if t_prio > &task_prio {
                    let a = t.inter_arrival as f32;
                    let calc = wcet(t) * (rt_next / a).ceil() as u32;
                    sum += calc;
                }
            }
        }

        current_rt = sum;
    }

    if current_rt > task.deadline {
        panic!("Bp(t) > D(t)");
    }

    current_rt
}
