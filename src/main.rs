use std::{fs, path::Path};

use handlebars::Handlebars;
use pulldown_cmark::{html::push_html, CowStr, Event, Options, Parser, Tag};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_site()
}

fn markdown_to_html(markdown_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parser = Parser::new_ext(markdown_input, Options::all());
    let mut events: Vec<Event> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                let new_dest = if dest_url.ends_with(".md") {
                    dest_url.replace(".md", ".html")
                } else {
                    dest_url.to_string()
                };

                // Push the modified or original link event
                events.push(Event::Start(Tag::Link {
                    link_type,
                    dest_url: CowStr::Boxed(new_dest.into_boxed_str()),
                    title,
                    id,
                }));
            }
            _ => events.push(event),
        }
    }

    let mut html_output = String::new();
    push_html(&mut html_output, events.into_iter());
    Ok(html_output)
}

fn render_template(
    title: &str,
    nav: &str,
    content: &str,
) -> Result<String, handlebars::RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("template", include_str!("../template.html"))?;

    let data = serde_json::json!({
        "title": title,
        "nav": nav,
        "content": content
    });

    handlebars.render("template", &data)
}

fn generate_site() -> Result<(), Box<dyn std::error::Error>> {
    let site_dir = Path::new("_site");
    ensure_directory_exists(site_dir)?;

    let nav_path = Path::new("content/nav.md");
    let nav_md = fs::read_to_string(nav_path)?;
    let nav_html = markdown_to_html(&nav_md)?;

    for entry in WalkDir::new("content")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str() != Some("nav.md"))
    {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "md") {
            let md = fs::read_to_string(entry.path())?;
            let html = markdown_to_html(&md)?;
            let title = entry.path().file_stem().unwrap().to_str().unwrap();

            let relative_path = entry.path().strip_prefix("content")?.with_extension("html");
            let output_path = site_dir.join(&relative_path);

            let parent_dir_depth = relative_path.ancestors().count() - 2;
            let relative_nav_path = "../".repeat(parent_dir_depth);
            let adjusted_nav_html = adjust_nav_paths(&nav_html, &relative_nav_path);

            let final_html = render_template(title, &adjusted_nav_html, &html)?;

            if let Some(parent) = output_path.parent() {
                ensure_directory_exists(parent)?; // Ensure each parent directory exists
            }
            fs::write(output_path, final_html)?;
        }
    }

    Ok(())
}

fn adjust_nav_paths(nav_html: &str, relative_path: &str) -> String {
    let mut adjusted_html = nav_html.to_string();

    // Regex to find all markdown links and adjust paths
    let re = regex::Regex::new(r#"href="\./([^"]+)"#).unwrap();
    adjusted_html = re
        .replace_all(
            &adjusted_html,
            format!(r#"href="{}$1""#, relative_path).as_str(),
        )
        .to_string();

    adjusted_html
}

fn ensure_directory_exists(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        fs::create_dir_all(path)?; // Creates the directory and all necessary parent directories
    }
    Ok(())
}
