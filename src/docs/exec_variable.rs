use indoc::indoc;
use md_rs::{
    components::{
        code_block::{CodeBlock, codeblock},
        container::Container,
        details::details,
        heading::{h1, h2, h3, h4},
        list::{ListItem, list},
        span_nodes::{Paragraph, paragraph},
    },
    github::components::alert::tip,
    md,
};

use crate::{docs::Documentation, loader::utils::ExecVariable};

struct ExecVariableDoc {
    title: &'static str,
    name: &'static str,
    description: fn() -> Paragraph,
    example: &'static str,
}

const EXEC_VARIABLE_DOCS: &[ExecVariableDoc] = &[
    ExecVariableDoc {
        title: "String Input",
        name: "string_input",
        description: || "A plain text input field.".into(),
        example: r#"{ "type": "string_input", "value": "hello world" }"#,
    },
    ExecVariableDoc {
        title: "Password Input",
        name: "password_input",
        description: || "Like string_input but the value is masked in the UI.".into(),
        example: r#"{ "password_input": "sudo" }"#,
    },
    ExecVariableDoc {
        title: "Path Input",
        name: "path_input",
        description: || {
            paragraph()
                .text("A text input featuring path completion.")
                .text("By default, completion paths are resolved relative to the")
                .code("$HOME")
                .text("directory. Starting the input with a")
                .code("/")
                .text("prefix will search from the system root instead.")
        },
        example: r#"{ "path_input": "path" }"#,
    },
    ExecVariableDoc {
        title: "Command Input",
        name: "command_input",
        description: || {
            paragraph()
                .text("A text input featuring path completion.")
                .text("Unlike")
                .code("path_input")
                .text(", this will only look at executeable files.")
                .text("First, it will look at the")
                .code("$PATH")
                .text(", then it will try to complete like")
                .text("path_input")
        },
        example: r#"{ "command_input": "command" }"#,
    },
    ExecVariableDoc {
        title: "Choice Input",
        name: "choice",
        description: || {
            paragraph()
                .text("Presents the user with a predefined list of options to select from.")
                .text("Each choice has a")
                .code("label")
                .text("shown in the UI and a")
                .code("value")
                .text("passed to the command.")
        },
        example: indoc! {r#"{
            "choice": {
                "name": "temperature",
                "choices": [
                    {"label": "5000", "value": "5000"},
                    {"label": "6000 <span color='#555555'><i>default</i></span>", "value": "6000"},
                    {"label": "7000", "value": "7000"},
                    {"label": "8000", "value": "8000"}
                ]
            }
        }"#},
    },
];

impl Documentation for ExecVariable {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(h1().text("Exec Variables"))
            .child(ReplacementVariables::docs())
            .child(
                paragraph()
                    .text("In Sherlock")
                    .bold("Exec Variables")
                    .text(
                        "are dynamic placeholders that allow you \
                        to inject real-time arguments into your applications, scripts, \
                        and commands right at the moment you launch them.",
                    )
                    .text("Instead of relying on hardcoded shortcuts,")
                    .italic("Exec Variables")
                    .text("turn your launcher into an interactive CLI shell."),
            )
            .children(EXEC_VARIABLE_DOCS.iter().map(|doc| {
                Container::default()
                    .child(h2().text(doc.title))
                    .child(paragraph().code(doc.name))
                    .child((doc.description)())
                    .child(codeblock().language("json").content(doc.example))
            }))
    }
}

struct ReplacementVariableToken {
    title: Paragraph,
    body: Container,
    syntax_example: CodeBlock,
}

struct ReplacementVariables;
impl Documentation for ReplacementVariables {
    type Docs = Container;
    fn docs() -> Self::Docs {
        let tokens: Vec<ReplacementVariableToken> = vec![
            ReplacementVariableToken {
                title: paragraph().code("{keyword}"),
                body: md().child(
                    "Replaces the token with the exact text \
                    currently typed into the search bar.",
                ),
                syntax_example: codeblock().line("https://www.example.com/search?q={keyword}"),
            },
            ReplacementVariableToken {
                title: paragraph().code("{terminal}"),
                body: md()
                    .child(
                        "Automatically resolbes the user-defined \
                        or system-detected default terminal emulator.",
                    )
                    .child(
                        tip().child(
                            paragraph()
                                .text(
                                    "Most terminal emulators close instant \
                                    once their child process finishes executing. \
                                    To keep the terminal window \
                                    open after your command runs,\
                                    wrap the execution string like this:",
                                )
                                .code(r#"{terminal} sh -c "<command>; exec $SHELL""#),
                        ),
                    ),
                syntax_example: codeblock().language("json").line(
                    r#"{terminal} sh -c \"ssh {variable:user}@{variable:host}; exec $SHELL\""#,
                ),
            },
            ReplacementVariableToken {
                title: paragraph().code("{variable:<name>}"),
                body: md().child(
                    paragraph()
                        .text("Inserts the Exec Variable into the command.")
                        .text(
                            "This token only works if a matching input field \
                            is explicitly declared in the configuration's",
                        )
                        .code("variables")
                        .text("array"),
                ),
                syntax_example: codeblock().language("json").content(indoc! {r#"
                        {
                            "variables": [
                                { "string_input": "query" }
                            ],
                            "exec": "https://example.com/search?q={variable:query}"
                        }
                    "#}),
            },
            ReplacementVariableToken {
                title: paragraph().code("{prefix[<variable name>]:<prefix text>}"),
                body: md()
                    .child(
                        "A conditional modifier token used used to \
                        handle optional inputs gracefully.",
                    )
                    .child(
                        list().style("-").items([
                            ListItem::from(
                                paragraph()
                                    .text("If the specified variable")
                                    .bold("contains a value")
                                    .text(", the entire token evaluates to the")
                                    .code("<prefix text>")
                                    .text("."),
                            ),
                            ListItem::from(
                                paragraph()
                                    .text("If the variable")
                                    .bold("is empty or unassigned")
                                    .text(", the entire token resolves to an emptry string")
                                    .code(r#""""#)
                                    .text("."),
                            ),
                        ]),
                    )
                    .child(
                        "This is highly effective for injecting optional CLI flags \
                        or toggling between a website's landing pgae and its search index.",
                    ),
                syntax_example: codeblock().language("json").content(indoc! {r#"
                        {
                            "variables": [
                                { "string_input": "query" }
                            ],
                            "exec": "https://example.com/{prefix[query]:search?q=}{variable:query}"
                        }
                    "#}),
            },
        ];
        let example = codeblock().language("json").content(indoc! {r#"
        {
            "name": "System & Network Utils",
            "type": "command",
            "args": {
                "commands": {
                    "SSH Tunnel": {
                        "icon": "sherlock-link",
                        "variables": [
                            { "string_input": "User" },
                            { "string_input": "Host" }
                        ],
                        "exec": "{terminal} ssh {variable:User}@{variable:Host}",
                        "search_string": "ssh"
                    },
                    "NordVPN Connect": {
                        "icon": "nordvpn",
                        "variables": [
                            { "choice": { "name": "Server", "choices": ["us", "uk", "de"] } }
                        ],
                        "exec": "{terminal} sh -c \"nordvpn c {variable:Server}; exec $SHELL\"",
                        "search_string": "nordvpn"
                    },
                    "NordVPN Daemon": {
                        "icon": "nordvpn",
                        "exec": "systemctl --user start nordvpnd",
                        "search_string": "nordvpn daemon"
                    }
                }
            },
            "priority": 1
        }"#});

        md().child(h2().text("Replacement Variable Notation"))
            .child(
                "The replacement variable notation allows you to \
                dynamically replace tokens in commands with contextual data–\
                such as the user's active search query, system environment settings \
                or runtime Exec Variables.",
            )
            .child(h3().text("Available Tokens"))
            .child(list().items(tokens.into_iter().map(|token| {
                md().child(token.title)
                    .child(token.body)
                    .child(h4().text("Syntax Example:"))
                    .child(token.syntax_example)
            })))
            .child(
                details()
                    .summary(paragraph().bold("Complete Configuration Example:"))
                    .child(
                        "Heres a practical look at how there replacement variables mesh \
                        inside a launcher configuration file.",
                    )
                    .child(example),
            )
    }
}
