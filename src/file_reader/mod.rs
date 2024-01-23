use core::future::Future;
use std::{future::IntoFuture, io::Error};
use tokio::task::{JoinError, JoinHandle};

pub struct FileReader {
    pub file_paths: Vec<String>,
}

impl FileReader {
    pub async fn read_from_paths(&self) {
        let mut async_reading_tasks = vec![];

        for path in &self.file_paths {
            let parsed_path: String = path.clone();

            let task = self.spawn_reading_task(parsed_path);

            async_reading_tasks.push(task)
        }

        self.execute_async_tasks(async_reading_tasks).await;
    }

    async fn spawn_reading_task(&self, path: String) -> tokio::task::JoinHandle<()> {
        return tokio::spawn(async move {
            let content: Result<String, Error> = tokio::fs::read_to_string(&path).await;

            match content {
                Ok(content) => println!("{:?}", &content),
                Err(content) => println!("Error: {:?}", &content),
            }
        });
    }

    async fn execute_async_tasks(&self, tasks: Vec<impl Future<Output = JoinHandle<()>>>) {
        for task in tasks {
            let _: Result<(), JoinError> = task.await.into_future().await;
        }
    }
}