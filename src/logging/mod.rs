use chrono::Local;
use chrono::Utc;
use cron::Schedule;
use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::{AtomicU16, Ordering};
use std::time::Duration;
use tokio::time;
use tokio_cron_scheduler::{Job, JobScheduler, JobToRun};

pub struct LogHandleError {
    pub code: i32,
    pub message: String,
}

impl<E: Display> From<E> for LogHandleError {
    fn from(value: E) -> Self {
        LogHandleError {
            code: 0,
            message: value.to_string(),
        }
    }
}

pub struct LogHandle {
    filename: String,
    data: AtomicU16,
    scheduler: JobScheduler,
    schedule: Schedule,
}

impl LogHandle {
    pub async fn new(filename: String, schedule: Schedule) -> Result<Self, LogHandleError> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&filename)
            .unwrap();

        let sched = JobScheduler::new().await?;

        // sched.add(Job::new(period, |_uuid, _l| {
        //     println!("I run every 10 seconds");
        // })?);

        // sched.start().await;

        Ok(Self {
            filename,
            data: AtomicU16::new(0),
            scheduler: sched,
            schedule,
        })
    }

    pub async fn start(&self) -> Result<(), LogHandleError> {
        // println!("Upcoming fire times:");
        // for datetime in self.schedule.upcoming(Utc).take(10) {
        //     println!("-> {}", datetime);
        // }

        self.scheduler
            .add(Job::new(self.schedule.clone(), |_uuid, _l| {
                println!("I run every 10 seconds");
            })?)
            .await
            .unwrap();

        self.scheduler.start().await.unwrap();

        return Ok(());
    }

    pub async fn drop(&mut self) {
        self.scheduler.shutdown().await.ok();
    }

    pub async fn log(&self) {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.filename);

        match file {
            Ok(mut f) => {
                let now = Local::now();
                let current_value = self.data.swap(0, Ordering::Relaxed);

                let result =
                    f.write_all(format!("{} {}\n", now.to_rfc3339(), current_value).as_bytes());

                match result {
                    Ok(_) => {}
                    Err(_e) => {
                        self.data.fetch_add(current_value, Ordering::Relaxed);
                    }
                }
            }
            Err(_) => {}
        }
    }

    pub async fn increment(&self) {
        self.data.fetch_add(1, Ordering::Relaxed);
    }
}

// https://docs.rs/rustbreak/latest/rustbreak/
