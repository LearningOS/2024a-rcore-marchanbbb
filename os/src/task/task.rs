//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// the numbers of syscall called by the task
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
    /// the total running time of the task
    pub task_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
