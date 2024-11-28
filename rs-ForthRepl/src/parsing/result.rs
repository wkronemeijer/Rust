// Never have been able to describe these well...
// It's EitherT fused to WriterT with a dash of MaybeT

use super::error::DiagnosticList;

#[derive(Debug, Clone)]
pub struct CompileResult<T> {
    /// Invariant:  
    /// `report.is_fatal()` &Implies; `value == None`  
    /// This is implication, not equivalence (!)
    value: Option<T>,
    report: DiagnosticList,
}

impl<T> CompileResult<T> {
    pub fn new(value: T, report: DiagnosticList) -> Self {
        let value = if report.is_fatal() { None } else { Some(value) };
        CompileResult { value, report }
    }

    pub fn fail(report: DiagnosticList) -> Self {
        CompileResult { value: None, report }
    }

    pub fn ok(self) -> Option<T> { self.value }

    pub fn into_result(self) -> Result<T, DiagnosticList> {
        let CompileResult { value, report } = self;
        match value {
            Some(inner) => Ok(inner),
            None => Err(report),
        }
    }

    pub fn report(&self) -> &DiagnosticList { &self.report }

    pub fn map<U, F>(self, func: F) -> CompileResult<U>
    where
        F: FnOnce(T) -> U,
    {
        let report = self.report;
        if let Some(t) = self.value {
            let u = func(t);
            CompileResult { value: Some(u), report }
        } else {
            CompileResult { value: None, report }
        }
    }

    pub fn and_then<U, F>(self, func: F) -> CompileResult<U>
    where
        F: FnOnce(T) -> CompileResult<U>,
    {
        self.map(func).flatten()
    }
}

impl<T> CompileResult<CompileResult<T>> {
    pub fn flatten(self) -> CompileResult<T> {
        // Reminder: is_fatal ==> value == None
        // No need to check for is_fatal
        let outer_value = self.value;
        let outer_report = self.report;

        match outer_value {
            Some(res) => {
                let inner_value = res.value;
                let inner_report = res.report;
                CompileResult {
                    value: inner_value,
                    report: outer_report.join(inner_report),
                }
            }
            None => CompileResult { value: None, report: outer_report },
        }
    }
}

#[expect(unused)]
fn test() {
    // Little sandbox for finding the appropriate result methods to copy
    let result: Result<i32, bool> = Ok(42);
}
