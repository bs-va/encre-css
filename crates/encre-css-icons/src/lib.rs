use directories::BaseDirs;
use encre_css::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
    Config,
};
use once_cell::sync::Lazy;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    env,
    fmt::{self, Write},
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    sync::Mutex,
};

pub mod collection;

use collection::{Collection, IconOptional};

const COLLECTIONS: &[&str] = &[
    "material-symbols",
    "ic",
    "mdi",
    "ph",
    "ri",
    "carbon",
    "bi",
    "tabler",
    "ion",
    "uil",
    "teenyicons",
    "clarity",
    "iconoir",
    "majesticons",
    "zondicons",
    "ant-design",
    "bx",
    "bxs",
    "gg",
    "cil",
    "lucide",
    "pixelarticons",
    "system-uicons",
    "ci",
    "akar-icons",
    "typcn",
    "radix-icons",
    "ep",
    "mdi-light",
    "fe",
    "eos-icons",
    "line-md",
    "charm",
    "prime",
    "heroicons-outline",
    "heroicons-solid",
    "uiw",
    "uim",
    "uit",
    "uis",
    "maki",
    "gridicons",
    "mi",
    "quill",
    "gala",
    "fluent",
    "icon-park-outline",
    "icon-park",
    "vscode-icons",
    "jam",
    "codicon",
    "pepicons",
    "bytesize",
    "ei",
    "fa6-solid",
    "fa6-regular",
    "octicon",
    "ooui",
    "nimbus",
    "openmoji",
    "twemoji",
    "noto",
    "noto-v1",
    "emojione",
    "emojione-monotone",
    "emojione-v1",
    "fxemoji",
    "bxl",
    "logos",
    "simple-icons",
    "cib",
    "fa6-brands",
    "arcticons",
    "file-icons",
    "brandico",
    "entypo-social",
    "cryptocurrency",
    "flag",
    "circle-flags",
    "flagpack",
    "cif",
    "gis",
    "map",
    "geo",
    "fad",
    "academicons",
    "wi",
    "healthicons",
    "medical-icon",
    "la",
    "eva",
    "dashicons",
    "flat-color-icons",
    "entypo",
    "foundation",
    "raphael",
    "icons8",
    "iwwa",
    "fa-solid",
    "fa-regular",
    "fa-brands",
    "fa",
    "fontisto",
    "icomoon-free",
    "ps",
    "subway",
    "oi",
    "wpf",
    "simple-line-icons",
    "et",
    "el",
    "vaadin",
    "grommet-icons",
    "whh",
    "si-glyph",
    "zmdi",
    "ls",
    "bpmn",
    "flat-ui",
    "vs",
    "topcoat",
    "il",
    "websymbol",
    "fontelico",
    "feather",
    "mono-icons",
];
const DEFAULT_CDN: &str = "https://esm.sh";

const CACHE_SUB_DIR_NAME: &str = "encre-css-icons-cache";

static PREFIX: Lazy<Mutex<&'static str>> = Lazy::new(|| Mutex::new(""));
static CUSTOM_CDN: Lazy<Mutex<&'static str>> = Lazy::new(|| Mutex::new(DEFAULT_CDN));
static SCALE: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(1.));
static MEM_CACHE: Lazy<Mutex<BTreeMap<&'static str, Collection>>> =
    Lazy::new(|| Mutex::new(BTreeMap::new()));

fn get_icon(collection: &Collection, icon_name: &str) -> Option<((String, String), String)> {
    let icon = if let Some(icon) = collection.icons.get(icon_name) {
        Cow::Borrowed(icon)
    } else if let Some(alias) = collection.aliases.get(icon_name) {
        if let Some(icon) = collection.icons.get(&alias.parent) {
            // Merge optional properties following the logic described in
            // https://docs.iconify.design/types/iconify-json.html
            let mut icon = icon.clone();
            icon.optional.top = alias.optional.top;
            icon.optional.left = alias.optional.left;
            icon.optional.width = alias.optional.width;
            icon.optional.height = alias.optional.height;
            icon.optional.rotate = (icon.optional.rotate + alias.optional.rotate) % 4.;
            icon.optional.h_flip = alias.optional.h_flip != icon.optional.h_flip;
            icon.optional.v_flip = alias.optional.v_flip != icon.optional.v_flip;
            Cow::Owned(icon)
        } else {
            return None;
        }
    } else if let Some(character) = collection.chars.get(icon_name) {
        if let Some(icon) = collection.icons.get(character) {
            Cow::Borrowed(icon)
        } else {
            return None;
        }
    } else {
        return None;
    };

    let IconOptional {
        mut left,
        mut top,
        h_flip,
        v_flip,
        ..
    } = icon.optional;
    let mut width = icon.optional.width.unwrap_or(collection.width);
    let mut height = icon.optional.height.unwrap_or(collection.height);
    let mut rotation = icon.optional.rotate;

    // The icon is flipped first, then rotated
    let after_transforms = if h_flip {
        if v_flip {
            rotation += 2.;
            String::new()
        } else {
            // Horizontal flip
            left = 0.;
            top = 0.;
            format!(
                "translate({} {}) scale(-1 1)",
                width + left,
                -(top as isize)
            )
        }
    } else if v_flip {
        left = 0.;
        top = 0.;
        format!(
            "translate({} {}) scale(1 -1)",
            -(left as isize),
            height + top
        )
    } else {
        String::new()
    };

    if rotation < 0. {
        rotation -= f32::floor(rotation as f32 / 4.) * 4.;
    }

    let rotation = (rotation % 4.) as usize;

    let before_transforms = match rotation {
        // 90 deg
        1 => format!("rotate(90 {val} {val})", val = height / 2. + top),

        // 180 deg
        2 => format!("rotate(180 {} {})", width / 2. + left, height / 2. + top),

        // 270 deg
        3 => format!("rotate(270 {val} {val})", val = width / 2. + left),

        _ => String::new(),
    };

    if rotation % 2 == 1 {
        // Swap width/height and top/left for 90deg or 270deg rotation
        if left != 0. || top != 0. {
            (left, top) = (top, left);
        }

        if width != height {
            (width, height) = (height, width);
        }
    }

    let formatted_width = format!("{}em", (width / height) * *SCALE.lock().unwrap());
    let formatted_height = format!("{}em", SCALE.lock().unwrap());

    let svg = {
        let body = if !before_transforms.is_empty() || !after_transforms.is_empty() {
            Cow::Owned(format!(
                r#"<g transform="{}{}{}">{}</g>"#,
                before_transforms,
                if before_transforms.is_empty() {
                    ""
                } else {
                    " "
                },
                after_transforms,
                icon.body
            ))
        } else {
            Cow::Borrowed(&icon.body)
        };

        // Optimize the SVG (from https://bl.ocks.org/jennyknuth/222825e315d45a738ed9d6e04c7a88d0)
        // And create a Data URI containing its data
        format!(r#"data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="{formatted_width}" height="{formatted_height}" preserveAspectRatio="XMidYMid meet" viewBox="{left} {top} {width} {height}">{body}</svg>"#)
            .replace('"', "\'")
            .replace('%', "%25")
            .replace('#', "%23")
            .replace('{', "%7B")
            .replace('}', "%7D")
            .replace('<', "%3C")
            .replace('>', "%3E")
    };

    Some(((formatted_width, formatted_height), svg))
}

fn fetch_or_cache_collection(collection: &'static str) {
    if MEM_CACHE.lock().unwrap().get(collection).is_some() {
        // Collection already in the memory cache, use it
        return;
    }

    let cache_dir = match BaseDirs::new() {
        Some(dirs) => PathBuf::from(dirs.cache_dir()),
        None => {
            // If the home directory is not set, use the current directory
            env::current_dir().expect("failed to get the current directory")
        }
    };

    let collection_file = cache_dir
        .join(CACHE_SUB_DIR_NAME)
        .join(collection)
        .with_extension("json");

    if collection_file.exists() {
        // File already in cache, use it
        let file = File::open(&collection_file).expect("failed to open the collection file");
        let reader = BufReader::new(file);
        MEM_CACHE.lock().unwrap().insert(
            collection,
            serde_json::from_reader(reader)
                .expect("failed to deserialize the response body as JSON"),
        );
    } else {
        // Fetch the file
        let url = format!(
            "{}/@iconify-json/{}/icons.json",
            CUSTOM_CDN.lock().unwrap(),
            collection
        );

        let content = reqwest::blocking::get(&url)
            .unwrap_or_else(|_| panic!("failed to get the response from `{url}`"))
            .text()
            .expect("failed to deserialize the response body as JSON");

        // Write the cached file to the disk
        fs::create_dir_all(collection_file.with_file_name(""))
            .expect("failed to create cache directory");
        fs::write(collection_file, &content).expect("failed to write the collection file");

        MEM_CACHE.lock().unwrap().insert(
            collection,
            serde_json::from_str(&content)
                .expect("failed to deserialize the response body as JSON"),
        );
    }
}

#[derive(Debug)]
struct Icons;

impl Plugin for Icons {
    fn namespace(&self) -> &'static str {
        *PREFIX.lock().unwrap()
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => COLLECTIONS.iter().any(|c| value.starts_with(c)),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let (collection, rest) = COLLECTIONS
                    .iter()
                    .find_map(|c| value.strip_prefix(c).map(|r| (c, r)))
                    .unwrap();
                fetch_or_cache_collection(collection);

                if let Some(((width, height), icon_data_uri)) = get_icon(
                    MEM_CACHE.lock().unwrap().get(collection).unwrap(),
                    rest.strip_prefix('-').unwrap_or(rest),
                ) {
                    if icon_data_uri.contains("currentColor") {
                        // From https://codepen.io/noahblon/post/coloring-svgs-in-css-background-images
                        writeln!(context.buffer, r#"--en-icon: url("{icon_data_uri}");"#)?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "mask: var(--en-icon) no-repeat;")?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "mask-size: 100% 100%;")?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "-webkit-mask: var(--en-icon) no-repeat;")?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "-webkit-mask-size: 100% 100%;")?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "background-color: currentColor;")?;
                    } else {
                        writeln!(
                            context.buffer,
                            r#"background: url("{icon_data_uri}") no-repeat center;"#
                        )?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "background-color: transparent;")?;
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "background-size: 100% 100%;")?;
                    }

                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "width: {width};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "height: {height};")?;
                }
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn register(
    config: &mut Config,
    prefix: Option<&'static str>,
    custom_cdn: Option<&'static str>,
    scale: Option<f32>,
) {
    // Reset variables
    *PREFIX.lock().unwrap() = prefix.unwrap_or("").trim_end_matches('-');
    *CUSTOM_CDN.lock().unwrap() = custom_cdn.unwrap_or(DEFAULT_CDN).trim_end_matches('/');
    *SCALE.lock().unwrap() = scale.unwrap_or(1.);

    config.register_plugin(&Icons);
}

#[cfg(test)]
mod tests {
    use encre_css::{Config, EncreGenerator};
    use std::fs;

    #[test]
    fn test() {
        let content = fs::read_to_string("tests/fixtures/icons.html").unwrap();
        let expected = fs::read_to_string("tests/fixtures/icons.css").unwrap();

        let mut config = Config::default();
        super::register(&mut config, Some("i-"), None, None);

        let mut generator = EncreGenerator::from_config(config);
        generator.scan(&content);

        assert_eq!(generator.generate(), expected);
    }
}
