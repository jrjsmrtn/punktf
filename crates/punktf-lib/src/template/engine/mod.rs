//! Template engine abstraction and implementations.
//!
//! This module provides a trait-based abstraction over different template engines,
//! allowing punktf to support multiple templating syntaxes and implementations.

use std::collections::HashMap;

use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

use crate::profile::variables::Vars;
use super::source::Source;

#[cfg(feature = "template-punktf")]
pub mod punktf;

#[cfg(feature = "template-minijinja")]
pub mod minijinja;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod manual_test;

/// Returns the target architecture string.
fn target_arch() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86")] {
            "x86"
        } else if #[cfg(target_arch = "x86_64")] {
            "x86_64"
        } else if #[cfg(target_arch = "mips")] {
            "mips"
        } else if #[cfg(target_arch = "powerpc")] {
            "powerpc"
        } else if #[cfg(target_arch = "powerpc64")] {
            "powerpc64"
        } else if #[cfg(target_arch = "arm")] {
            "arm"
        } else if #[cfg(target_arch = "aarch64")] {
            "aarch64"
        } else {
            "unknown"
        }
    }
}

/// Returns the target operating system string.
fn target_os() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            "windows"
        } else if #[cfg(target_os = "macos")] {
            "macos"
        } else if #[cfg(target_os = "ios")] {
            "ios"
        } else if #[cfg(target_os = "linux")] {
            "linux"
        } else if #[cfg(target_os = "android")] {
            "android"
        } else if #[cfg(target_os = "freebsd")] {
            "freebsd"
        } else if #[cfg(target_os = "dragonfly")] {
            "dragonfly"
        } else if #[cfg(target_os = "openbsd")] {
            "openbsd"
        } else if #[cfg(target_os = "netbsd")] {
            "netbsd"
        } else {
            "unknown"
        }
    }
}

/// Returns the target family string.
fn target_family() -> &'static str {
    cfg_if::cfg_if! {
        if #[cfg(target_family = "unix")] {
            "unix"
        } else if #[cfg(target_os = "windows")] {
            "windows"
        } else if #[cfg(target_family = "wasm")] {
            "wasm"
        } else {
            "unknown"
        }
    }
}

/// Template engine configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateEngineType {
    /// Use the original punktf template engine (handlebars-like syntax).
    Punktf,
    /// Use the MiniJinja template engine (Jinja2-like syntax).
    #[cfg(feature = "template-minijinja")]
    Minijinja,
}

impl Default for TemplateEngineType {
    fn default() -> Self {
        Self::Punktf
    }
}

impl std::fmt::Display for TemplateEngineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Punktf => write!(f, "punktf"),
            #[cfg(feature = "template-minijinja")]
            Self::Minijinja => write!(f, "minijinja"),
        }
    }
}

impl std::str::FromStr for TemplateEngineType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "punktf" => Ok(Self::Punktf),
            #[cfg(feature = "template-minijinja")]
            "minijinja" | "jinja" | "jinja2" => Ok(Self::Minijinja),
            _ => Err(format!("Unknown template engine: {}", s)),
        }
    }
}

/// An enum that can hold any supported template engine type.
#[derive(Debug, Clone, Copy)]
pub enum AnyTemplateEngine {
    #[cfg(feature = "template-punktf")]
    /// The punktf template engine (original handlebars-inspired syntax).
    Punktf(punktf::PunktfTemplateEngine),
    #[cfg(feature = "template-minijinja")]
    /// The MiniJinja template engine (Jinja2-compatible syntax).
    Minijinja(minijinja::MiniJinjaTemplateEngine),
}

impl AnyTemplateEngine {
    /// Create a template engine of the specified type.
    pub fn new(engine_type: TemplateEngineType) -> Self {
        match engine_type {
            #[cfg(feature = "template-punktf")]
            TemplateEngineType::Punktf => Self::Punktf(punktf::PunktfTemplateEngine::new()),
            #[cfg(feature = "template-minijinja")]
            TemplateEngineType::Minijinja => Self::Minijinja(minijinja::MiniJinjaTemplateEngine::new()),
        }
    }
}

/// A template engine that can parse and render templates.
pub trait TemplateEngine {
    /// The compiled template type for this engine.
    type Template;
    
    /// Engine-specific error type.
    type Error: Into<color_eyre::Report>;
    
    /// Parse a template from source content.
    ///
    /// # Arguments
    /// * `source` - The template source with content and metadata
    ///
    /// # Errors
    /// Returns an error if the template syntax is invalid.
    fn parse(&self, source: Source<'_>) -> Result<Self::Template, Self::Error>;
    
    /// Render a compiled template with the given context.
    ///
    /// # Arguments
    /// * `template` - The compiled template to render
    /// * `context` - Variable context for template rendering
    ///
    /// # Errors
    /// Returns an error if template rendering fails.
    fn render(&self, template: &Self::Template, context: &TemplateContext) -> Result<String, Self::Error>;
}

/// Context for template rendering containing variables from different sources.
#[derive(Debug, Clone)]
pub struct TemplateContext {
    /// Variables from the profile configuration.
    pub profile_vars: Option<HashMap<String, String>>,
    
    /// Variables from the dotfile configuration.
    pub dotfile_vars: Option<HashMap<String, String>>,
    
    /// Whether to include system environment variables.
    pub include_env: bool,
}

impl TemplateContext {
    /// Create a new template context.
    pub fn new() -> Self {
        Self {
            profile_vars: None,
            dotfile_vars: None,
            include_env: true,
        }
    }
    
    /// Set profile variables.
    pub fn with_profile_vars<V: Vars>(mut self, vars: Option<&V>) -> Self {
        self.profile_vars = vars.map(|v| v.as_map());
        self
    }
    
    /// Set dotfile variables.
    pub fn with_dotfile_vars<V: Vars>(mut self, vars: Option<&V>) -> Self {
        self.dotfile_vars = vars.map(|v| v.as_map());
        self
    }
    
    /// Set whether to include environment variables.
    pub fn with_env(mut self, include_env: bool) -> Self {
        self.include_env = include_env;
        self
    }
    
    /// Convert to a flat map suitable for template engines.
    /// Variables are prefixed to avoid conflicts: env_, profile_, dotfile_.
    pub fn to_flat_map(&self) -> HashMap<String, String> {
        let mut context = HashMap::new();
        
        // Add environment variables with env_ prefix
        if self.include_env {
            for (key, value) in std::env::vars() {
                context.insert(format!("env_{}", key), value);
            }
            
            // Add special punktf environment variables without prefix for compatibility
            if let Ok(val) = std::env::var("PUNKTF_CURRENT_SOURCE") {
                context.insert("PUNKTF_CURRENT_SOURCE".to_string(), val);
            }
            if let Ok(val) = std::env::var("PUNKTF_CURRENT_TARGET") {
                context.insert("PUNKTF_CURRENT_TARGET".to_string(), val);
            }
            if let Ok(val) = std::env::var("PUNKTF_CURRENT_PROFILE") {
                context.insert("PUNKTF_CURRENT_PROFILE".to_string(), val);
            }
        }
        
        // Add profile variables with profile_ prefix
        if let Some(ref vars) = self.profile_vars {
            for (key, value) in vars {
                context.insert(format!("profile_{}", key), value.clone());
                // Also add without prefix for compatibility
                context.insert(key.clone(), value.clone());
            }
        }
        
        // Add dotfile variables with dotfile_ prefix (these take precedence)
        if let Some(ref vars) = self.dotfile_vars {
            for (key, value) in vars {
                context.insert(format!("dotfile_{}", key), value.clone());
                // Also add without prefix for compatibility (overrides profile vars)
                context.insert(key.clone(), value.clone());
            }
        }
        
        // Add target architecture/OS information
        context.insert("PUNKTF_TARGET_ARCH".to_string(), target_arch().to_string());
        context.insert("PUNKTF_TARGET_OS".to_string(), target_os().to_string());
        context.insert("PUNKTF_TARGET_FAMILY".to_string(), target_family().to_string());
        
        context
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert context to a serializable format for template engines that need it.
#[derive(Serialize, Debug)]
pub struct SerializableContext(HashMap<String, String>);

impl From<&TemplateContext> for SerializableContext {
    fn from(context: &TemplateContext) -> Self {
        Self(context.to_flat_map())
    }
}

impl SerializableContext {
    /// Get the underlying map.
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.0
    }
}

/// A generic template that can be rendered with any template engine.
#[derive(Debug)]
pub struct GenericTemplate<E: TemplateEngine> {
    /// The template engine used for this template.
    engine: E,
    
    /// The compiled template.
    template: E::Template,
}

impl<E: TemplateEngine> GenericTemplate<E> {
    /// Create a new generic template.
    pub fn new(engine: E, template: E::Template) -> Self {
        Self { engine, template }
    }
    
    /// Parse a template using the given engine.
    pub fn parse(engine: E, source: Source<'_>) -> Result<Self, E::Error> {
        let template = engine.parse(source)?;
        Ok(Self::new(engine, template))
    }
    
    /// Render the template with the given context.
    pub fn render(&self, context: &TemplateContext) -> Result<String, E::Error> {
        self.engine.render(&self.template, context)
    }
    
    /// Render the template with variables from profile and dotfile.
    pub fn render_with_vars<PV: Vars, DV: Vars>(
        &self,
        profile_vars: Option<&PV>,
        dotfile_vars: Option<&DV>,
    ) -> Result<String, E::Error> {
        let context = TemplateContext::new()
            .with_profile_vars(profile_vars)
            .with_dotfile_vars(dotfile_vars)
            .with_env(true);
        
        self.render(&context)
    }
}