use crate::types::*;
use askama::Template;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
struct TaskData<'a> {
    name: &'a str,
    response: &'a u32,
    wcet: &'a u32,
    bt: &'a u32,
    preemption: &'a u32,
    load: &'a f32,
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate<'a> {
    data: &'a Vec<TaskData<'a>>,
}

/// Writes the ResponseTimes of a set of tasks to an HTML file of the given
/// filename.
pub fn write_report_to_file(rt: &ResponseTimes, filename: &String) -> Result<(), String> {
    let output = render_report(rt)?;

    let file = File::create(filename);
    match file {
        Ok(mut f) => {
            f.write_all(output.as_bytes());
            Ok(())
        },
        Err(e) => Err(e.to_string())
    }

}

fn format_rt(rt: &ResponseTimes) -> Vec<TaskData> {
    let mut fmt: Vec<TaskData> = Vec::new();

    for (name, response, wcet, bt, preemption, load) in rt {
        fmt.push(TaskData {
            name,
            response,
            wcet,
            bt,
            preemption,
            load,
        });
    }

    fmt
}

fn render_report(rt: &ResponseTimes) -> Result<String, String> {
    let fmt = format_rt(rt);
    let r = ReportTemplate { data: &fmt }.render();

    match r {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}
