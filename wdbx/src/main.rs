use clap::{Parser, Subcommand};
use wdbx::benchmark_matmul;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a matrix multiplication benchmark
    Benchmark {
        /// Number of iterations
        #[arg(short, long, default_value_t = 10)]
        iters: usize,

        /// Size of the square matrices
        #[arg(short, long, default_value_t = 1024)]
        size: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Benchmark { iters, size } => {
            println!("🚀 Running benchmark: {}x{} matmul, {} iterations...", size, size, iters);
            let avg_time = benchmark_matmul(*size, *iters);
            println!("✅ Average time per iteration: {:.4} ms", avg_time * 1000.0);
            println!("📊 Total time: {:.4} s", avg_time * (*iters as f64));
        }
    }
}
