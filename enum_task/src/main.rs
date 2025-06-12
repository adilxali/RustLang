enum StatusCode {
    Ok,
    NotFound,
    ServerError,
    Custom(u16, String),
}

fn status(code: StatusCode) -> String {
    match code {
        StatusCode::Ok => "200 OK".to_string(),
        StatusCode::NotFound => "404 Not Found".to_string(),
        StatusCode::ServerError => "500 Internal Server Error".to_string(),
        StatusCode::Custom(code, message) => format!("{} {}", code, message),
    }
}

enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Deferred(String),
}

struct Task {
    title: String,
    status: TaskStatus,
}

fn create_task(title:&str) ->Task {
    Task {
        title: title.to_string(),
        status: TaskStatus::Pending,
    }
}

fn update_status(task:&mut Task, new_status: TaskStatus) {
    task.status = new_status;
}

fn print_task(task: &Task) {
    let status_str = match &task.status {
        TaskStatus::Pending => "Pending",
        TaskStatus::InProgress => "In Progress",
        TaskStatus::Completed => "Completed",
        TaskStatus::Deferred(reason) => &format!("Deferred: {}", reason).to_owned(),
    };
    println!("Task: {}, Status: {}", task.title, status_str);
}

fn main() {
    // let ok = status(StatusCode::Ok);
    // let not_found = status(StatusCode::NotFound);
    // let server_error = status(StatusCode::ServerError);
    // let custom = status(StatusCode::Custom(418, "I'm a teapot".to_string()));

    // println!("{}", ok);
    // println!("{}", not_found);
    // println!("{}", server_error);
    // println!("{}", custom);
    let mut task = create_task("Learn Rust enums");
    print_task(&task);
    update_status(&mut task, TaskStatus::InProgress);
    print_task(&task);
    update_status(&mut task, TaskStatus::Deferred("Need more practice".to_string()));
    print_task(&task);
    update_status(&mut task, TaskStatus::Completed);
    print_task(&task);
}
