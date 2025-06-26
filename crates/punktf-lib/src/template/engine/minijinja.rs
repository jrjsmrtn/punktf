//! MiniJinja template engine implementation.

use color_eyre::eyre::Result;
use thiserror::Error;

use super::{TemplateEngine, TemplateContext, SerializableContext};
use crate::template::source::Source;

/// The MiniJinja template engine using Jinja2-compatible syntax.
#[derive(Debug, Clone, Copy, Default)]
pub struct MiniJinjaTemplateEngine;

/// A compiled MiniJinja template that owns its content.
#[derive(Debug)]
pub struct CompiledTemplate {
    /// The template source content.
    content: String,
    /// Template name for debugging.
    name: String,
}

/// Errors that can occur when using the MiniJinja template engine.
#[derive(Debug, Error)]
pub enum MiniJinjaTemplateError {
    /// Template parsing failed.
    #[error("MiniJinja template parsing failed: {0}")]
    ParseError(#[from] minijinja::Error),
    
    /// Template rendering failed.
    #[error("MiniJinja template rendering failed: {0}")]
    RenderError(minijinja::Error),
}

impl TemplateEngine for MiniJinjaTemplateEngine {
    type Template = CompiledTemplate;
    type Error = MiniJinjaTemplateError;
    
    fn parse(&self, source: Source<'_>) -> Result<Self::Template, Self::Error> {
        let template_name = match source.origin() {
            crate::template::source::SourceOrigin::File(path) => {
                path.to_string_lossy().to_string()
            }
            crate::template::source::SourceOrigin::Anonymous => "anonymous".to_string(),
        };
        
        // Create a temporary environment to validate the template syntax
        let mut env = minijinja::Environment::new();
        
        // Add useful filters for dotfile management
        env.add_filter("upper", str::to_uppercase);
        env.add_filter("lower", str::to_lowercase);
        env.add_filter("replace", |s: &str, from: &str, to: &str| s.replace(from, to));
        
        // Validate the template by attempting to add it
        env.add_template(&template_name, source.content())?;
        
        // If validation succeeds, store the content for later rendering
        Ok(CompiledTemplate {
            content: source.content().to_string(),
            name: template_name,
        })
    }
    
    fn render(&self, template: &Self::Template, context: &TemplateContext) -> Result<String, Self::Error> {
        // Create a fresh environment for rendering
        let mut env = minijinja::Environment::new();
        
        // Add useful filters for dotfile management
        env.add_filter("upper", str::to_uppercase);
        env.add_filter("lower", str::to_lowercase);
        env.add_filter("replace", |s: &str, from: &str, to: &str| s.replace(from, to));
        
        // Add the template content
        env.add_template(&template.name, &template.content)
            .map_err(MiniJinjaTemplateError::ParseError)?;
        
        // Get the template and render it
        let tmpl = env.get_template(&template.name)
            .map_err(MiniJinjaTemplateError::ParseError)?;
        
        let serializable_context = SerializableContext::from(context);
        
        tmpl.render(serializable_context.as_map())
            .map_err(MiniJinjaTemplateError::RenderError)
    }
}

impl MiniJinjaTemplateEngine {
    /// Create a new MiniJinja template engine.
    pub fn new() -> Self {
        Self
    }
}