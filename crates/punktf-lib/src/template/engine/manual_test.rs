// Manual test for MiniJinja template engine
// Run with: cargo test --features template-minijinja manual_test -- --nocapture

#[cfg(test)]
mod manual_tests {
    use crate::template::source::Source;
    use crate::template::engine::{TemplateEngine, TemplateContext, TemplateEngineType};
    use std::collections::HashMap;
    
    #[test]
    fn test_template_engine_type_parsing() {
        println!("\n🧪 Testing Template Engine Type Parsing");
        
        // Test parsing different engine types
        let test_cases = vec![
            ("punktf", TemplateEngineType::Punktf),
            #[cfg(feature = "template-minijinja")]
            ("minijinja", TemplateEngineType::Minijinja),
            #[cfg(feature = "template-minijinja")]
            ("jinja", TemplateEngineType::Minijinja),
            #[cfg(feature = "template-minijinja")]
            ("jinja2", TemplateEngineType::Minijinja),
        ];
        
        for (input, expected) in test_cases {
            match input.parse::<TemplateEngineType>() {
                Ok(engine_type) => {
                    assert_eq!(engine_type, expected);
                    println!("  ✓ '{}' -> {:?}", input, engine_type);
                }
                Err(e) => {
                    panic!("Failed to parse '{}': {}", input, e);
                }
            }
        }
    }
    
    #[test] 
    #[cfg(feature = "template-minijinja")]
    fn test_minijinja_engine_creation() {
        println!("\n🧪 Testing MiniJinja Engine Creation");
        
        use crate::template::engine::minijinja::MiniJinjaTemplateEngine;
        
        let engine = MiniJinjaTemplateEngine::new();
        println!("  ✓ MiniJinja engine created successfully");
        
        // Test basic template compilation
        let source = Source::anonymous("Hello {{ name }}!");
        match engine.parse(source) {
            Ok(_template) => {
                println!("  ✓ Template parsing works");
            }
            Err(e) => {
                panic!("Template parsing failed: {:?}", e);
            }
        }
    }
    
    #[test]
    #[cfg(feature = "template-minijinja")]
    fn test_minijinja_basic_rendering() {
        println!("\n🧪 Testing MiniJinja Basic Rendering");
        
        use crate::template::engine::minijinja::MiniJinjaTemplateEngine;
        use crate::profile::variables::Variables;
        
        let engine = MiniJinjaTemplateEngine::new();
        
        // Test simple variable substitution
        let source = Source::anonymous("Hello {{ name }}!");
        let template = engine.parse(source).expect("Should parse template");
        
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "World".to_string());
        let variables = Variables { inner: vars };
        
        let context = TemplateContext::new()
            .with_profile_vars(Some(&variables))
            .with_env(false);
        
        let result = engine.render(&template, &context).expect("Should render template");
        assert_eq!(result, "Hello World!");
        println!("  ✓ Basic variable substitution: '{}'", result);
    }
    
    #[test]
    #[cfg(feature = "template-minijinja")]
    fn test_minijinja_conditionals() {
        println!("\n🧪 Testing MiniJinja Conditionals");
        
        use crate::template::engine::minijinja::MiniJinjaTemplateEngine;
        use crate::profile::variables::Variables;
        
        let engine = MiniJinjaTemplateEngine::new();
        
        // Test conditional rendering
        let source = Source::anonymous("{% if OS == \"linux\" %}Linux{% else %}Other{% endif %}");
        let template = engine.parse(source).expect("Should parse template");
        
        let mut vars = HashMap::new();
        vars.insert("OS".to_string(), "linux".to_string());
        let variables = Variables { inner: vars };
        
        let context = TemplateContext::new()
            .with_profile_vars(Some(&variables))
            .with_env(false);
        
        let result = engine.render(&template, &context).expect("Should render template");
        assert_eq!(result, "Linux");
        println!("  ✓ Conditional rendering: '{}'", result);
    }
    
    #[test]
    #[cfg(feature = "template-minijinja")]
    fn test_minijinja_filters() {
        println!("\n🧪 Testing MiniJinja Filters");
        
        use crate::template::engine::minijinja::MiniJinjaTemplateEngine;
        use crate::profile::variables::Variables;
        
        let engine = MiniJinjaTemplateEngine::new();
        
        // Test built-in filters
        let test_cases = vec![
            ("{{ name | upper }}", "world", "WORLD"),
            ("{{ name | lower }}", "WORLD", "world"),
            ("{{ text | replace(\"old\", \"new\") }}", "old_text", "new_text"),
        ];
        
        for (template_str, var_value, expected) in test_cases {
            let source = Source::anonymous(template_str);
            let template = engine.parse(source).expect("Should parse template");
            
            let mut vars = HashMap::new();
            if template_str.contains("name") {
                vars.insert("name".to_string(), var_value.to_string());
            } else {
                vars.insert("text".to_string(), var_value.to_string());
            }
            let variables = Variables { inner: vars };
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&variables))
                .with_env(false);
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, expected);
            println!("  ✓ Filter test '{}' -> '{}'", template_str, result);
        }
    }
    
    #[test]
    fn test_template_context_variable_precedence() {
        println!("\n🧪 Testing Template Context Variable Precedence");
        
        use crate::profile::variables::Variables;
        
        // Create test variables with overlapping names
        let mut profile_vars = HashMap::new();
        profile_vars.insert("NAME".to_string(), "profile_value".to_string());
        profile_vars.insert("UNIQUE_PROFILE".to_string(), "profile_only".to_string());
        
        let mut dotfile_vars = HashMap::new();
        dotfile_vars.insert("NAME".to_string(), "dotfile_value".to_string());
        dotfile_vars.insert("UNIQUE_DOTFILE".to_string(), "dotfile_only".to_string());
        
        let profile_variables = Variables { inner: profile_vars };
        let dotfile_variables = Variables { inner: dotfile_vars };
        
        // Create context with variable precedence
        let context = TemplateContext::new()
            .with_profile_vars(Some(&profile_variables))
            .with_dotfile_vars(Some(&dotfile_variables))
            .with_env(false);
        
        let flat_map = context.to_flat_map();
        
        // Test variable precedence (dotfile should override profile)
        assert_eq!(flat_map.get("NAME"), Some(&"dotfile_value".to_string()));
        println!("  ✓ Variable precedence works (dotfile > profile)");
        
        // Test prefixed variables exist
        assert_eq!(flat_map.get("profile_NAME"), Some(&"profile_value".to_string()));
        assert_eq!(flat_map.get("dotfile_NAME"), Some(&"dotfile_value".to_string()));
        println!("  ✓ Prefixed variables available");
        
        // Test unique variables
        assert_eq!(flat_map.get("UNIQUE_PROFILE"), Some(&"profile_only".to_string()));
        assert_eq!(flat_map.get("UNIQUE_DOTFILE"), Some(&"dotfile_only".to_string()));
        println!("  ✓ Unique variables from both sources available");
        
        println!("  📊 Context contains {} variables", flat_map.len());
    }
}