use indoc::indoc;
use md_rs::{
    components::{
        Component, code_block::code_block, heading::Heading, hr::hr, span_nodes::paragraph,
        table::table,
    },
    md,
};

pub const BASE_FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "name",
        ty: "string",
        required: false,
        default: None,
        description: "Display name of the launcher. Shown in the widget if set.",
    },
    FieldDoc {
        name: "alias",
        ty: "string",
        required: false,
        default: None,
        description: "Short trigger prefix (e.g. `app`) that scopes the launcher to alias-only mode.",
    },
    FieldDoc {
        name: "type",
        ty: "LauncherVariant",
        required: true,
        default: None,
        description: "The category and functional variant of the launcher.",
    },
    FieldDoc {
        name: "priority",
        ty: "u16",
        required: true,
        default: None,
        description: "Display order weight. Lower values appear first. `0` only appears in alias mode.",
    },
    FieldDoc {
        name: "home",
        ty: "HomeType",
        required: false,
        default: Some("Home"),
        description: "Controls when the launcher is shown: `Home`, `OnlyHome`, `Search`, or `Persist`.",
    },
    FieldDoc {
        name: "exit",
        ty: "bool",
        required: false,
        default: Some("true"),
        description: "Whether Sherlock closes after the launcher is executed.",
    },
    FieldDoc {
        name: "shortcut",
        ty: "bool",
        required: false,
        default: Some("true"),
        description: "Whether a UI shortcut key is assigned to this launcher.",
    },
    FieldDoc {
        name: "spawn_focus",
        ty: "bool",
        required: false,
        default: Some("true"),
        description: "Whether this launcher can receive spawned focus.",
    },
    FieldDoc {
        name: "on_return",
        ty: "string",
        required: false,
        default: None,
        description: "Command or action executed when the user confirms this launcher.",
    },
    FieldDoc {
        name: "args",
        ty: "object",
        required: false,
        default: Some("{}"),
        description: "Launcher-specific arguments. Shape depends on the launcher type.",
    },
    FieldDoc {
        name: "binds",
        ty: "Bind[]",
        required: false,
        default: None,
        description: "Key bindings attached to this launcher.",
    },
    FieldDoc {
        name: "actions",
        ty: "ApplicationAction[]",
        required: false,
        default: None,
        description: "Primary context menu actions. Overwrites any actions defined in desktop files.",
    },
    FieldDoc {
        name: "add_actions",
        ty: "ApplicationAction[]",
        required: false,
        default: None,
        description: "Supplementary actions appended to the primary action list.",
    },
    FieldDoc {
        name: "variables",
        ty: "ExecVariable[]",
        required: false,
        default: None,
        description: "Runtime variable substitutions available to this launcher's commands.",
    },
];

pub trait LauncherDoc {
    fn doc() -> LauncherDocEntry;
}

pub struct LauncherDocEntry {
    pub name: &'static str,
    pub variant_name: &'static str,
    pub description: &'static str,
    pub args: &'static [FieldDoc],
    pub inner_functions: &'static [InnerFunctionDoc],
    pub examples: &'static [Example],
    pub hidden: bool,
}

impl LauncherDocEntry {
    pub fn new() -> Self {
        Self {
            name: "",
            variant_name: "",
            description: "",
            args: &[],
            inner_functions: &[],
            examples: &[],
            hidden: false,
        }
    }
    pub fn new_hidden(
        name: &'static str,
        variant_name: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            variant_name,
            description,
            hidden: true,
            ..Self::new()
        }
    }
}

pub struct InnerFunctionDoc {
    pub name: &'static str,
    pub identifier: &'static str,
    pub description: &'static str,
}

pub struct FieldDoc {
    pub name: &'static str,
    pub ty: &'static str,
    pub required: bool,
    pub default: Option<&'static str>,
    pub description: &'static str,
}

pub struct Example {
    pub description: &'static str,
    pub json: &'static str,
}

// markdown
pub fn to_markdown(entries: &[LauncherDocEntry]) -> String {
    let mut out = String::new();
    md()
        .child(Heading::new(1, "Launchers"))
        .child(
            paragraph()
            .with_text("Launchers are the backbone for Sherlock's widget engine. Each displayed widget is owned by a launcher. For example:"))
        .child(
            code_block().content(indoc! {"
            [Weather Launcher]
                [Widget] Weather Display
            [App Launcher]
                [Widget] App 1
                [Widget] App 2
                [Widget] App 3"})
        )
        .child(paragraph().with_text("A launcher's widgets will share the same behavior based on their shared launcher configuration."))
        .child(Heading::new(2, "Shared Launcher Configuration"))
        .child(Heading::new(3, "Fields"))
        .child(table()
            .headers(["Field", "Type", "Required", "Default", "Description"])
            .rows(BASE_FIELDS.iter().map(|f| [
                    format!("`{}`", f.name),
                    format!("`{}`", f.ty),
                    if f.required { "✓" } else { "" }.into(),
                    f.default.unwrap_or("—").into(),
                    f.description.into(),
            ]))
            )
        .children(
            entries.iter().filter(|e| !e.hidden).map(|e| {
                md()
                    .child(Heading::new(2, e.name))
                    .child(paragraph().with_text(format!("`type = {}`", e.variant_name)))
                    .child(paragraph().with_text(e.description))
                    .when(!e.args.is_empty(), |this| {
                        this
                            .child(Heading::new(3, "Args"))
                            .child(table()
                                .headers(["Field", "Type", "Required", "Default", "Description"])
                                .rows(e.args.iter().map(|f| [
                                    format!("`{}`", f.name),
                                    format!("`{}`", f.ty),
                                    if f.required { "✓" } else { "" }.into(),
                                    f.default.unwrap_or("—").into(),
                                    f.description.into()
                                ]))
                            )
                    })
                    .when(!e.inner_functions.is_empty(), |this| {
                        this
                            .child(Heading::new(3, "Inner Functions"))
                            .child(table()
                                .headers(["Name", "Identifier", "Description"])
                                .rows(e.inner_functions.iter().map(|f| [
                                    f.name.into(),
                                    format!("`{}`", f.identifier),
                                    f.description.into()
                                ]))
                            )
                    })
                    .when(!e.examples.is_empty(), |this| {
                        this
                            .child(Heading::new(3, "Examples"))
                            .children(e.examples.iter().map(|ex| {
                                md()
                                    .child(paragraph().with_text_italic(ex.description))
                                    .child(code_block().language("json").content(ex.json.trim()))
                            }))
                    })
                    .child(hr())
            })
        )
        .render(&mut out);

    out
}
