// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Timeout,
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionStatus::Pending => write!(f, "pending"),
            ExecutionStatus::Running => write!(f, "running"),
            ExecutionStatus::Completed => write!(f, "completed"),
            ExecutionStatus::Failed => write!(f, "failed"),
            ExecutionStatus::Timeout => write!(f, "timeout"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeExecution {
    pub program_id: String,
    pub status: ExecutionStatus,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
}

impl CodeExecution {
    pub fn new<S: Into<String>>(program_id: S) -> Self {
        Self {
            program_id: program_id.into(),
            status: ExecutionStatus::Pending,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: None,
            duration_ms: 0,
        }
    }

    pub fn with_status(mut self, status: ExecutionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_output<S: Into<String>>(mut self, stdout: S, stderr: S) -> Self {
        self.stdout = stdout.into();
        self.stderr = stderr.into();
        self
    }

    pub fn with_exit_code(mut self, code: i32) -> Self {
        self.exit_code = Some(code);
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn is_success(&self) -> bool {
        matches!(self.status, ExecutionStatus::Completed) && self.exit_code == Some(0)
    }

    pub fn has_errors(&self) -> bool {
        !self.stderr.is_empty() || self.exit_code.map_or(false, |c| c != 0)
    }
}

impl fmt::Display for CodeExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Execution {} -> {}", self.program_id, self.status)
    }
}

