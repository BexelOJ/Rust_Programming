
#!/bin/bash

#===================================================
# Rust Data Structures - File Generator
#===================================================

START_DATE="20261101"
PREFIX="rust"
CATEGORY="ds"
SEQUENCE="01"
TOPICS_FILE="topics.txt"

#===================================================
# 01. Validate input
#===================================================

if [ ! -f "$TOPICS_FILE" ]; then
    echo "Error: $TOPICS_FILE not found!"
    exit 1
fi

#===================================================
# 02. Read topics
#===================================================

mapfile -t TOPICS < "$TOPICS_FILE"

# Remove empty lines
FILTERED_TOPICS=()

for TOPIC in "${TOPICS[@]}"; do
    [ -z "$TOPIC" ] && continue
    FILTERED_TOPICS+=("$TOPIC")
done

TOTAL=${#FILTERED_TOPICS[@]}

#===================================================
# 03. Generate new filenames and rename existing
#===================================================

declare -A RENAME_MAP

DAY=0

for TOPIC in "${FILTERED_TOPICS[@]}"; do

    DATE=$(date -d "$START_DATE + $DAY days" +%Y%m%d)

    NEW_FILE="${PREFIX}_${DATE}_${CATEGORY}_${TOPIC}_${SEQUENCE}.rs"

    # Find existing file for this topic
    EXISTING_FILE=$(find . -maxdepth 1 -type f \
        -name "${PREFIX}_*_${CATEGORY}_${TOPIC}_${SEQUENCE}.rs" \
        -print -quit)

    if [ -n "$EXISTING_FILE" ]; then

        if [ "$EXISTING_FILE" != "./$NEW_FILE" ]; then
            RENAME_MAP["$EXISTING_FILE"]="$NEW_FILE"
        fi

    else
        echo "New file required: $NEW_FILE"
    fi

    DAY=$((DAY + 1))

done

#===================================================
# 04. Check rename conflicts
#===================================================

for OLD_FILE in "${!RENAME_MAP[@]}"; do

    NEW_FILE="${RENAME_MAP[$OLD_FILE]}"

    if [ -e "$NEW_FILE" ] && [[ -z "${RENAME_MAP["./$NEW_FILE"]+x}" ]]; then
        echo "Error: Destination already exists: $NEW_FILE"
        echo "Resolve the conflict before running again."
        exit 1
    fi

done

#===================================================
# 05. Rename existing files using temporary names
#===================================================

declare -A FINAL_MAP

for OLD_FILE in "${!RENAME_MAP[@]}"; do

    TEMP_FILE="${OLD_FILE}.tmp_rename"

    mv -- "$OLD_FILE" "$TEMP_FILE" || exit 1

    FINAL_MAP["$TEMP_FILE"]="${RENAME_MAP[$OLD_FILE]}"

done

for TEMP_FILE in "${!FINAL_MAP[@]}"; do

    mv -- "$TEMP_FILE" "${FINAL_MAP[$TEMP_FILE]}" || exit 1

    echo "Renamed: $TEMP_FILE -> ${FINAL_MAP[$TEMP_FILE]}"

done

#===================================================
# 06. Create missing Rust files
#===================================================

DAY=0

for TOPIC in "${FILTERED_TOPICS[@]}"; do

    DATE=$(date -d "$START_DATE + $DAY days" +%Y%m%d)

    FILENAME="${PREFIX}_${DATE}_${CATEGORY}_${TOPIC}_${SEQUENCE}.rs"

    if [ ! -f "$FILENAME" ]; then

        cat > "$FILENAME" <<EOF
fn main()
{
    //---------------------------------------------------
    // ${TOPIC} - Rust Data Structures
    // Date: ${DATE}
    //---------------------------------------------------


}
EOF

        echo "Created: $FILENAME"

    else
        echo "Exists:  $FILENAME"
    fi

    DAY=$((DAY + 1))

done

#===================================================
# 07. Summary
#===================================================

echo
echo "==================================================="
echo "Rust Data Structures - File Generation Completed"
echo "==================================================="

echo "Total topics : $TOTAL"
echo "Start date   : $START_DATE"
echo "End date     : $(date -d "$START_DATE + $((TOTAL - 1)) days" +%Y%m%d)"
echo "==================================================="



