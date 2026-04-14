use std::path::PathBuf;

/// Struct to hold all the metrics and details of the execution.
pub struct ExecutionReport {
    pub target_dir: PathBuf,
    pub mode: String,
    pub dry_run: bool,
    pub ignore_case: bool,
    pub allow_hidden: bool,
    pub total_files_scanned: usize,
    pub total_matched: usize,
    pub total_moved: usize,
    pub total_skipped_conflict: usize,
    pub total_failed: usize,
    pub total_size_moved: u64,
    pub elapsed_ms: u128,
    pub moving_elapsed_ms: u128,
    pub failed_files: Vec<(String, String)>,
}

/// Formats bytes into a human-readable string (KB, MB, GB).
fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let i = (bytes as f64).log(1024.0).floor() as usize;
    if i == 0 {
        return format!("{} {}", bytes, sizes[i]);
    }
    format!(
        "{:.2} {}",
        bytes as f64 / 1024.0f64.powi(i as i32),
        sizes[i]
    )
}

/// Generates and prints the execution report to the terminal.
/// Uses bright green ANSI codes for highlighting values.
pub fn generate_execution_report(report: ExecutionReport) {
    // let green = "\x1b[32;1m";
    // let reset = "\x1b[0m";
    let green = ""; // TODO: REMOVE AFTER TESTING
    let reset = ""; // TODO: REMOVE AFTER TESTING

    println!("\n-----------EXECUTION REPORT-----------");

    // Header Information
    println!(
        "Target Directory : {}{}{}",
        green,
        report.target_dir.display(),
        reset
    );
    println!("Execution Mode: {}{}{}", green, report.mode, reset);
    println!(
        "Flags: {}dry-run={}, ignore-case={}, allow-hidden={}{}",
        green, report.dry_run, report.ignore_case, report.allow_hidden, reset
    );

    println!("\n--- Summary ---");
    println!(
        "Total Files : {}{}{}",
        green, report.total_files_scanned, reset
    );
    println!("Total Matched: {}{}{}", green, report.total_matched, reset);

    let action_label = if report.dry_run {
        "To be Moved"
    } else {
        "Moved Successfully"
    };
    println!("{}: {}{}{}", action_label, green, report.total_moved, reset);

    println!(
        "Skipped (Conflict): {}{}{}",
        green, report.total_skipped_conflict, reset
    );
    println!("Failed (Errors): {}{}{}", green, report.total_failed, reset);
    println!(
        "Total Data Volume: {}{}{}",
        green,
        format_bytes(report.total_size_moved),
        reset
    );

    // Performance metrics (only shown for actual runs)
    if !report.dry_run {
        println!("\n--- Performance ---");
        println!("Total Time: {}{}ms{}", green, report.elapsed_ms, reset);

        // Calculate throughput using moving_elapsed_ms if time > 0
        if report.moving_elapsed_ms > 0 && report.total_size_moved > 0 {
            let speed = (report.total_size_moved as f64 / 1024.0 / 1024.0)
                / (report.moving_elapsed_ms as f64 / 1000.0);
            println!(
                "Moving Time: {}{}ms{}",
                green, report.moving_elapsed_ms, reset
            );
            println!(
                "Avg. Speed: {}{:.2} MB/s (during move phase){}",
                green, speed, reset
            );
        }
    }

    // List failures if any
    if !report.failed_files.is_empty() {
        println!("\n--- Errors ---");
        for (file, error) in report.failed_files {
            println!(" - {}: {}", file, error);
        }
    }

    println!("-----------EXECUTION REPORT-----------\n");
}
