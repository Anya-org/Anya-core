# BIP Health Check Makefile

.PHONY: check report monitor clean

# Default variables
FORMAT ?= markdown
OUTPUT_DIR ?= ../../reports/bip
TIMESTAMP := $(shell date +%Y%m%d%H%M%S)
OUTPUT ?= $(OUTPUT_DIR)/bip-health-$(TIMESTAMP).md
VERBOSE ?= 

# Add verbose flag if requested
ifdef VERBOSE
VERBOSE_FLAG = --verbose
endif

# Make sure output directory exists
$(OUTPUT_DIR):
	mkdir -p $(OUTPUT_DIR)

# Build the BIP health checker
build:
	cd ../../ && cargo build --bin bip_health

# Check BIP status and print to console
check: build
	../../target/debug/bip_health $(VERBOSE_FLAG) check --format $(FORMAT)

# Generate a report and save to file
report: build $(OUTPUT_DIR)
	../../target/debug/bip_health $(VERBOSE_FLAG) report --format $(FORMAT) --output $(OUTPUT)
	@echo "Report saved to: $(OUTPUT)"

# Start monitoring mode
monitor: build $(OUTPUT_DIR)
	../../target/debug/bip_health $(VERBOSE_FLAG) monitor --output-dir $(OUTPUT_DIR)

# Run a full update check and save report
update: build $(OUTPUT_DIR)
	@echo "Checking BIP status..."
	../../target/debug/bip_health $(VERBOSE_FLAG) report --format markdown --output $(OUTPUT)
	@echo "Full BIP health check completed"
	@echo "Report saved to: $(OUTPUT)"

# Clean up reports older than 30 days
clean:
	find $(OUTPUT_DIR) -name "bip-health-*.md" -type f -mtime +30 -delete
	find $(OUTPUT_DIR) -name "bip-health-*.json" -type f -mtime +30 -delete
	@echo "Cleaned up old reports"

# Help message
help:
	@echo "BIP Health Check Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make [target] [OPTION=value]"
	@echo ""
	@echo "Targets:"
	@echo "  check    - Check BIP status and print to console"
	@echo "  report   - Generate a report and save to file"
	@echo "  monitor  - Start monitoring mode"
	@echo "  update   - Run a full update check and save report"
	@echo "  clean    - Clean up reports older than 30 days"
	@echo "  help     - Show this help message"
	@echo ""
	@echo "Options:"
	@echo "  FORMAT=format   - Output format (markdown, json, text)"
	@echo "  OUTPUT=file     - Output file path"
	@echo "  VERBOSE=1       - Enable verbose output"
	@echo ""
	@echo "Examples:"
	@echo "  make check FORMAT=json VERBOSE=1"
	@echo "  make report FORMAT=markdown OUTPUT=./my-report.md"
	@echo "  make monitor" 