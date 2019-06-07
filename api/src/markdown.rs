use comrak::{ComrakOptions, markdown_to_html};

static COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
    // The defaults that would normally come from `..ComrakOptions::default()`
    default_info_string: None,
    ext_description_lists: false,
    ext_footnotes: false,
    ext_header_ids: None,
    ext_tagfilter: false,
    ext_tasklist: false,
    github_pre_lang: false,
    hardbreaks: false,
    unsafe_: false,
    width: 0,

    ext_autolink: true,      // GFM: https://github.github.com/gfm/#autolinks-extension-
    ext_strikethrough: true, // GFM: https://github.github.com/gfm/#strikethrough-extension-
    ext_table: true,         // GFM: https://github.github.com/gfm/#tables-extension-
    ext_superscript: true,   // Comrak: https://github.com/kivikakk/comrak/blob/master/src/parser/mod.rs#L312
    smart: true,             // https://github.com/kivikakk/comrak/blob/master/src/parser/mod.rs#L141
};

pub fn render(md: &str) -> String {
    markdown_to_html(md, &COMRAK_OPTIONS)
}
