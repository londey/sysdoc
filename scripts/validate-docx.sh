#!/bin/bash
# validate-docx.sh - Build test fixtures and validate DOCX output with OOXML Validator
#
# Prerequisites:
#   - .NET SDK installed
#   - OOXMLValidator installed: dotnet tool install -g OOXMLValidator
#
# Usage:
#   ./scripts/validate-docx.sh [--install-validator]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
FIXTURES_DIR="$PROJECT_ROOT/tests/fixtures"
BUILD_DIR="$PROJECT_ROOT/target/docx-validation"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Install validator if requested
if [[ "$1" == "--install-validator" ]]; then
    echo "Installing OOXMLValidator..."
    dotnet tool install -g OOXMLValidator || dotnet tool update -g OOXMLValidator
    exit 0
fi

# Check if OOXMLValidator is available
if ! command -v OOXMLValidator &> /dev/null; then
    echo -e "${YELLOW}Warning: OOXMLValidator not found. Install with:${NC}"
    echo "  dotnet tool install -g OOXMLValidator"
    echo ""
    echo "Alternatively, run: ./scripts/validate-docx.sh --install-validator"
    exit 1
fi

# Build sysdoc if needed
echo "Building sysdoc..."
cd "$PROJECT_ROOT"
cargo build --release

SYSDOC="$PROJECT_ROOT/target/release/sysdoc"
if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" || "$OSTYPE" == "cygwin" ]]; then
    SYSDOC="$SYSDOC.exe"
fi

# Create build directory
mkdir -p "$BUILD_DIR"

# Track results
TOTAL=0
PASSED=0
FAILED=0
FAILED_TESTS=""

# Test fixtures
TEST_CASES=(
    "test-normal-text"
    "test-italics"
    "test-bold"
    "test-strikethrough"
    "test-png-image"
    "test-svg-image"
    "test-csv-table"
    "test-inline-table"
)

echo ""
echo "========================================="
echo "DOCX Validation Test Suite"
echo "========================================="
echo ""

for test_case in "${TEST_CASES[@]}"; do
    TOTAL=$((TOTAL + 1))
    TEST_DIR="$FIXTURES_DIR/$test_case"
    OUTPUT_FILE="$BUILD_DIR/${test_case}.docx"

    echo -n "Testing $test_case... "

    # Build the fixture
    if ! "$SYSDOC" build "$TEST_DIR" -o "$OUTPUT_FILE" 2>/dev/null; then
        echo -e "${RED}BUILD FAILED${NC}"
        FAILED=$((FAILED + 1))
        FAILED_TESTS="$FAILED_TESTS\n  - $test_case (build failed)"
        continue
    fi

    # Validate with OOXMLValidator
    VALIDATION_OUTPUT=$(OOXMLValidator "$OUTPUT_FILE" 2>&1) || true

    # Check if validation passed (empty JSON array [] means no errors)
    if echo "$VALIDATION_OUTPUT" | grep -q '^\[\]$' || echo "$VALIDATION_OUTPUT" | grep -q '"errors":\s*\[\]'; then
        echo -e "${GREEN}PASSED${NC}"
        PASSED=$((PASSED + 1))
    elif echo "$VALIDATION_OUTPUT" | grep -qiE 'error|invalid|failed'; then
        echo -e "${RED}FAILED${NC}"
        FAILED=$((FAILED + 1))
        FAILED_TESTS="$FAILED_TESTS\n  - $test_case"
        echo "    Validation errors:"
        echo "$VALIDATION_OUTPUT" | head -20 | sed 's/^/    /'
    else
        # No errors found in output
        echo -e "${GREEN}PASSED${NC}"
        PASSED=$((PASSED + 1))
    fi
done

echo ""
echo "========================================="
echo "Results: $PASSED/$TOTAL passed"
echo "========================================="

if [[ $FAILED -gt 0 ]]; then
    echo -e "${RED}Failed tests:${NC}"
    echo -e "$FAILED_TESTS"
    exit 1
else
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi
