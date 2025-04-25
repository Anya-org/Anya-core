use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossterm::{
    cursor, execute, terminal,
    style::{self, Color, Stylize},
};
use std::io::{stdout, Write};

/// Dashboard configuration
pub struct DashboardConfig {
    pub title: String,
    pub refresh_rate_ms: u64,
    pub show_spinner: bool,
    pub show_progress_bar: bool,
    pub show_details: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            title: "Anya-Core Dashboard".to_string(),
            refresh_rate_ms: 500,
            show_spinner: true,
            show_progress_bar: true,
            show_details: true,
        }
    }
}

/// Dashboard state
pub struct DashboardState {
    pub current_operation: String,
    pub operation_type: OperationType,
    pub progress: usize,
    pub total: usize,
    pub details: Vec<String>,
    pub is_running: bool,
}

impl Default for DashboardState {
    fn default() -> Self  -> Result<(), Box<dyn Error>> {
        Self {
            current_operation: "Initializing...".to_string(),
            operation_type: OperationType::Info,
            progress: 0,
            total: 100,
            details: Vec::new(),
            is_running: true,
        }
    }
}

/// Operation type enum
#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Info,
    Warning,
    Error,
    Success,
}

/// The dashboard controller
pub struct Dashboard {
    config: DashboardConfig,
    state: Arc<Mutex<DashboardState>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Dashboard {
    /// Create a new dashboard
    pub fn new(config: DashboardConfig) -> Self  -> Result<(), Box<dyn Error>> {
        let state = Arc::new(Mutex::new(DashboardState::default()));
        Self {
            config,
            state,
            handle: None,
        }
    }
    
    /// Start the dashboard
    pub fn start(&mut self)  -> Result<(), Box<dyn Error>> {
        // Save terminal state
        execute!(stdout(), terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        
        // Clone state for the dashboard thread
        let state = Arc::clone(&self.state);
        let config = self.config.clone();
        
        // Create dashboard thread
        let handle = thread::spawn(move || {
            let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let mut spinner_idx = 0;
            
            while state.lock()?.is_running {
                // Clear the screen
                execute!(
                    stdout(),
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0, 0)
                )?;
                
                // Get the current state
                let dash_state = state.lock()?.clone();
                
                // Print the title
                let title = format!(" {} ", config.title);
                println!("{}", title.black().on_white());
                println!();
                
                // Print current operation with spinner
                let operation_color = match dash_state.operation_type {
                    OperationType::Info => Color::Blue,
                    OperationType::Warning => Color::Yellow,
                    OperationType::Error => Color::Red,
                    OperationType::Success => Color::Green,
                };
                
                let spinner = if config.show_spinner {
                    spinner_chars[spinner_idx]
                } else {
                    ' '
                };
                
                println!("{} {}", 
                    spinner.with(operation_color),
                    dash_state.current_operation.with(operation_color)
                );
                
                // Print progress bar
                if config.show_progress_bar && dash_state.total > 0 {
                    let progress_percentage = (dash_state.progress as f64 / dash_state.total as f64 * 100.0) as usize;
                    let bar_width = 50;
                    let filled_width = (bar_width as f64 * progress_percentage as f64 / 100.0) as usize;
                    
                    print!("[");
                    for i in 0..bar_width {
                        if i < filled_width {
                            print!("{}", "█".green());
                        } else {
                            print!(" ");
                        }
                    }
                    print!("] {}%\n", progress_percentage);
                }
                
                println!();
                
                // Print details
                if config.show_details {
                    for detail in dash_state.details.iter().rev().take(10) {
                        println!("  {}", detail);
                    }
                }
                
                // Update spinner index
                spinner_idx = (spinner_idx + 1) % spinner_chars.len();
                
                // Flush and sleep
                stdout().flush()?;
                thread::sleep(Duration::from_millis(config.refresh_rate_ms));
            }
            
            // Restore terminal state
            terminal::disable_raw_mode()?;
            execute!(stdout(), terminal::LeaveAlternateScreen)?;
        });
        
        self.handle = Some(handle);
    }
    
    /// Update the dashboard state
    pub fn update(&self, update_fn: impl FnOnce(&mut DashboardState))  -> Result<(), Box<dyn Error>> {
        if let Ok(mut state) = self.state.lock() {
            update_fn(&mut state);
        }
    }
    
    /// Set the current operation
    pub fn set_operation(&self, operation: &str, operation_type: OperationType)  -> Result<(), Box<dyn Error>> {
        self.update(|state| {
            state.current_operation = operation.to_string();
            state.operation_type = operation_type;
            state.details.push(format!("{}: {}", 
                match operation_type {
                    OperationType::Info => "INFO",
                    OperationType::Warning => "WARN",
                    OperationType::Error => "ERROR",
                    OperationType::Success => "SUCCESS",
                },
                operation
            ));
        });
    }
    
    /// Set the progress
    pub fn set_progress(&self, progress: usize, total: usize)  -> Result<(), Box<dyn Error>> {
        self.update(|state| {
            state.progress = progress;
            state.total = total;
        });
    }
    
    /// Add a detail message
    pub fn add_detail(&self, detail: &str)  -> Result<(), Box<dyn Error>> {
        self.update(|state| {
            state.details.push(detail.to_string());
        });
    }
    
    /// Stop the dashboard
    pub fn stop(&mut self)  -> Result<(), Box<dyn Error>> {
        self.update(|state| {
            state.is_running = false;
        });
        
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for Dashboard {
    fn drop(&mut self)  -> Result<(), Box<dyn Error>> {
        self.stop();
    }
} 
