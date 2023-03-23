use chrono::Local;
use cron::Schedule;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

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
    filename: Arc<String>,
    data: Arc<AtomicU16>,
    scheduler: JobScheduler,
    schedule: Schedule,
}

pub async fn create_and_start_counter(
    filename: String,
    schedule: String,
) -> Result<LogHandle, LogHandleError> {
    let log_handle = LogHandle::new(filename, Schedule::from_str(&schedule).unwrap()).await;
    match log_handle {
        Ok(handle) => {
            let result = handle.start().await;
            match result {
                Ok(_) => {
                    return Ok(handle);
                }
                Err(e) => {
                    return Err(LogHandleError::from(e));
                }
            }
        }
        Err(e) => {
            return Err(LogHandleError::from(e));
        }
    }
}

fn save_log(filename: &str, value: &Arc<AtomicU16>) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(filename) {
        let now = Local::now();
        let current_value = value.swap(0, Ordering::Relaxed);

        if let Err(_) = writeln!(file, "{} {}", now.to_rfc3339(), current_value) {
            value.fetch_add(current_value, Ordering::Relaxed);
        }
    }
}

impl LogHandle {
    pub async fn new(filename: String, schedule: Schedule) -> Result<Self, LogHandleError> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&filename)
            .unwrap();

        let sched = JobScheduler::new().await?;

        Ok(Self {
            filename: Arc::new(filename),
            data: Arc::new(AtomicU16::new(0)),
            scheduler: sched,
            schedule,
        })
    }

    pub async fn start(&self) -> Result<(), LogHandleError> {
        let value = Arc::clone(&self.data);
        let filename = Arc::clone(&self.filename);
        self.scheduler
            .add(Job::new(self.schedule.clone(), move |_uuid, _l| {
                save_log(&filename, &value);
            })?)
            .await
            .unwrap();

        self.scheduler.start().await.unwrap();

        return Ok(());
    }

    pub async fn drop(&mut self) {
        self.scheduler.shutdown().await.ok();
    }

    pub fn increment(&self) {
        self.data.fetch_add(1, Ordering::Relaxed);
    }

    pub fn getData(&self) -> Arc<AtomicU16> {
        Arc::clone(&self.data)
    }
}

// https://docs.rs/rustbreak/latest/rustbreak/
