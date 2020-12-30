/* Home exam functions etc.*/
use crate::blocking::*;
use crate::common::*;
use crate::helpers::*;

// Custom types to make it easier to read
type R = u32;
type B = u32;
type C = u32;
type I = u32;
type ResponseTimes = Vec<(String, R, C, B, I)>;

/* 3. Preemption and response times */
// Calculate the response time of a task. R = B + C + I.
pub fn response_time(
    task: &Task,
    tasks: &[Task],
    approx: bool,
) -> Result<u32, String> {
    let (ip, tr) = pre_analysis(tasks);
    let blocking = blocking_time(task, tasks, &ip, &tr);
    let wcet = wcet(task);
    let interference = preemption(task, tasks, &ip, &tr, approx)?;

    Ok(blocking + wcet + interference)
}

// Calculates the response time of a all tasks. R = B + C + I.
// Returns a vector with the above values
pub fn calc_response_times(
    tasks: &[Task],
    approx: bool,
) -> Result<ResponseTimes, String> {
    let mut res = Vec::new();
    let (ip, tr) = pre_analysis(tasks);

    for task in tasks {
        let c = wcet(task);
        let b = blocking_time(task, tasks, &ip, &tr);
        let i = preemption(task, tasks, &ip, &tr, approx)?;
        let r = c + b + i;
        res.push((task.id.to_string(), r, c, b, i));
    }

    Ok(res)
}

// Calculates the preemption (I(t)) of a task with or without approximation.
fn preemption(
    task: &Task,
    tasks: &[Task],
    ip: &IdPrio,
    tr: &TaskResources,
    approx: bool,
) -> Result<I, String> {
    if approx {
        Ok(preemption_approx(task, tasks, ip))
    } else {
        let base = wcet(task) + blocking_time(task, tasks, ip, tr);
        let premp = preemption_rec(task, tasks, ip, tr, base)?;
        Ok(premp - base)
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
) -> Result<u32, String> {
    let mut current_rt = wcet(task) + blocking_time(task, tasks, ip, tr);
    let mut task_prio = 0;

    if let Some(prio) = ip.get(&task.id) {
        task_prio = *prio
    }

    // The summation part of eq. 7.22 in Hard Real-Time Computing Systems
    for t in tasks {
        if let Some(t_prio) = ip.get(&t.id) {
            if t_prio > &task_prio {
                let a = t.inter_arrival as f32;
                let calc = wcet(t) * (prev_rt as f32 / a).ceil() as u32;
                current_rt += calc;
            }
        }
    }

    if current_rt > task.deadline {
        let msg = format!(
            "{}: (Bp(t) == {}) > (D(t) = {})! ",
            task.id,
            current_rt.to_string(),
            task.deadline.to_string()
        );
        return Err(msg);
    }

    if current_rt == prev_rt {
        Ok(current_rt)
    } else {
        preemption_rec(task, tasks, ip, tr, current_rt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_rt() {
        let t1 = Task {
            id: "T1".to_string(),
            prio: 1,
            deadline: 100,
            inter_arrival: 100,
            trace: Trace {
                id: "T1".to_string(),
                start: 0,
                end: 10,
                inner: vec![],
            },
        };

        let t2 = Task {
            id: "T2".to_string(),
            prio: 2,
            deadline: 200,
            inter_arrival: 200,
            trace: Trace {
                id: "T2".to_string(),
                start: 0,
                end: 30,
                inner: vec![
                    Trace {
                        id: "R1".to_string(),
                        start: 10,
                        end: 20,
                        inner: vec![Trace {
                            id: "R2".to_string(),
                            start: 12,
                            end: 16,
                            inner: vec![],
                        }],
                    },
                    Trace {
                        id: "R1".to_string(),
                        start: 22,
                        end: 28,
                        inner: vec![],
                    },
                ],
            },
        };

        let t3 = Task {
            id: "T3".to_string(),
            prio: 3,
            deadline: 50,
            inter_arrival: 50,
            trace: Trace {
                id: "T3".to_string(),
                start: 0,
                end: 30,
                inner: vec![Trace {
                    id: "R2".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![],
                }],
            },
        };

        // builds a vector of tasks t1, t2, t3
        let tasks = vec![t1, t2, t3];
        
        let rt1 = response_time(&tasks[0], &tasks, false).unwrap();
        let rt2 = response_time(&tasks[1], &tasks, false).unwrap();
        let rt3 = response_time(&tasks[2], &tasks, false).unwrap();

        assert_eq!(rt1, 100);
        assert_eq!(rt2, 90);
        assert_eq!(rt3, 34);
    }
}
