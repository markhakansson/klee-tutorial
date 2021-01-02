use std::fs::File;
use std::io::prelude::*;
use crate::types::*;
use askama::Template;

#[derive(Clone, Debug)]
struct TaskData<'a> {
    name: &'a str,
    response: &'a u32,
    wcet: &'a u32,
    bt: &'a u32,
    preemption: &'a u32,
    load: &'a f32
}

#[derive(Template)]
#[template(path = "report.html")]
struct ReportTemplate<'a> {
    data: &'a Vec<TaskData<'a>> 
}

pub fn write_report_to_file(rt: &ResponseTimes, filename: String) {
    let output = render_report(rt);

    let mut file = File::create(filename + ".html").unwrap(); 
    file.write_all(output.as_bytes());
}

fn format_rt(rt: &ResponseTimes) -> Vec<TaskData> {
    let mut fmt: Vec<TaskData> = Vec::new();

    for (name, response, wcet, bt, preemption, load) in rt {
        fmt.push(TaskData {name, response, wcet, bt, preemption, load }); 
    }

    fmt
}

fn render_report(rt: &ResponseTimes) -> String {
    let fmt = format_rt(rt);

    ReportTemplate{data: &fmt}.render().unwrap()
}
