#!/bin/bash
# Anya Core Interactive Dashboard
# [AIR-3][AIS-3][AIT-3][AIP-3][RES-3]

set -eo pipefail

# Script version
VERSION="1.0.0"

# Directory setup
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"
INSTALL_DIR="${PROJECT_ROOT}/scripts/install"
UTILS_DIR="${INSTALL_DIR}/utils"
LOG_DIR="${PROJECT_ROOT}/logs"
DATA_DIR="${PROJECT_ROOT}/data"

# Source common utilities if available
if [ -f "${UTILS_DIR}/install_common.sh" ]; then
    source "${UTILS_DIR}/install_common.sh"
fi

# Colors and formatting
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
BLUE="\033[0;34m"
MAGENTA="\033[0;35m"
CYAN="\033[0;36m"
GRAY="\033[0;37m"
BOLD="\033[1m"
NC="\033[0m"

# Dashboard refresh rate (seconds)
REFRESH_RATE=3

# Terminal size
TERM_COLS=$(tput cols 2>/dev/null || echo 80)
TERM_ROWS=$(tput lines 2>/dev/null || echo 24)

# Ensure minimum size
if [ "$TERM_COLS" -lt 80 ]; then TERM_COLS=80; fi
if [ "$TERM_ROWS" -lt 24 ]; then TERM_ROWS=24; fi

# Check if a command exists
command_exists() {
    command -v "$1" &> /dev/null
}

# Print header
print_dashboard_header() {
    local width="$TERM_COLS"
    local title="Anya Core System Dashboard v$VERSION"
    local timestamp=$(date "+%Y-%m-%d %H:%M:%S")
    local padding=$(( (width - ${#title} - ${#timestamp} - 4) / 2 ))
    
    echo -e "${BOLD}${BLUE}"
    printf "%${width}s\n" | tr ' ' '='
    printf "%s%${padding}s%s%${padding}s%s\n" "=" " " "$title" " " "="
    printf "%${width}s\n" | tr ' ' '='
    echo -e "${NC}"
    echo -e "${GRAY}Refreshed: $timestamp${NC}\n"
}

# Get system stats
get_system_stats() {
    echo -e "${BOLD}${CYAN}System Statistics:${NC}"
    
    # CPU usage
    if command_exists mpstat; then
        CPU_USAGE=$(mpstat 1 1 | awk '/Average:/ {print 100 - $NF}')
    else
        CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4}')
    fi
    
    # Memory usage
    MEM_TOTAL=$(free -m | awk '/Mem:/ {print $2}')
    MEM_USED=$(free -m | awk '/Mem:/ {print $3}')
    MEM_PERCENT=$((MEM_USED * 100 / MEM_TOTAL))
    
    # Disk usage
    DISK_USAGE=$(df -h "${PROJECT_ROOT}" | awk 'NR==2 {print $5}' | tr -d '%')
    
    # Load average
    LOAD_AVG=$(cat /proc/loadavg | awk '{print $1}')
    
    # Uptime
    UPTIME=$(uptime -p | sed 's/up //')
    
    # Format stats with progress bars
    print_progress_bar "CPU Usage" "${CPU_USAGE:-0}%" 100
    print_progress_bar "Memory" "$MEM_PERCENT%" 100
    print_progress_bar "Disk" "$DISK_USAGE%" 100
    
    echo -e "Load Average: ${YELLOW}$LOAD_AVG${NC}"
    echo -e "System Uptime: ${YELLOW}$UPTIME${NC}"
}

# Print a progress bar
print_progress_bar() {
    local label="$1"
    local percent="$2"
    local max="$3"
    
    # Remove % symbol and extract only the integer part for calculations
    local value=$(echo "${percent//%}" | cut -d. -f1)
    # Default to 0 if empty or non-numeric
    value=${value:-0}
    
    local bar_length=40
    local filled=$(( value * bar_length / max ))
    local empty=$(( bar_length - filled ))
    
    # Choose color based on percentage
    local color=$GREEN
    if [ "$value" -ge 80 ]; then
        color=$RED
    elif [ "$value" -ge 60 ]; then
        color=$YELLOW
    fi
    
    # Print the bar
    printf "%-15s [" "$label"
    printf "%${filled}s" | tr ' ' '█'
    printf "%${empty}s" | tr ' ' '░'
    printf "] ${color}%5s${NC}\n" "$percent"
}

# Get Anya Core service status
get_service_status() {
    echo -e "${BOLD}${CYAN}Anya Core Service:${NC}"
    
    # Check if service is installed and running
    if systemctl list-unit-files | grep -q anya-core; then
        SERVICE_STATUS=$(systemctl is-active anya-core)
        SERVICE_ENABLED=$(systemctl is-enabled anya-core 2>/dev/null || echo "disabled")
        
        if [ "$SERVICE_STATUS" = "active" ]; then
            echo -e "Service Status: ${GREEN}●${NC} Active"
        else
            echo -e "Service Status: ${RED}●${NC} Inactive"
        fi
        
        if [ "$SERVICE_ENABLED" = "enabled" ]; then
            echo -e "Auto-start: ${GREEN}●${NC} Enabled"
        else
            echo -e "Auto-start: ${RED}●${NC} Disabled"
        fi
        
        # Get service runtime stats
        RUNTIME=$(systemctl show anya-core -p ActiveEnterTimestamp | cut -d= -f2)
        if [ -n "$RUNTIME" ]; then
            echo -e "Running since: ${YELLOW}$RUNTIME${NC}"
        fi
        
        # Get memory and CPU usage for the service
        if command_exists systemctl; then
            MEM_USAGE=$(systemctl status anya-core | grep Memory | awk '{print $2}')
            if [ -n "$MEM_USAGE" ]; then
                echo -e "Memory Usage: ${YELLOW}$MEM_USAGE${NC}"
            fi
        fi
    else
        echo -e "Service Status: ${GRAY}Not Installed${NC}"
    fi
}

# Get log summary
get_log_summary() {
    echo -e "${BOLD}${CYAN}Recent Logs:${NC}"
    
    # Find the most recent log file
    RECENT_LOG=$(find "$LOG_DIR" -name "*.log" -type f -mtime -1 | sort -r | head -n 1)
    
    if [ -n "$RECENT_LOG" ]; then
        echo -e "Log File: ${BLUE}$RECENT_LOG${NC}"
        
        # Count error and warning entries
        ERROR_COUNT=$(grep -c "ERROR" "$RECENT_LOG" 2>/dev/null || echo 0)
        WARN_COUNT=$(grep -c "WARN" "$RECENT_LOG" 2>/dev/null || echo 0)
        
        echo -e "Errors: ${RED}$ERROR_COUNT${NC} | Warnings: ${YELLOW}$WARN_COUNT${NC}"
        
        # Show the 5 most recent log entries
        echo -e "\n${GRAY}Recent entries:${NC}"
        tail -n 5 "$RECENT_LOG" | sed -E "s/ERROR/${RED}ERROR${NC}/g" | sed -E "s/WARN/${YELLOW}WARN${NC}/g" | sed -E "s/INFO/${GREEN}INFO${NC}/g"
    else
        echo -e "${GRAY}No recent logs found${NC}"
    fi
}

# Get feature status
get_feature_status() {
    echo -e "${BOLD}${CYAN}Anya Core Features:${NC}"
    
    # Check for installed binary
    BINARY_PATH="${PROJECT_ROOT}/target/release/anya-core"
    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        echo -e "Binary: ${GREEN}●${NC} Installed (${YELLOW}$BINARY_SIZE${NC})"
        
        # Check which features are enabled by inspecting the binary
        if command_exists strings && strings "$BINARY_PATH" | grep -q "feature=\"hsm\""; then
            echo -e "HSM Support: ${GREEN}●${NC} Enabled"
        else
            echo -e "HSM Support: ${GRAY}●${NC} Disabled"
        fi
        
        if command_exists strings && strings "$BINARY_PATH" | grep -q "feature=\"dao_governance\""; then
            echo -e "DAO Governance: ${GREEN}●${NC} Enabled"
        else
            echo -e "DAO Governance: ${GRAY}●${NC} Disabled"
        fi
        
        if command_exists strings && strings "$BINARY_PATH" | grep -q "feature=\"lightning\""; then
            echo -e "Lightning Support: ${GREEN}●${NC} Enabled"
        else
            echo -e "Lightning Support: ${GRAY}●${NC} Disabled"
        fi
    else
        echo -e "Binary: ${RED}●${NC} Not Installed"
    fi
    
    # Check for config file
    if [ -f "${PROJECT_ROOT}/config/anya.conf" ]; then
        echo -e "Configuration: ${GREEN}●${NC} Present"
        
        # Get network type from config
        NETWORK=$(grep "network" "${PROJECT_ROOT}/config/anya.conf" | cut -d= -f2 | tr -d ' ' 2>/dev/null || echo "unknown")
        if [ -n "$NETWORK" ] && [ "$NETWORK" != "unknown" ]; then
            echo -e "Network: ${YELLOW}$NETWORK${NC}"
        fi
    else
        echo -e "Configuration: ${RED}●${NC} Missing"
    fi
}

# Get installation status
get_installation_status() {
    echo -e "${BOLD}${CYAN}Installation Status:${NC}"
    
    # Check if installation is in progress
    if [ -f "${PROJECT_ROOT}/.anya_install.lock" ]; then
        LOCK_PID=$(cat "${PROJECT_ROOT}/.anya_install.lock" 2>/dev/null || echo "none")
        if ps -p "$LOCK_PID" > /dev/null 2>&1; then
            echo -e "Status: ${YELLOW}●${NC} Installation in Progress (PID: $LOCK_PID)"
            
            # Try to determine which stage it's at
            if pgrep -f "auto_install.sh" > /dev/null; then
                echo -e "Stage: ${BLUE}Hardware Detection & Setup${NC}"
            elif pgrep -f "linux_install.sh" > /dev/null; then
                echo -e "Stage: ${BLUE}Core Installation${NC}"
                
                # If building, try to estimate progress
                if pgrep -f "cargo build" > /dev/null; then
                    echo -e "Building: ${YELLOW}In Progress${NC}"
                    
                    # Estimate progress based on CPU activity
                    CPU_USAGE=$(ps -p $(pgrep -f "cargo build") -o %cpu | tail -n 1 | tr -d ' ')
                    ELAPSED_TIME=$(ps -p $(pgrep -f "cargo build") -o etime | tail -n 1 | tr -d ' ')
                    
                    echo -e "Build Activity: ${YELLOW}CPU ${CPU_USAGE}% | Running for ${ELAPSED_TIME}${NC}"
                fi
            elif pgrep -f "systemd_config.sh" > /dev/null; then
                echo -e "Stage: ${BLUE}Service Configuration${NC}"
            fi
        else
            echo -e "Status: ${RED}●${NC} Stale Installation Lock (PID: $LOCK_PID not running)"
        fi
    else
        # Check if installed
        if [ -f "${PROJECT_ROOT}/target/release/anya-core" ] && systemctl list-unit-files | grep -q anya-core; then
            echo -e "Status: ${GREEN}●${NC} Installed and Configured"
            
            # Try to get version
            VERSION_FILE="${PROJECT_ROOT}/.version"
            if [ -f "$VERSION_FILE" ]; then
                INSTALLED_VERSION=$(cat "$VERSION_FILE" 2>/dev/null || echo "unknown")
                echo -e "Version: ${YELLOW}$INSTALLED_VERSION${NC}"
            fi
        elif [ -f "${PROJECT_ROOT}/target/release/anya-core" ]; then
            echo -e "Status: ${YELLOW}●${NC} Partially Installed (Binary Only)"
        else
            echo -e "Status: ${GRAY}●${NC} Not Installed"
        fi
    fi
}

# Display block with actions
display_actions() {
    echo -e "\n${BOLD}${CYAN}Available Actions:${NC}"
    echo -e "${GRAY}Press key to perform action:${NC}"
    echo -e "${GREEN}i${NC} - Install/Upgrade Anya Core"
    echo -e "${RED}u${NC} - Uninstall Anya Core"
    echo -e "${YELLOW}r${NC} - Restart Service"
    echo -e "${BLUE}l${NC} - View Full Logs"
    echo -e "${MAGENTA}t${NC} - Run Tests"
    echo -e "${CYAN}c${NC} - Edit Configuration"
    echo -e "${GREEN}s${NC} - Start Service"
    echo -e "${RED}p${NC} - Stop Service"
    echo -e "${GRAY}q${NC} - Quit Dashboard"
    
    if [ -t 0 ]; then  # Only if running in interactive terminal
        read -t 0.1 -n 1 key 2>/dev/null || true
        handle_key_press "$key"
    fi
}

# Handle key press for actions
handle_key_press() {
    local key="$1"
    case "$key" in
        i|I)
            clear
            echo -e "${GREEN}Starting Installation...${NC}"
            if [ -f "${SCRIPT_DIR}/auto_install.sh" ]; then
                sudo "${SCRIPT_DIR}/auto_install.sh" --auto-run
            else
                sudo "${INSTALL_DIR}/auto_install.sh" --auto-run
            fi
            echo -e "${GREEN}Press any key to continue...${NC}"
            read -n 1
            ;;
        u|U)
            clear
            echo -e "${RED}Starting Uninstallation...${NC}"
            sudo "${INSTALL_DIR}/uninstall.sh"
            echo -e "${GREEN}Press any key to continue...${NC}"
            read -n 1
            ;;
        r|R)
            echo -e "${YELLOW}Restarting Anya Core service...${NC}"
            sudo systemctl restart anya-core
            echo -e "${GREEN}Service restarted.${NC}"
            sleep 2
            ;;
        l|L)
            clear
            if command_exists less; then
                RECENT_LOG=$(find "$LOG_DIR" -name "*.log" -type f -mtime -1 | sort -r | head -n 1)
                if [ -n "$RECENT_LOG" ]; then
                    less "$RECENT_LOG"
                else
                    echo -e "${RED}No recent logs found.${NC}"
                    sleep 2
                fi
            else
                RECENT_LOG=$(find "$LOG_DIR" -name "*.log" -type f -mtime -1 | sort -r | head -n 1)
                if [ -n "$RECENT_LOG" ]; then
                    tail -n 50 "$RECENT_LOG"
                    echo -e "\n${GREEN}Press any key to continue...${NC}"
                    read -n 1
                else
                    echo -e "${RED}No recent logs found.${NC}"
                    sleep 2
                fi
            fi
            ;;
        t|T)
            clear
            echo -e "${MAGENTA}Running Tests...${NC}"
            if [ -f "${PROJECT_ROOT}/scripts/test/test_installation.sh" ]; then
                sudo "${PROJECT_ROOT}/scripts/test/test_installation.sh"
            else
                echo -e "${RED}Test script not found.${NC}"
            fi
            echo -e "${GREEN}Press any key to continue...${NC}"
            read -n 1
            ;;
        c|C)
            CONF_FILE="${PROJECT_ROOT}/config/anya.conf"
            if [ -f "$CONF_FILE" ]; then
                if command_exists nano; then
                    sudo nano "$CONF_FILE"
                elif command_exists vim; then
                    sudo vim "$CONF_FILE"
                else
                    echo -e "${RED}No editor found. Please install nano or vim.${NC}"
                    sleep 2
                fi
            else
                echo -e "${RED}Configuration file not found.${NC}"
                sleep 2
            fi
            ;;
        s|S)
            echo -e "${GREEN}Starting Anya Core service...${NC}"
            sudo systemctl start anya-core
            echo -e "${GREEN}Service started.${NC}"
            sleep 2
            ;;
        p|P)
            echo -e "${RED}Stopping Anya Core service...${NC}"
            sudo systemctl stop anya-core
            echo -e "${RED}Service stopped.${NC}"
            sleep 2
            ;;
        q|Q)
            echo -e "${GRAY}Exiting dashboard.${NC}"
            exit 0
            ;;
    esac
}

# Display the dashboard
display_dashboard() {
    clear
    print_dashboard_header
    
    # Layout - dynamically adjust based on terminal size
    if [ "$TERM_ROWS" -gt 40 ]; then
        # Two-column layout for larger terminals
        # Use temporary files for each section
        TEMP_DIR=$(mktemp -d)
        STATS_FILE="$TEMP_DIR/stats"
        SERVICE_FILE="$TEMP_DIR/service"
        FEATURE_FILE="$TEMP_DIR/feature"
        INSTALL_FILE="$TEMP_DIR/install"
        LOG_FILE="$TEMP_DIR/log"
        
        # Generate each section in parallel
        get_system_stats > "$STATS_FILE" &
        get_service_status > "$SERVICE_FILE" &
        get_feature_status > "$FEATURE_FILE" &
        get_installation_status > "$INSTALL_FILE" &
        get_log_summary > "$LOG_FILE" &
        wait
        
        # Calculate column width
        COL_WIDTH=$(( TERM_COLS / 2 - 2 ))
        
        # Read each section
        readarray -t STATS < "$STATS_FILE"
        readarray -t SERVICE < "$SERVICE_FILE"
        readarray -t FEATURE < "$FEATURE_FILE"
        readarray -t INSTALL < "$INSTALL_FILE"
        readarray -t LOG < "$LOG_FILE"
        
        # Display sections side by side
        paste_sections "$COL_WIDTH" STATS SERVICE
        echo
        paste_sections "$COL_WIDTH" FEATURE INSTALL
        echo
        
        # Display log section full width
        for line in "${LOG[@]}"; do
            echo -e "$line"
        done
        
        # Clean up
        rm -rf "$TEMP_DIR"
    else
        # Single column layout for smaller terminals
        get_system_stats
        echo
        get_service_status
        echo
        get_feature_status
        echo
        get_installation_status
        echo
        get_log_summary
    fi
    
    # Display actions
    display_actions
}

# Helper function to paste sections side by side
paste_sections() {
    local width="$1"
    local left_array="$2[@]"
    local right_array="$3[@]"
    local left=("${!left_array}")
    local right=("${!right_array}")
    local max_rows=$(( ${#left[@]} > ${#right[@]} ? ${#left[@]} : ${#right[@]} ))
    
    for ((i=0; i<max_rows; i++)); do
        if [ $i -lt ${#left[@]} ]; then
            printf "%-${width}s" "${left[$i]}"
        else
            printf "%-${width}s" ""
        fi
        printf "  "
        if [ $i -lt ${#right[@]} ]; then
            echo -e "${right[$i]}"
        else
            echo
        fi
    done
}

# Refresh dashboard until quit
start_dashboard() {
    while true; do
        display_dashboard
        sleep "$REFRESH_RATE"
    done
}

# Main function
main() {
    # Check if running in interactive mode
    if [ ! -t 0 ]; then
        echo "This dashboard requires an interactive terminal."
        exit 1
    fi
    
    # Check for dependencies
    if ! command_exists tput; then
        echo "Warning: 'tput' not found. Dashboard may not display correctly."
    fi
    
    # Handle CTRL+C gracefully
    trap "echo -e '${GRAY}Exiting dashboard.${NC}'; exit 0" INT
    
    # Start the dashboard
    start_dashboard
}

# Run the main function
main "$@"
