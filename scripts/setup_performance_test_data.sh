#!/bin/bash

# ==========================================
# CONFIGURATION
# ==========================================
TARGET_DIR="$HOME/test-dir"
TOTAL_FILES=5000

# Size Categories (in percentages out of 1000 to support decimals like 12.5%)
# Note: 12.5% is represented as 125.
CAT1_PERC=800  # 80.0%
CAT1_SIZE="200k"

CAT2_PERC=125  # 12.50%
CAT2_SIZE="2m"

CAT3_PERC=75   # 7.5%
CAT3_SIZE="10m"

# ==========================================
# PREPARATION
# ==========================================
echo "Target Directory: $TARGET_DIR"
mkdir -p "$TARGET_DIR"

# Clear existing files
rm -f "$TARGET_DIR"/*

# Temporary file to track metrics for the summary
STATS_FILE=$(mktemp)

echo "Generating $TOTAL_FILES files..."

# ==========================================
# GENERATION LOOP
# ==========================================
for i in $(seq 1 $TOTAL_FILES); do
    # 1. Non-uniform Extension selection
    RAND_EXT=$((i % 20))
    if [ $RAND_EXT -lt 5 ]; then EXT="txt";
    elif [ $RAND_EXT -lt 9 ]; then EXT="pdf";
    elif [ $RAND_EXT -lt 12 ]; then EXT="jpg";
    elif [ $RAND_EXT -lt 15 ]; then EXT="png";
    elif [ $RAND_EXT -lt 17 ]; then EXT="zip";
    elif [ $RAND_EXT -lt 18 ]; then EXT="mp4";
    elif [ $RAND_EXT -lt 19 ]; then EXT="docx";
    else EXT="mp3"; fi

    # 2. Configurable Size selection
    # Use a 1000-point scale for precision (supporting 12.5% etc)
    RAND_SIZE=$((RANDOM % 1000))

    if [ $RAND_SIZE -lt $CAT1_PERC ]; then
        SIZE=$CAT1_SIZE
    elif [ $RAND_SIZE -lt $((CAT1_PERC + CAT2_PERC)) ]; then
        SIZE=$CAT2_SIZE
    else
        SIZE=$CAT3_SIZE
    fi

    FILE_PATH="$TARGET_DIR/test_file_$i.$EXT"

    # Create the file
    mkfile "$SIZE" "$FILE_PATH"

    # Log extension and size for summary (in KB for easy math)
    # Using 'du -k' would be slow in a loop, so we map the config strings to numbers
    case $SIZE in
        *k) BYTES=$(( ${SIZE%k} )) ;;
        *m) BYTES=$(( ${SIZE%m} * 1024 )) ;;
        *g) BYTES=$(( ${SIZE%g} * 1024 * 1024 )) ;;
    esac
    echo "$EXT $BYTES" >> "$STATS_FILE"

    # Progress indicator
    if [ $((i % 5000)) -eq 0 ]; then
        echo "Created $i files..."
    fi
done

# ==========================================
# FINAL SUMMARY
# ==========================================
echo -e "\n------------------------------------------------------------"
echo -e "FINISH SUMMARY"
echo -e "------------------------------------------------------------"
echo -e "Target: $TARGET_DIR"
echo -e "Total Files: $TOTAL_FILES"
echo -e "Total Volume: $(du -sh "$TARGET_DIR" | cut -f1)"
echo -e "------------------------------------------------------------"
echo -e "Composition:"
echo -e " - $((CAT1_PERC / 10)).$((CAT1_PERC % 10))% of files => $CAT1_SIZE each"
echo -e " - $((CAT2_PERC / 10)).$((CAT2_PERC % 10))% of files => $CAT2_SIZE each"
echo -e " - $((CAT3_PERC / 10)).$((CAT3_PERC % 10))% of files => $CAT3_SIZE each"
echo -e "------------------------------------------------------------"
printf "%-10s %-15s %-15s\n" "[EXT]" "[COUNT]" "[TOTAL SIZE]"
echo -e "------------------------------------------------------------"

# Use awk to aggregate the stats file
awk '
{
    count[$1]++;
    size[$1]+=$2;
}
END {
    for (ext in count) {
        s = size[ext];
        if (s >= 1048576) {
            pretty_size = sprintf("%.2f GB", s / 1048576);
        } else if (s >= 1024) {
            pretty_size = sprintf("%.2f MB", s / 1024);
        } else {
            pretty_size = sprintf("%d KB", s);
        }
        printf "%-10s %-15d %-15s\n", ext, count[ext], pretty_size;
    }
}' "$STATS_FILE" | sort -k2 -nr

echo -e "------------------------------------------------------------"

# Cleanup
rm "$STATS_FILE"
