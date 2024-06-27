use std::{fs, path::Path};

use handlebars::Handlebars;
use pulldown_cmark::{html::push_html, CowStr, Event, Options, Parser, Tag};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_site()
}

fn markdown_to_html(
    markdown_input: &str,
    file_path: &Path,
    root_path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let parser = Parser::new_ext(markdown_input, Options::all());
    let mut events: Vec<Event> = Vec::new();

    let relative_path = file_path.strip_prefix(root_path)?;
    let depth = relative_path.ancestors().count() - 1; // Count levels to the root of 'content'
    let prefix = "../".repeat(depth.max(0)); // Generate relative path prefix back to the content root

    for event in parser {
        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                if dest_url.ends_with(".md") {
                    // let new_dest = dest_url.trim_end_matches(".md").to_owned() + ".html";
                    let new_dest = if dest_url.ends_with(".md") {
                        let new_url = format!(
                            "{}{}",
                            prefix,
                            dest_url.strip_suffix(".md").unwrap().to_owned() + ".html"
                        );
                        CowStr::Boxed(new_url.into_boxed_str())
                    } else {
                        // Cow::Owned(dest_url.to_string())
                        dest_url
                    };
                    events.push(Event::Start(Tag::Link {
                        link_type,
                        dest_url: new_dest,
                        title,
                        id,
                    }));
                } else {
                    events.push(Event::Start(Tag::Link {
                        link_type,
                        dest_url,
                        title,
                        id,
                    }));
                }
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
    // Register template
    handlebars.register_template_string("template", include_str!("../template.html"))?;

    // Create data structure for template
    let data = serde_json::json!({
        "title": title,
        "nav": nav,
        "content": content
    });

    // Render template
    handlebars.render("template", &data)
}

fn generate_site() -> Result<(), Box<dyn std::error::Error>> {
    let site_dir = Path::new("site");
    ensure_directory_exists(site_dir)?;
    let content_dir = Path::new("content");

    let nav_path = Path::new("content/nav.md");
    let nav_md = fs::read_to_string(nav_path)?;
    let nav_html = markdown_to_html(&nav_md, nav_path, content_dir)?;

    for entry in WalkDir::new("content")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str() != Some("nav.md"))
        .filter(|e| e.file_name().to_str() != Some("footer.md"))
    {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "md") {
            let md = fs::read_to_string(entry.path())?;
            let html = markdown_to_html(&md, entry.path(), content_dir)?;
            let title = entry.path().file_stem().unwrap().to_str().unwrap();
            let final_html = render_template(title, &nav_html, &html)?;

            let relative_path = entry.path().strip_prefix("content")?.with_extension("html");
            let output_path = site_dir.join(relative_path);

            if let Some(parent) = output_path.parent() {
                ensure_directory_exists(parent)?; // Ensure each parent directory exists
            }
            fs::write(output_path, final_html)?;
        }
    }

    Ok(())
}

fn ensure_directory_exists(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        fs::create_dir_all(path)?; // Creates the directory and all necessary parent directories
    }
    Ok(())
}
