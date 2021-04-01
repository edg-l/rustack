use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakParseOptions, ComrakRenderOptions};

pub fn get_markdown_options() -> ComrakOptions {
    ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: Some("".into()),
            footnotes: true,
            ..ComrakExtensionOptions::default()
        },
        render: ComrakRenderOptions {
            escape: true,
            ..ComrakRenderOptions::default()
        },
        parse: ComrakParseOptions {
            smart: true,
            ..ComrakParseOptions::default()
        },
    }
}
