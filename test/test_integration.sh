#!/bin/bash

# Integration test for Carrot recipe parser
# Tests the CLI tool with a known recipe URL

set -e

echo "=== Carrot Recipe Parser Integration Test ==="
echo ""

# Test URL - Budget Bytes Orange Julius recipe (print version for clean parsing)
TEST_URL="https://www.budgetbytes.com/wprm_print/orange-julius"

echo "Building CLI tool..."
cargo build --bin carrot-cli --quiet

echo "Testing with URL: $TEST_URL"
echo ""

# Run the CLI tool and capture output
OUTPUT=$(../target/debug/carrot-cli --url "$TEST_URL" --format markdown)

echo "✅ CLI tool executed successfully"
echo ""

# Check if output contains expected recipe elements
if echo "$OUTPUT" | grep -q "Orange Julius"; then
    echo "✅ Recipe title found"
else
    echo "❌ Recipe title NOT found"
    exit 1
fi

if echo "$OUTPUT" | grep -q "Ingredients"; then
    echo "✅ Ingredients section found"
else
    echo "❌ Ingredients section NOT found"
    exit 1
fi

if echo "$OUTPUT" | grep -q "Instructions"; then
    echo "✅ Instructions section found"
else
    echo "❌ Instructions section NOT found"
    exit 1
fi

if echo "$OUTPUT" | grep -q "orange juice concentrate"; then
    echo "✅ Specific ingredient found"
else
    echo "❌ Specific ingredient NOT found"
    exit 1
fi

if echo "$OUTPUT" | grep -q "blend until smooth"; then
    echo "✅ Instruction text found"
else
    echo "❌ Instruction text NOT found"
    exit 1
fi

echo ""
echo "🎉 All tests passed! Recipe parsing is working correctly."
echo ""
echo "Sample output:"
echo "=============="
echo "$OUTPUT"
echo "..."
echo "=============="
