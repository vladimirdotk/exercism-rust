use std::{collections::HashMap, sync::Arc};

use tokio::runtime::Builder;
use tokio::sync::Mutex as TokioMutex;

pub fn frequency(input: &[&str], workers: usize) -> HashMap<char, usize> {
    let worker_threads = if workers > 0 { workers } else { 1 };

    let runtime = Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .unwrap();

    let result = Arc::new(TokioMutex::new(HashMap::new()));

    let input = input.iter().map(|&s| s.to_string()).collect::<Vec<_>>();

    runtime.block_on(async {
        let mut tasks = vec![];

        for line in input {
            let result = Arc::clone(&result);
            tasks.push(tokio::spawn(async move {
                process_string(line.as_str(), result).await
            }));
        }

        for task in tasks {
            let _ = task.await.expect("Task failed");
        }

        Arc::try_unwrap(result).unwrap().into_inner()
    })
}

async fn process_string(s: &str, result: Arc<TokioMutex<HashMap<char, usize>>>) {
    for c in s.chars() {
        if c.is_alphabetic() {
            let c = c.to_lowercase().next().unwrap();
            print!("{}\n", c);
            *result.lock().await.entry(c).or_insert(0) += 1;
        }
    }
}
