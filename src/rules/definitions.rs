#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RuleStatus {
    Continue,
    Stop
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ActionStatus {
    Accept,
    Reject
}

