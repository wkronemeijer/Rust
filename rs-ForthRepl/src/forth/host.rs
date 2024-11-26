use std::collections::VecDeque;
use std::fs;

pub trait Host {
    fn println(&mut self, line: &str) -> crate::Result;
    fn read_file(&mut self, filename: &str) -> crate::Result<String>;
}

///////////////////
// Standard Host //
///////////////////
// TODO: Come up with a better name
// Something like ProdHost?

/// StandardHost prints to stdout. For testing use [TestHost].
#[non_exhaustive]
pub struct StandardHost;

impl StandardHost {
    pub fn new() -> Self { StandardHost }
}

impl Host for StandardHost {
    fn println(&mut self, line: &str) -> crate::Result {
        println!("{line}");
        Ok(())
    }

    fn read_file(&mut self, filename: &str) -> crate::Result<String> {
        fs::read_to_string(filename)
            .map_err(|e| crate::Error::IoError(e.to_string().into()))
    }
}

///////////////
// Test Host //
///////////////

/// TestHost stores all printed text internally.
pub struct TestHost {
    lines: VecDeque<String>,
    file_contents: Option<String>,
}

impl TestHost {
    pub fn new() -> Self {
        let lines = VecDeque::new();
        let file_contents = None;
        TestHost { lines, file_contents }
    }

    pub fn next_line(&mut self) -> Option<String> { self.lines.pop_front() }

    pub fn set_file_contents(&mut self, contents: String) {
        self.file_contents = Some(contents);
    }
}

impl Host for TestHost {
    fn println(&mut self, line: &str) -> crate::Result {
        self.lines.push_back(line.to_string());
        Ok(())
    }

    fn read_file(&mut self, _: &str) -> crate::Result<String> {
        if let Some(file) = self.file_contents.take() {
            Ok(file)
        } else {
            Err(crate::Error::IoError("file not found".into()))
        }
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_object_safe() {
        #[expect(unused)]
        let dyn_host: Box<dyn Host> = Box::new(TestHost::new());
    }
}
