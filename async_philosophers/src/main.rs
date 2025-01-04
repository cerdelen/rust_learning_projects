use clap::Parser;
use std::sync::Arc;
use std::time::Instant;

use tokio::time::{sleep, Duration};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    philosophers: usize,

    #[arg(short, long)]
    eat_time: u64,

    #[arg(short, long)]
    sleep_time: u64,

    #[arg(short, long)]
    die_time: u64,

    #[arg(short, long)]
    meals: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let start_time = Instant::now();
    let forks = Arc::new(
        (0..args.philosophers)
            .map(|_| Mutex::new(()))
            .collect::<Vec<_>>(),
    );

    let mut handles = vec![];

    for i in 0..args.philosophers {
        let forks = Arc::clone(&forks);
        let start_time = start_time.clone();

        let handle = tokio::spawn(async move {
            let left = i;
            let right = (i + 1) % args.philosophers;
            let mut last_meal = start_time.clone();
            let mut meal_count = 0;

            if i % 2 == 1 {
                sleep(Duration::from_millis(args.eat_time / 2)).await;
            }
            loop {
                let _left_fork = forks[left].lock().await;
                println!("[{:06}]: Philosopher {} took fork {}.", start_time.elapsed().as_millis(), i, left);
                let _right_fork = forks[right].lock().await;
                println!("[{:06}]: Philosopher {} took fork {}.", start_time.elapsed().as_millis(), i, right);

                if meal_count >= args.meals {
                    println!("[{:06}]: Philosopher {} is full of dolma.", start_time.elapsed().as_millis(), i);
                    break;
                }
                if last_meal.elapsed() > Duration::from_millis(args.die_time) {
                    println!("[{:06}]: Philosopher {} dies of starvation.", start_time.elapsed().as_millis(), i);
                    break;
                }
                println!("[{:06}]: Philosopher {} is eating.", start_time.elapsed().as_millis(), i);
                sleep(Duration::from_millis(args.eat_time)).await;
                last_meal = Instant::now();
                drop(_left_fork);
                drop(_right_fork);
                meal_count += 1;

                println!("[{:06}]: Philosopher {} is sleeping.", start_time.elapsed().as_millis(), i);
                sleep(Duration::from_millis(args.sleep_time)).await;
                println!("[{:06}]: Philosopher {} is thinking.", start_time.elapsed().as_millis(), i);
            }
        });
        handles.push(handle);
    }


    for handle in handles {
        let _ = handle.await;
    }
}
