use mist::algorithms::vdb12::applications::Application;
use mist::algorithms::vdb12::tasks::Task;

fn main() {
    let mut app1 = Application::default();
    app1.add_tasks(vec![Task::default()]);

    let mut app2 = Application::default();
    app2.add_tasks(vec![Task::default(), Task::default()]);

    println!("{}", app1.to_json());
    println!("{}", app2.to_json());
}
