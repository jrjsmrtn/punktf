//! Original punktf template engine implementation.

use color_eyre::eyre::Result;
use thiserror::Error;

use super::{TemplateEngine, TemplateContext};
use crate::template::{source::Source, Template as PunktfTemplate};
use crate::profile::variables::Variables;

/// The original punktf template engine using handlebars-inspired syntax.
#[derive(Debug, Clone, Copy, Default)]
pub struct PunktfTemplateEngine;

/// Errors that can occur when using the punktf template engine.
#[derive(Debug, Error)]
pub enum PunktfTemplateError {
    /// Template parsing failed.
    #[error("Template parsing failed: {0}")]
    ParseError(color_eyre::Report),
    
    /// Template rendering failed.
    #[error("Template rendering failed: {0}")]
    RenderError(color_eyre::Report),
}

impl TemplateEngine for PunktfTemplateEngine {
    type Template = PunktfTemplate<'static>;
    type Error = PunktfTemplateError;
    
    fn parse(&self, source: Source<'_>) -> Result<Self::Template, Self::Error> {
        // Convert to owned source for storage
        let owned_source = source.to_owned();
        PunktfTemplate::parse(owned_source).map_err(PunktfTemplateError::ParseError)
    }
    
    fn render(&self, template: &Self::Template, context: &TemplateContext) -> Result<String, Self::Error> {
        // Convert context to Variables
        let profile_vars = context.profile_vars.as_ref().map(|vars| {
            Variables { inner: vars.clone() }
        });
        
        let dotfile_vars = context.dotfile_vars.as_ref().map(|vars| {
            Variables { inner: vars.clone() }
        });
        
        template.resolve(profile_vars.as_ref(), dotfile_vars.as_ref())
            .map_err(PunktfTemplateError::RenderError)
    }
}

impl PunktfTemplateEngine {
    /// Create a new punktf template engine.
    pub fn new() -> Self {
        Self
    }
}