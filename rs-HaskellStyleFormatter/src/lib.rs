use std::default::Default;
use std::fmt;
use std::fmt::Write;

use serde_json::value::Value;
use serde_json::Map;

pub type Result<T = (), E = Box<dyn std::error::Error>> =
    std::result::Result<T, E>;

/////////////
// Options //
/////////////

#[derive(Debug, Clone)]
pub struct Options {
    pub indent_size: u8,
    pub max_line_width: u16,
}

impl Default for Options {
    fn default() -> Self { Options { indent_size: 4, max_line_width: 80 } }
}

/////////////
// Context //
/////////////

#[derive(Debug)]
struct Context {
    buffer: String,
    primed: bool,
    indent_level: usize,
    indent_size: usize,
    padding_size: usize,
}

impl Context {
    pub fn new(options: &Options) -> Self {
        let buffer = String::new();
        let primed = false;
        let indent_level = 0;
        let indent_size = options.indent_size as usize;
        let padding_size = indent_size.saturating_sub(1);
        Context { buffer, primed, indent_level, indent_size, padding_size }
    }

    pub fn indent(&mut self) {
        self.indent_level = self.indent_level.saturating_add(1);
    }

    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }

    pub fn write_space(&mut self, count: usize) -> fmt::Result {
        for _ in 0..count {
            self.write_char(' ')?;
        }
        Ok(())
    }

    pub fn finish(self) -> String { self.buffer }

    fn check_primed(&mut self) -> Result<(), fmt::Error> {
        Ok(if self.primed {
            let indent = " ".repeat(self.indent_level * self.indent_size);
            self.buffer.write_str(&indent)?;
            self.primed = false;
        })
    }
}

impl Write for Context {
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.check_primed()?;
        self.buffer.write_char(c)?;
        if matches!(c, '\n') {
            self.primed = true;
        }
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.check_primed()?;
        self.buffer.write_str(s)?;
        if s.contains('\n') {
            self.primed = true;
        }
        Ok(())
    }

    // Ensure it trigger write_str to check for newlines
    // fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
    //     self.buffer.write_fmt(args)
    // }
}

////////////////
// Formatting //
////////////////

/// Falls back to default Serde formatting.
fn serde_fmt_value<W: Write>(f: &mut W, this: &Value) -> crate::Result {
    f.write_str(&serde_json::to_string(this)?)?;
    Ok(())
}

fn value_is_multline(this: &Value) -> bool {
    match this {
        Value::Null |
        Value::Bool(..) |
        Value::Number(..) |
        Value::String(..) => false,
        Value::Array(..) | Value::Object(..) => true,
    }
}

impl Context {
    fn fmt_string(&mut self, first_key: &String) -> crate::Result {
        // TODO: Make this more efficient
        self.fmt_value(&Value::String(first_key.to_string()))?;
        Ok(())
    }

    fn fmt_array(&mut self, array: &Vec<Value>) -> crate::Result {
        self.write_char('[')?;
        let mut values = array.iter();
        if let Some(first) = values.next() {
            self.write_space(self.padding_size)?;
            self.fmt_value(first)?;
            self.write_char('\n')?;

            for rest in values {
                self.write_char(',')?;
                self.write_space(self.padding_size)?;
                self.fmt_value(rest)?;
                self.write_char('\n')?;
            }
        }
        self.write_char(']')?;
        Ok(())
    }

    fn fmt_object(&mut self, object: &Map<String, Value>) -> crate::Result {
        self.write_char('{')?;
        let mut values = object.iter();
        if let Some((first_key, first_value)) = values.next() {
            self.write_space(self.padding_size)?;

            self.fmt_string(first_key)?;
            self.write_str(": ")?;
            self.fmt_element(first_value)?;
            self.write_char('\n')?;

            for (rest_key, rest_value) in values {
                self.write_char(',')?;
                self.write_space(self.padding_size)?;
                self.fmt_value(&Value::String(rest_key.to_string()))?;
                self.write_str(": ")?;
                self.fmt_element(rest_value)?;
                self.write_char('\n')?;
            }
        }
        self.write_char('}')?;
        Ok(())
    }

    pub fn fmt_element(&mut self, element: &Value) -> crate::Result {
        if value_is_multline(element) {
            self.write_char('\n')?;
            self.indent();
            self.fmt_value(element)?;
            self.dedent();
        } else {
            self.fmt_value(element)?;
        }
        Ok(())
    }

    pub fn fmt_value(&mut self, value: &Value) -> crate::Result {
        match value {
            Value::Null |
            Value::Bool(..) |
            Value::Number(..) |
            Value::String(..) => serde_fmt_value(self, value),
            Value::Array(vec) => self.fmt_array(vec),
            Value::Object(map) => self.fmt_object(map),
        }
    }
}

/////////////
// Wrapper //
/////////////

pub fn format(root: &Value, options: &Options) -> crate::Result<String> {
    let mut f = Context::new(options);
    f.fmt_value(root)?;
    Ok(f.finish())
}
