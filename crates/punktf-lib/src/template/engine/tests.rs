//! Tests for template engines.

#[cfg(test)]
mod tests {
    use crate::template::source::Source;
    use crate::profile::variables::Variables;
    use crate::template::engine::{TemplateEngine, TemplateContext, TemplateEngineType};

    #[cfg(feature = "template-punktf")]
    mod punktf_tests {
        use super::*;
        use crate::template::engine::punktf::PunktfTemplateEngine;

        #[test]
        fn test_punktf_engine_basic() {
            crate::tests::setup_test_env();
            
            let engine = PunktfTemplateEngine::new();
            let source = Source::anonymous("Hello {{NAME}}!");
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("NAME", "World")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "Hello World!");
        }

        #[test]
        fn test_punktf_engine_if_block() {
            crate::tests::setup_test_env();
            
            let engine = PunktfTemplateEngine::new();
            let source = Source::anonymous("{{@if {{OS}} == \"linux\"}}Linux{{@else}}Other{{@fi}}");
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("OS", "linux")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "Linux");
        }
    }

    #[cfg(feature = "template-minijinja")]
    mod minijinja_tests {
        use super::*;
        use crate::template::engine::minijinja::MiniJinjaTemplateEngine;

        #[test]
        fn test_minijinja_engine_basic() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let source = Source::anonymous("Hello {{ NAME }}!");
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("NAME", "World")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "Hello World!");
        }

        #[test]
        fn test_minijinja_engine_if_block() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let source = Source::anonymous("{% if OS == \"linux\" %}Linux{% else %}Other{% endif %}");
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("OS", "linux")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "Linux");
        }

        #[test]
        fn test_minijinja_engine_with_filters() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let source = Source::anonymous("{{ name | upper }}");
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("name", "world")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "WORLD");
        }

        #[test]
        fn test_minijinja_engine_complex_conditionals() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let template_content = r#"
{%- if PUNKTF_TARGET_OS == "linux" -%}
export PATH="$HOME/.local/bin:$PATH"
{%- elif PUNKTF_TARGET_OS == "macos" -%}
export PATH="/opt/homebrew/bin:$PATH"
{%- else -%}
# Unknown OS
{%- endif -%}
"#;
            let source = Source::anonymous(template_content);
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new();
            let result = engine.render(&template, &context).expect("Should render template");
            
            // Should use the actual target OS
            #[cfg(target_os = "linux")]
            assert_eq!(result, r#"export PATH="$HOME/.local/bin:$PATH""#);
            #[cfg(target_os = "macos")]
            assert_eq!(result, r#"export PATH="/opt/homebrew/bin:$PATH""#);
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            assert_eq!(result, "# Unknown OS");
        }

        #[test]
        fn test_minijinja_engine_dotfile_scenarios() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            
            // Test .gitconfig template
            let gitconfig_template = r#"
[user]
    name = {{ git_name }}
    email = {{ git_email }}

{%- if PUNKTF_TARGET_OS == "windows" %}
[core]
    autocrlf = true
{%- else %}
[core]
    autocrlf = input
{%- endif %}

[alias]
    st = status
    co = checkout
    br = branch
    la = log --oneline --graph --all
"#;
            
            let source = Source::anonymous(gitconfig_template);
            let template = engine.parse(source).expect("Should parse gitconfig template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([
                    ("git_name", "John Doe"),
                    ("git_email", "john@example.com"),
                ])));
            
            let result = engine.render(&template, &context).expect("Should render gitconfig");
            
            assert!(result.contains("name = John Doe"));
            assert!(result.contains("email = john@example.com"));
            #[cfg(target_os = "windows")]
            assert!(result.contains("autocrlf = true"));
            #[cfg(not(target_os = "windows"))]
            assert!(result.contains("autocrlf = input"));
        }

        #[test]
        fn test_minijinja_engine_with_replace_filter() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let source = Source::anonymous(r#"{{ path | replace("/", "\\") }}"#);
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([("path", "/usr/local/bin")])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, r#"\usr\local\bin"#);
        }

        #[test]
        fn test_minijinja_engine_variable_precedence() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let source = Source::anonymous("Profile: {{ profile_NAME }}, Dotfile: {{ dotfile_NAME }}, Current: {{ NAME }}");
            let template = engine.parse(source).expect("Should parse template");
            
            let profile_vars = Variables::from_items([("NAME", "from_profile")]);
            let dotfile_vars = Variables::from_items([("NAME", "from_dotfile")]);
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&profile_vars))
                .with_dotfile_vars(Some(&dotfile_vars));
            
            let result = engine.render(&template, &context).expect("Should render template");
            assert_eq!(result, "Profile: from_profile, Dotfile: from_dotfile, Current: from_dotfile");
        }

        #[test]
        fn test_minijinja_engine_multiline_template() {
            crate::tests::setup_test_env();
            
            let engine = MiniJinjaTemplateEngine::new();
            let template_content = r#"# Configuration for {{ app_name }}
# Generated by punktf

[settings]
debug = {{ debug | lower }}
port = {{ port }}

{% if enable_auth -%}
[auth]
enabled = true
{% endif -%}

{% if enable_logging -%}
[logging]
level = {{ log_level }}
{% endif -%}
"#;
            
            let source = Source::anonymous(template_content);
            let template = engine.parse(source).expect("Should parse template");
            
            let context = TemplateContext::new()
                .with_profile_vars(Some(&Variables::from_items([
                    ("app_name", "MyApp"),
                    ("debug", "True"),
                    ("port", "8080"),
                    ("enable_auth", "true"),
                    ("enable_logging", "true"),
                    ("log_level", "INFO"),
                ])));
            
            let result = engine.render(&template, &context).expect("Should render template");
            
            assert!(result.contains("# Configuration for MyApp"));
            assert!(result.contains("debug = true"));
            assert!(result.contains("port = 8080"));
            assert!(result.contains("[auth]"));
            assert!(result.contains("enabled = true"));
            assert!(result.contains("[logging]"));
            assert!(result.contains("level = INFO"));
        }
    }

    #[test]
    fn test_template_engine_type_parsing() {
        use std::str::FromStr;
        
        assert_eq!(TemplateEngineType::from_str("punktf").unwrap(), TemplateEngineType::Punktf);
        
        #[cfg(feature = "template-minijinja")]
        {
            assert_eq!(TemplateEngineType::from_str("minijinja").unwrap(), TemplateEngineType::Minijinja);
            assert_eq!(TemplateEngineType::from_str("jinja").unwrap(), TemplateEngineType::Minijinja);
            assert_eq!(TemplateEngineType::from_str("jinja2").unwrap(), TemplateEngineType::Minijinja);
        }
        
        assert!(TemplateEngineType::from_str("unknown").is_err());
    }

    #[test]
    fn test_template_context_variable_precedence() {
        let profile_vars = Variables::from_items([("NAME", "profile"), ("UNIQUE", "profile_only")]);
        let dotfile_vars = Variables::from_items([("NAME", "dotfile"), ("OVERRIDE", "dotfile_only")]);
        
        let context = TemplateContext::new()
            .with_profile_vars(Some(&profile_vars))
            .with_dotfile_vars(Some(&dotfile_vars));
        
        let flat_map = context.to_flat_map();
        
        // Dotfile variables should override profile variables for unprefixed keys
        assert_eq!(flat_map.get("NAME"), Some(&"dotfile".to_string()));
        
        // Profile-specific prefixed variables should exist
        assert_eq!(flat_map.get("profile_NAME"), Some(&"profile".to_string()));
        assert_eq!(flat_map.get("profile_UNIQUE"), Some(&"profile_only".to_string()));
        
        // Dotfile-specific prefixed variables should exist
        assert_eq!(flat_map.get("dotfile_NAME"), Some(&"dotfile".to_string()));
        assert_eq!(flat_map.get("dotfile_OVERRIDE"), Some(&"dotfile_only".to_string()));
    }
}