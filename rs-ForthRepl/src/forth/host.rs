pub trait Host {
    // read_file
    fn println(&mut self, line: &str) -> crate::Result;
}

// TO BE CONTINUED
// |-> Delegate<'a>
// |-> Interpreter<'a>
// |-> PrintLnDelegate : Delegate<'static>

///////////////////
// Standard Host //
///////////////////

#[non_exhaustive]
pub struct StandardHost;

impl StandardHost {
    pub fn new() -> Self { Self }
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

#[cfg(test)]
pub mod tests {
    use super::*;

    pub struct TestHost {
        lines: Vec<String>,
    }

    impl TestHost {
        pub fn new() -> Self { Self { lines: Vec::new() } }
    }

    impl Host for TestHost {
        fn println(&mut self, line: &str) -> crate::Result {
            self.lines.push(line.to_string());
            Ok(())
        }
    }
}
