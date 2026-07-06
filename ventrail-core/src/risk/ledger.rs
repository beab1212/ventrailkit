//! Shared session-owned cached views for HVWS risk contracts.

#[derive(Clone, Debug)]
pub struct RiskViewSlot {
    pub storage: Vec<u8>,
    pub view: *const u8,
    pub len: usize,
    pub generation: u32,
    pub view_generation: u32,
}

impl Default for RiskViewSlot {
    fn default() -> Self {
        Self {
            storage: Vec::new(),
            view: std::ptr::null(),
            len: 0,
            generation: 0,
            view_generation: 0,
        }
    }
}

pub struct RiskSessionState {
    pub slots: Vec<RiskViewSlot>,
}

impl Default for RiskSessionState {
    fn default() -> Self {
        Self { slots: (0..8).map(|_| RiskViewSlot::default()).collect() }
    }
}

unsafe impl Send for RiskViewSlot {}
unsafe impl Sync for RiskViewSlot {}
