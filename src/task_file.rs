use std::cmp::{Ord, Ordering};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::mem::drop;
use std::path::PathBuf;

use {Status, Task};

/// Representation of a todo.txt task file
///
/// It is associated with an optional done file to archive the done files.
/// If no done file is associated, the done tasks are discarded. If you want
/// to keep done tasks in the tasks file, just use the same filename for the
/// done file.
///
/// This TaskFile represents the state of the task file the last time it was read,
/// and will internally buffer any changes you made to the files. Use the `reload` and
/// `flush` methods for synchronization with disk. You are responsible of calling these
/// methods regularly to ensure good syncrhonisation if your app is long-running.
pub struct TaskFile {
    filename: PathBuf,
    done_file: Option<PathBuf>,
    tasks: Vec<Task>,
    done: Vec<Task>,
}

impl TaskFile {
    /// Open a task file and its optional associated done file
    pub fn open(
        path: impl Into<PathBuf>,
        done_file: Option<impl Into<PathBuf>>,
    ) -> io::Result<TaskFile> {
        let mut me = TaskFile {
            filename: path.into(),
            done_file: done_file.map(Into::into),
            tasks: Vec::new(),
            done: Vec::new(),
        };
        me.reload()?;
        Ok(me)
    }

    /// Reload the file from the disk
    ///
    /// Will overwrite any not-flushed pending modifications.
    pub fn reload(&mut self) -> io::Result<()> {
        self.tasks = BufReader::new(File::open(&self.filename)?)
            .lines()
            .map(|r| r.map(Task::parse))
            .collect::<Result<Vec<Task>, _>>()?;
        // handle done tasks and filter them to the done file
        self.done
            .extend(self.tasks.iter().filter(|t| t.is_done()).cloned());
        self.tasks.retain(|t| !t.is_done());
        // sort and dedup the done tasks
        // sort by finished date, and otherwise by subject
        self.done.sort_unstable_by(|t1, t2| {
            match (&t1.status, &t2.status, &t1.subject, &t2.subject) {
                (
                    &Status::Finished(Some(ref date1)),
                    &Status::Finished(Some(ref date2)),
                    sub1,
                    sub2,
                ) => {
                    let cmp = Ord::cmp(date1, date2);
                    if let Ordering::Equal = cmp {
                        Ord::cmp(sub1, sub2)
                    } else {
                        cmp
                    }
                }
                (&Status::Finished(Some(_)), &Status::Finished(None), _, _) => Ordering::Greater,
                (&Status::Finished(None), Status::Finished(Some(_)), _, _) => Ordering::Less,
                (&Status::Finished(None), &Status::Finished(None), sub1, sub2) => {
                    Ord::cmp(sub1, sub2)
                }
                _ => panic!("BUG: found unfinished tasks in the done list"),
            }
        });
        self.done.dedup();
        Ok(())
    }

    /// Flush the pending modifications to the task file.
    ///
    /// The task file will be entirely overwritten as a result.
    ///
    /// If a done file was specified, the pending done tasks will then be appended
    /// to it and removed from the pending buffer.
    ///
    /// Note: if the same file was specified for the task and done file, the final
    /// result of loading it then flushing it will be to reorder the tasks, putting all
    /// unfinished tasks at the top and all finished tasks at the end.
    pub fn flush(&mut self) -> io::Result<()> {
        // flush the main file
        let mut writer = BufWriter::new(File::create(&self.filename)?);
        for task in self.tasks.iter().cloned() {
            writeln!(writer, "{}", Into::<::todo_txt::Task>::into(task))?;
        }
        drop(writer);
        // flush the done file if any
        if let Some(ref done_file) = self.done_file {
            let mut writer = BufWriter::new(OpenOptions::new().append(true).open(done_file)?);
            for task in self.done.iter().cloned() {
                writeln!(writer, "{}", Into::<::todo_txt::Task>::into(task))?;
            }
            // clear the done list, but only if the done file is distinct from the task file,
            // otherwise flushing would not be idempotent
            if done_file != &self.filename {
                self.done.clear();
            }
        } else {
            self.done.clear();
        }
        Ok(())
    }

    /// Iterator over the unfinished tasks, associated with and index to identify them
    ///
    /// This index is used to identify them through the other methods manipulating the task list.
    /// It is **not** stable accross calls of the `reload` method.
    pub fn tasks(&self) -> impl Iterator<Item = (usize, &Task)> {
        self.tasks.iter().enumerate()
    }

    /// Add a new task in the list
    ///
    /// Returns the index of the newly created task
    pub fn add(&mut self, task: Task) -> usize {
        self.tasks.push(task);
        self.tasks.len() - 1
    }

    /// Delete a task from the list
    pub fn delete(&mut self, idx: usize) {
        self.tasks.remove(idx);
    }
}
