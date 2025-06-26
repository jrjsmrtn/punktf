# Examples

## Structure

Examples are organized by template engine to demonstrate the syntax differences:

- `punktf/` - Examples using the original punktf template engine (handlebars-like syntax)
- `minijinja/` - Examples using the MiniJinja template engine (Jinja2 syntax)

For each example, a separate folder should be created in the respective engine directory.
To automatically test the deployment of the profile, create a script named `run.sh` in the
newly created example folder (e.g. `examples/punktf/01_test/run.sh`).

The example folder should start with a number which determines the execution order and also the level of `advancedness` of the example (e.g. `10_extending_profiles`).

## Template Engine Differences

### punktf Template Engine (Original)
- Variables: `{{OS}}`, `{{#PROFILE_VAR}}`, `{{&DOTFILE_VAR}}`
- Conditionals: `{{@if {{OS}} == "linux"}}...{{@fi}}`
- Comments: `{{!-- comment --}}`

### MiniJinja Template Engine (Jinja2 Syntax)
- Variables: `{{ OS }}`, `{{ profile_VAR }}`, `{{ dotfile_VAR }}`
- Conditionals: `{% if OS == "linux" %}...{% endif %}`
- Comments: `{# comment #}`
- Filters: `{{ name | upper }}`, `{{ text | replace("old", "new") }}`

## Examples

- `01_single_file`: Deploys a single file dotfile
- `02_single_dir`: Deploys a single directory dotfile
- `03_single_template`: Deploys a single template dotfile
- `10_extending_profiles`: Deploys a profile which extends a base profile
- `80_simple_complete`: Deploys a really simple "real world" example
- `85_multi_os`: Holds profiles and dotfiles for a windows and linux host system with shared dotfiles.
