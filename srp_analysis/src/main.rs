mod common;
use common::*;
mod exam;
use exam::*;

fn main() {
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
    let tasks: Tasks = vec![t1, t2, t3];

    println!("tasks {:#?}", &tasks);
    println!("total_load_factor: {}", total_load_factor(&tasks));

    let (ip, tr) = pre_analysis(&tasks);
    println!("ip: {:?}", ip);
    println!("tr: {:?}", tr);

    println!(
        "max_time: {:?}",
        max_time_hold_resource(&tasks[1].trace, &"R2")
    );
    println!(
        "block_time: {:?}",
        blocking_time(&tasks[2], &tasks, &ip, &tr)
    );
    println!("(Task, R(t), C(t), B(t), I(t))");
    println!(
        "response times (approx): {:#?}",
        calc_response_times(&tasks, &ip, &tr, true)
    );
    println!(
        "response times: {:#?}",
        calc_response_times(&tasks, &ip, &tr, false)
    );
    println!(
        "rta: {:#?}",
        preemption_rec(&tasks[1], &tasks, &ip, &tr, 0) 
    );
}
