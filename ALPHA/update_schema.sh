#!/bin/bash

# Generate the schema.rs file from the database
echo "Generating schema.rs..."
diesel print-schema > src/schema.rs

# Generate the models.rs file from the schema
echo "Generating models.rs..."
diesel_ext  -t -s src/schema.rs --derive "Queryable, Selectable, Identifiable, Debug, Clone, serde::Serialize, serde::Deserialize" --model > src/models.rs
# TODO: use -r flag, but first change the 'type' columns to another name.

# Add the import statement after the last line starting with #!
echo "Updating models.rs with schema import..."
awk '
# First pass: find the last line that starts with #!
BEGIN { last_bang_line = 0 }
/^#!/ { last_bang_line = NR }
{ line[NR] = $0 }
END {
    # Second pass: print all lines and add our import after the last #! line
    for (i = 1; i <= NR; i++) {
        print line[i];
        if (i == last_bang_line) {
            print "use crate::schema::*;";
        }
    }
}
' src/models.rs > src/models.rs.tmp && mv src/models.rs.tmp src/models.rs

echo "Done!"
