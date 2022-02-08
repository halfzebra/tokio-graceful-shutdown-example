use std::error::Error;
use std::process::Stdio;
use tokio::process::Command;
use tokio::signal::unix;

async fn run() -> Result<(), Box<dyn Error>> {
    println!("Starting...");

    let exit_status = Command::new("./scripts/run_with_progress.sh")
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()?
        .wait()
        .await?;

    println!("Exited: {:?}", exit_status);

    Ok(())
}

async fn unix_process_stop() -> Result<(), Box<dyn Error>> {
    let mut sigint_stream = unix::signal(unix::SignalKind::interrupt())?;
    let mut sigquit_stream = unix::signal(unix::SignalKind::quit())?;

    let sigint = sigint_stream.recv();
    let sigquit = sigquit_stream.recv();

    tokio::pin!(sigint, sigquit);

    tokio::select! {
        _ = &mut sigint => {
            println!("sigint");
            return Ok(());
        }
        _ = &mut sigquit => {
            println!("sigquit");
            return Ok(());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stop = unix_process_stop();

    tokio::select! {
        res = run() => {
            println!("Operation completed {:?}", res);
            std::process::exit(0);
        }
        _ = stop => {
            println!("Stopping");
            std::process::exit(1);
        }
    }
}
