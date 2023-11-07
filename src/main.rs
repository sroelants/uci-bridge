use std::{process::{self, Stdio}, io::{Write, BufReader, BufRead}, time::Duration, sync::{Mutex, Arc}};
use std::thread;
use anyhow::*;
use axum::{routing::post, Router, extract::State};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let engine = Engine::new("stockfish", 200)?;
    let state = Arc::new(Mutex::new(engine));

    // build our application with a single route
    let app = Router::new()
        .route("/", post(uci_handler))
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn uci_handler(State(engine): State<Arc<Mutex<Engine>>>, body: String) -> String {
    let mut engine = engine.lock().unwrap();
    let output = engine.post(&body).unwrap();
    format!("{output}")
}

#[derive(Debug)]
struct Engine {
    process: std::process::Child,
    output: Arc<Mutex<Vec<String>>>,
    timeout: u64,
}

impl Engine {
    pub fn new(command: &str, timeout: u64 ) -> anyhow::Result<Engine> {
        let mut child = process::Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Command failed to spawn");

        let buffer: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let stdout = child.stdout.take().ok_or(anyhow!("Failed to acquire stdout"))?;
        let stdout = BufReader::new(stdout);

        let thread_buffer: Arc<Mutex<Vec<String>>> = Arc::clone(&buffer);
        thread::spawn(move || {
            for output in stdout.lines() {
                let output = output.unwrap();
                thread_buffer.lock().unwrap().push(output);
            }
        });

        Ok(Engine { 
            process: child, 
            output: buffer,
            timeout,
        })
    }

    pub fn post(&mut self, input: &str) -> anyhow::Result<String> {
        let child_stdin = self.process.stdin.as_mut().ok_or(anyhow!("Failed to acquire stdin"))?;
        child_stdin.write(input.as_bytes())?;
        child_stdin.write(b"\n")?;
        child_stdin.flush()?;

        // Go to sleep for 200ms
        thread::sleep(Duration::from_millis(self.timeout));

        // Read what's in the buffer and clear it (for some reason that feels
        // safer than iterating over the buffer while output might still be
        // coming in
        let taken = self.output.lock().unwrap().clone();
        self.output.lock().unwrap().clear();

        Ok(taken.join("\n"))
    }
}
