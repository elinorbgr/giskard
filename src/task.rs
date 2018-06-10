use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::NaiveDate;
use todo_txt::Task as RawTask;

/// A Task with its metadata
#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub subject: String,
    pub priority: u8,
    pub creation_date: Option<NaiveDate>,
    pub status: Status,
    pub threshold_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub contexts: Vec<String>,
    pub projects: Vec<String>,
    pub hashtags: Vec<String>,
    pub tags: BTreeMap<String, String>,
}

impl Task {
    /// Parse a Task from some string
    pub fn parse(txt: impl Borrow<str>) -> Task {
        // For this unwrap, see https://github.com/sanpii/todo-txt/issues/10
        RawTask::from_str(txt.borrow()).unwrap().into()
    }

    /// Check if the task is done
    pub fn is_done(&self) -> bool {
        if let Status::Finished(_) = self.status {
            true
        } else {
            false
        }
    }
}

/// Possible status of a Task
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    /// The task is started
    Started,
    /// The task is finished, with an optional finish date
    Finished(Option<NaiveDate>),
}

impl ::std::convert::From<RawTask> for Task {
    fn from(t: RawTask) -> Task {
        // compute status
        let status = if t.finished {
            Status::Finished(t.finish_date)
        } else {
            Status::Started
        };

        Task {
            subject: t.subject,
            priority: t.priority,
            creation_date: t.create_date,
            status,
            threshold_date: t.threshold_date,
            due_date: t.due_date,
            contexts: t.contexts,
            projects: t.projects,
            hashtags: t.hashtags,
            tags: t.tags,
        }
    }
}

impl ::std::convert::Into<RawTask> for Task {
    fn into(self) -> RawTask {
        let (finished, finish_date) = match self.status {
            Status::Started => (false, None),
            Status::Finished(date) => (true, date),
        };
        RawTask {
            subject: self.subject,
            priority: self.priority,
            create_date: self.creation_date,
            finished,
            finish_date,
            threshold_date: self.threshold_date,
            due_date: self.due_date,
            contexts: self.contexts,
            projects: self.projects,
            hashtags: self.hashtags,
            tags: self.tags,
        }
    }
}
