/**
 * @fileoverview Unified Logging System
 * 
 * This module provides standardized logging capabilities across all DAO tools,
 * with support for multiple output formats, log levels, and integrations.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

// Load environment variables
const LOG_LEVEL = process.env.ANYA_LOG_LEVEL || 'info';
const LOG_FORMAT = process.env.ANYA_LOG_FORMAT || 'text';
const LOG_FILE = process.env.ANYA_LOG_FILE || path.join(os.homedir(), '.anya', 'logs', 'dao.log');

// Log levels with numeric values for comparison
const LOG_LEVELS = {
    trace: 10,
    debug: 20,
    info: 30,
    warn: 40,
    error: 50,
    fatal: 60,
    silent: 70
};

/**
 * Unified Logger class
 */
class UnifiedLogger {
    /**
     * Constructor for UnifiedLogger
     * @param {Object} options - Logger options
     * @param {string} options.level - Log level
     * @param {string} options.format - Log format ('text', 'json')
     * @param {string} options.logFile - Path to log file
     * @param {boolean} options.console - Whether to log to console
     * @param {boolean} options.file - Whether to log to file
     */
    constructor(options = {}) {
        this.level = options.level || LOG_LEVEL;
        this.format = options.format || LOG_FORMAT;
        this.logFile = options.logFile || LOG_FILE;
        this.console = options.console !== undefined ? options.console : true;
        this.file = options.file !== undefined ? options.file : true;

        // Numeric log level for comparison
        this.levelValue = LOG_LEVELS[this.level] || LOG_LEVELS.info;

        // Create log directory if it doesn't exist
        if (this.file) {
            const logDir = path.dirname(this.logFile);
            if (!fs.existsSync(logDir)) {
                fs.mkdirSync(logDir, { recursive: true });
            }
        }
    }

    /**
     * Log a message at the specified level
     * @param {string} level - Log level
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     * @private
     */
    _log(level, message, data = {}) {
        const levelValue = LOG_LEVELS[level];

        // Skip if log level is below the configured level
        if (levelValue < this.levelValue) {
            return;
        }

        // Create log entry
        const timestamp = new Date().toISOString();
        const entry = {
            timestamp,
            level,
            message,
            ...data
        };

        // Format log entry
        let formattedEntry;
        if (this.format === 'json') {
            formattedEntry = JSON.stringify(entry);
        } else {
            formattedEntry = `${timestamp} [${level.toUpperCase()}] ${message}`;
            if (Object.keys(data).length > 0) {
                formattedEntry += ' ' + JSON.stringify(data);
            }
        }

        // Write to console
        if (this.console) {
            const consoleMethod = level === 'error' || level === 'fatal' ? 'error' :
                level === 'warn' ? 'warn' : 'log';
            console[consoleMethod](formattedEntry);
        }

        // Write to file
        if (this.file) {
            fs.appendFileSync(this.logFile, formattedEntry + '\n');
        }
    }

    /**
     * Log a trace message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    trace(message, data = {}) {
        this._log('trace', message, data);
    }

    /**
     * Log a debug message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    debug(message, data = {}) {
        this._log('debug', message, data);
    }

    /**
     * Log an info message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    info(message, data = {}) {
        this._log('info', message, data);
    }

    /**
     * Log a warning message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    warn(message, data = {}) {
        this._log('warn', message, data);
    }

    /**
     * Log an error message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    error(message, data = {}) {
        this._log('error', message, data);
    }

    /**
     * Log a fatal error message
     * @param {string} message - Log message
     * @param {Object} data - Additional data to log
     */
    fatal(message, data = {}) {
        this._log('fatal', message, data);
    }

    /**
     * Set log level
     * @param {string} level - Log level
     */
    setLevel(level) {
        if (LOG_LEVELS[level]) {
            this.level = level;
            this.levelValue = LOG_LEVELS[level];
        }
    }

    /**
     * Clear log file
     */
    clearLogFile() {
        if (this.file && fs.existsSync(this.logFile)) {
            fs.writeFileSync(this.logFile, '');
        }
    }

    /**
     * Get log file contents
     * @param {number} maxLines - Maximum number of lines to get
     * @returns {string} - Log file contents
     */
    getLogFileContents(maxLines = 100) {
        if (!this.file || !fs.existsSync(this.logFile)) {
            return '';
        }

        const fileContent = fs.readFileSync(this.logFile, 'utf8');
        const lines = fileContent.split('\n');

        return lines.slice(-maxLines).join('\n');
    }
}

// Export a singleton instance
const logger = new UnifiedLogger();

module.exports = logger;
