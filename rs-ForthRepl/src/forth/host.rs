pub trait Host {
    // read_file
    fn println(&mut self, line: &str) -> crate::Result;
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
}

///////////////
// Test Host //
///////////////

/// TestHost stores all printed text internally.
pub struct TestHost {
    lines: Vec<String>,
}

impl TestHost {
    pub fn new() -> Self { TestHost { lines: Vec::new() } }
}

impl Host for TestHost {
    fn println(&mut self, line: &str) -> crate::Result {
        self.lines.push(line.to_string());
        Ok(())
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
