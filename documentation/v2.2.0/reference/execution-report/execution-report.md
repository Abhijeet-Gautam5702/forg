# Understanding the Execution Report

Every time you run **forg**, it provides a detailed breakdown of the operations performed. This report is your audit log for the organization process.

## Report Sections

### 1. Header Information
*   **Target Directory**: The folder that was scanned.
*   **Execution Mode**: Whether it used `ConfigBased` or `ConfigBypass` (On-the-fly).
*   **Flags**: A summary of which options (`on-conflict`, `dry-run`, etc.) were active.

### 2. Summary
*   **Total Files**: Total number of files scanned in the target directory.
*   **Total Matched**: How many files matched at least one regex rule.
*   **To be Moved / Moved Successfully**: The count of files that passed all checks and were (or would be) moved.
*   **Skipped (Conflict)**: Files that weren't moved because they already existed at the destination (only in `skip` mode).
*   **Failed (Errors)**: Count of operations that failed due to I/O errors, permission issues, etc.
*   **Total Data Volume**: The cumulative size of all files moved.

### 3. Performance (Actual Runs Only)
*   **Total Time**: The entire duration from start to finish.
*   **Moving Time**: Specifically the time spent in the file movement phase.
*   **Avg. Speed**: Logical throughput (MB/s). Because **forg** is metadata-bound, this can often exceed several GB/s!

### 4. Errors
If any files failed to move, a list is provided with the filename and the specific error message encountered.

## Example Report

```text
-----------EXECUTION REPORT-----------
Target Directory : /Users/user/Downloads
Execution Mode: ConfigBased
Flags: on-conflict=skip, dry-run=false, ignore-case=false, allow-hidden=false

--- Summary ---
Total Files : 1240
Total Matched: 850
Moved Successfully: 845
Skipped (Conflict): 5
Failed (Errors): 0
Total Data Volume: 12.45 GB

--- Performance ---
Total Time: 145ms
Moving Time: 82ms
Avg. Speed: 155.42 MB/s (during move phase)
-----------EXECUTION REPORT-----------
```
