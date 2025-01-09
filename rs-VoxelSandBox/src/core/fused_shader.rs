use std::fmt;

use anyhow::bail;
use glium::Program;
use glium::backend::Facade;

//////////////////
// Shader stage //
//////////////////

#[derive(Debug, Clone, Copy)]
enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
}

impl ShaderStage {
    pub fn keyword(&self) -> &'static str {
        match self {
            ShaderStage::Vertex => "@vertex",
            ShaderStage::Fragment => "@fragment",
            ShaderStage::Geometry => "@geometry",
        }
    }
}

impl fmt::Display for ShaderStage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.keyword()[1..].fmt(f) // just slice off the '@'
    }
}

/// Returns the byte index of the start of the line containing a comment with a shader stage attribute.
/// In its simplest form:
///
/// `// @vertex`
fn find_stage_comment_start(
    source: &str,
    stage: ShaderStage,
) -> crate::Result<usize> {
    let keyword = stage.keyword();
    let Some(keyword_start) = source.find(keyword) else {
        bail!("failed to find {stage} shader");
    };
    let keyword_end = keyword_start + keyword.len();
    if source[keyword_end..].contains(keyword) {
        bail!("more than one {stage} shader found");
    }
    let Some(line_start) = source[0..keyword_start].rfind("//") else {
        bail!("failed to find comment start for {keyword} shader")
    };
    Ok(line_start)
}

////////////////////////////
// Bundled shader sources //
////////////////////////////

#[derive(Debug)]
pub struct BundledShaderSource<'a> {
    pub vertex: &'a str,
    pub fragment: &'a str,
    pub geometry: Option<&'a str>,
}

impl<'a> BundledShaderSource<'a> {
    pub fn into_program(self, gl: &impl Facade) -> crate::Result<Program> {
        Ok(Program::from_source(gl, self.vertex, self.fragment, self.geometry)?)
    }
}

///////////////////
// Split shaders //
///////////////////

/// Splits a joined .vert and .frag file.
/// WGPU fuses them and that is way nicer.
pub fn split_shader(src: &str) -> crate::Result<BundledShaderSource> {
    // Find starting line of each stage
    let vert_start = find_stage_comment_start(src, ShaderStage::Vertex)?;
    let frag_start = find_stage_comment_start(src, ShaderStage::Fragment)?;
    let geo_start = find_stage_comment_start(src, ShaderStage::Geometry);

    if !matches!(geo_start, Err(_)) {
        bail!("geometry shader splitting is not supported");
    }
    if !(vert_start <= frag_start) {
        bail!("vertex shader must precede fragment shader");
    }

    // Sort indices in ascending order
    let a = 0usize;
    let b = vert_start;
    let c = frag_start;
    let d = src.len();
    debug_assert!(a <= b && b <= c && c <= d, "indices not in order");

    // ↓ splits source on c <==> splits the substring starting at b on (c - b)
    let (bc, cd) = src[b..d].split_at(c - b);

    let vertex = bc;
    let fragment = cd;
    let geometry = None;
    Ok(BundledShaderSource { vertex, fragment, geometry })
}
