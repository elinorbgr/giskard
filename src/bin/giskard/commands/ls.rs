use giskard::TaskFile;

pub struct Args {}

pub fn run(file: TaskFile, _args: Args) {
    for (idx, task) in file.tasks() {
        println!(
            "{:>4}  ({}) {}",
            idx,
            if task.priority < 26 {
                (b'A' + task.priority) as char
            } else {
                ' '
            },
            task.subject
        );
    }
}
