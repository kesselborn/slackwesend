use anyhow::{anyhow, bail, Result};
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn lock_and_read(bucket: &str, key: &str) -> Result<String> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let lock_key = format!(".{}.lock", key);

    // Set the maximum wait time for the lock (2 seconds)
    let max_wait_time = Duration::from_secs(2);
    let start_time = Instant::now();

    // Wait for the lock file to be deleted or timeout
    while client
        .head_object()
        .bucket(bucket)
        .key(&lock_key)
        .send()
        .await
        .is_ok()
    {
        if start_time.elapsed() > max_wait_time {
            return Err(anyhow!("Timed out waiting for lock file to be deleted"));
        }
        sleep(Duration::from_millis(100)).await;
    }

    // Create the lock file
    client
        .put_object()
        .bucket(bucket)
        .key(&lock_key)
        .send()
        .await?;

    // Read the shared state file
    let shared_state_object = client.get_object().bucket(bucket).key(key).send().await?;
    let mut shared_state = String::new();
    let mut stream = shared_state_object.body.into_async_read();
    tokio::io::AsyncReadExt::read_to_string(&mut stream, &mut shared_state).await?;

    Ok(shared_state)
}

pub async fn write_and_unlock(bucket: &str, key: &str, content: &str) -> anyhow::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    let lock_key = format!(".{}.lock", key);

    // Write the updated shared state back to S3
    let result = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(content.to_string().into_bytes().into())
        .send()
        .await;

    unlock(bucket, &lock_key).await?;

    match result {
        Ok(_) => Ok(()),
        Err(e) => bail!("error writing state to s3: {}", e),
    }
}

pub async fn unlock(bucket: &str, lock_key: &str) -> Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    // Delete the lock file
    client
        .delete_object()
        .bucket(bucket)
        .key(lock_key)
        .send()
        .await?;

    Ok(())
}
