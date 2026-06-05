use indoc::indoc;
use md_rs::{
    components::{
        code_block::codeblock,
        container::{Container, container},
        details::{Details, details},
        heading::{h2, h3, h4},
        list::{ListItem, list, listitem},
        span_nodes::paragraph,
    },
    github::components::alert::alert,
    md,
};

use crate::docs::{
    Documentation,
    book::{BookEntry, TopLevelEntry},
};

pub(super) struct Installation;

impl Documentation for Installation {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(h2().text("Installation"))
            .child(ArchLinux::docs())
            .child(Debian::docs())
            .child(Source::docs())
            .child(NixOS::docs())
            .child(PostInstallation::docs())
    }
}

impl TopLevelEntry for Installation {
    type Summary = Container;
    fn summary() -> Self::Summary {
        md().child(h2().with_text_underline("Installation"))
            .child(
                paragraph()
                    .text("Sherlock can be installed on a variety of Linux distributions.")
                    .br()
                    .text("Choose your distribution below to get started:"),
            )
            .child(list().style("-").items(Self::children().map(|child| {
                ListItem::from(paragraph().link(child.title, child.file.unwrap_or("#")))
            })))
    }
    fn children() -> impl Iterator<Item = BookEntry> + 'static {
        [
            BookEntry::of::<ArchLinux>()
                .with_title("Arch Linux")
                .with_file("arch-linux.md"),
            BookEntry::of::<Debian>()
                .with_title("Debian / Ubuntu")
                .with_file("debian.md"),
            BookEntry::of::<NixOS>()
                .with_title("NixOs")
                .with_file("nixos.md"),
            BookEntry::of::<Source>()
                .with_title("Build from Source")
                .with_file("source.md"),
        ]
        .into_iter()
    }
}

impl From<Installation> for BookEntry {
    fn from(_: Installation) -> Self {
        BookEntry {
            title: "Installation",
            file: Some("installation.md"),
            render_fn: Some(Installation::summary_md),
            children: Installation::children().collect(),
        }
    }
}

struct ArchLinux;
impl Documentation for ArchLinux {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(h3().with_text_underline("Arch Linux"))
            .child(
                "If you're using Arch Linux, you can install the pre-built \
                binary package with the follwing command:",
            )
            .child(
                codeblock()
                    .language("bash")
                    .content("yay -S sherlock-launcher-bin"),
            )
            .child(
                paragraph()
                    .text("Or install the community-maintained")
                    .code("git")
                    .text("build with the following command:"),
            )
            .child(
                codeblock()
                    .language("bash")
                    .content("yay -S sherlock-launcher-git"),
            )
    }
}

struct Source;
impl Documentation for Source {
    type Docs = Container;
    fn docs() -> Self::Docs {
        md().child(h3().with_text_underline("From Source"))
            .child(
                paragraph()
                    .text("To build Sherlock from source, follow these steps.")
                    .br()
                    .text("Make sure to have the following dependencies installed:"),
            )
            .child(BuildDependencies::docs())
            .child(
                details()
                    .summary(paragraph().html_strong("Build Steps:"))
                    .child(
                        list().items([
                            listitem()
                                .child(paragraph().bold("Clone the repository").text(":"))
                                .child(
                                    codeblock()
                                        .language("bash")
                                        .line("git clone https://github.com/skxxtz/sherlock.git")
                                        .line("cd sherlock"),
                                ),
                            listitem()
                                .child(
                                    paragraph()
                                        .bold("Build the project using the following command")
                                        .text(":"),
                                )
                                .child(codeblock().language("bash").line("cargo build --release")),
                            listitem()
                                .child(paragraph().bold("Install the binary").text(":"))
                                .child(
                                    "After the build completes, install the binary to your system:",
                                )
                                .child(
                                    codeblock()
                                        .language("bash")
                                        .line("sudo cp target/release/sherlock /usr/local/bin/"),
                                ),
                            listitem()
                                .child(
                                    paragraph()
                                        .bold("(Recommended) Remove the build directory")
                                        .text(":"),
                                )
                                .child("You can optionally remove the source code directory")
                                .child(
                                    codeblock()
                                        .language("bash")
                                        .line("rm -rf /path/to/sherlock"),
                                ),
                        ]),
                    ),
            )
    }
}

struct NixOS;
impl Documentation for NixOS {
    type Docs = Container;
    fn docs() -> Self::Docs {
        let non_flake_systems = container().child(h4().text("Non-Flake Systems")).child(
            paragraph()
                .text("Sherlock is available in")
                .code("nixpkgs/unstable")
                .text("as")
                .code("sherlock-launcher")
                .text(".")
                .text("If you're installing it as a standalone package,")
                .text("you'll need to do the")
                .link("config setup", "#config-setup")
                .text("yourself."),
        );

        let flakes_with_home_manager = container()
            .child(h4().text("Flakes & Home-Manager"))
            .child(
                paragraph()
                    .text("A module for Sherlock is available in home manager.")
                    .text("You can find it's configuration")
                    .link(
                        "here",
                        "https://github.com/nix-community/home-manager/blob\
                        /master/modules/programs/sherlock.nix",
                    )
                    .text(".")
                    .text("If you want to use the lastest updates and module options,")
                    .text("follow the steps below."),
            )
            .child(
                details()
                    .summary(paragraph().html_strong("Home-Manager Example Configuration"))
                    .child(
                        paragraph()
                            .text("Add the floowing")
                            .code("inputs")
                            .text("of")
                            .code("flake.nix")
                            .text("if you want to use the lastest upstream version of Sherlock."),
                    )
                    .child(codeblock().language("nix").content(indoc! {r#"
                            sherlock = {
                                url = "github:Skxxtz/sherlock";
                                inputs.nixpkgs.follows = "nixpkgs";
                            };
                        "#}))
                    .child("Home-Manager config:")
                    .child(codeblock().language("nix").content(indoc! {r#"
                        programs.sherlock = {
                            enable = true;

                            # to run sherlock as a daemon
                            systemd.enable = true;

                            # If wanted, you can use this line for the _latest_ package.
                            # Otherwise, you're relying on nixpkgs to update it frequently enough.
                            # For this to work, make sure to add sherlock as a flake input!
                            # package = inputs.sherlock.packages.${pkgs.system}.default;

                            # config.toml
                            settings = {};

                            # sherlock_alias.json
                            aliases = {
                                vesktop = { name = "Discord"; };
                            };

                            # sherlockignore
                            ignore = ''
                                Avahi*
                            '';

                            # fallback.json
                            launchers = [
                                {
                                    name = "Calculator";
                                    type = "calculation";
                                    args = {
                                        capabilities = [
                                            "calc.math"
                                            "calc.units"
                                        ];
                                    };
                                    priority = 1;
                                }
                                {
                                    name = "App Launcher";
                                    type = "apps";
                                    args = {};
                                    priority = 2;
                                    home = "Home";
                                }
                            ];
                        };
                        "#})),
            );

        let flakes_without_home_manager = container()
            .child(h4().text("Flakes without Home-Manager"))
            .child(
                paragraph()
                    .text("To install the standalone package, add")
                    .code("sherlock.packages.${pkgs.system}.default")
                    .text("to")
                    .code("environment.systemPackages")
                    .text("/")
                    .code("home.packages")
                    .text(".")
                    .text("You will need to create the configuration files yourself, see below."),
            );

        md().child(h3().with_text_underline("NixOs"))
            .child(non_flake_systems)
            .child(flakes_with_home_manager)
            .child(flakes_without_home_manager)
    }
}

struct Debian;
impl Documentation for Debian {
    type Docs = Container;
    fn docs() -> Self::Docs {
        let build_step_1 = listitem()
            .child(
                paragraph()
                    .bold("Install the")
                    .code("cargo-deb")
                    .bold("tool:"),
            )
            .child(
                paragraph()
                    .text("First, you need to install the")
                    .code("cargo-deb")
                    .text("tool, which specifies packaging Rust projects as Debian packages:"),
            )
            .child(codeblock().language("bash").line("cargo deb"));

        let build_step_2 = listitem()
            .child(paragraph().bold("Build the Debian package").text(":"))
            .child(
                paragraph()
                    .text("After installing")
                    .code("cargo-deb")
                    .text(", run the following command to build the")
                    .code(".deb")
                    .text("package:"),
            )
            .child(codeblock().language("bash").line("cargo deb"));

        let build_step_3 = listitem()
            .child(
                paragraph()
                    .bold("Install the generated")
                    .code(".deb")
                    .bold("package")
                    .text(":"),
            )
            .child("Once the package is built, you can install it using:")
            .child(codeblock().language("bash").line(concat!(
                "sudo dpkg -i target/debian/sherlock-launcher_v",
                env!("CARGO_PKG_VERSION"),
                "_amd64.deb"
            )))
            .child(
                alert().child("You can also use tab-completion to auto complete the file name."),
            );

        md().child(h3().with_text_underline("Build Debian Package"))
            .child(
                paragraph()
                    .text("To build a")
                    .code(".deb")
                    .text("package directly from source, follow these steps:"),
            )
            .child("Make sure you have the following dependencies installed:")
            .child(BuildDependencies::docs())
            .child(
                details()
                    .summary(paragraph().html_strong("Build Steps:"))
                    .child(list().items([build_step_1, build_step_2, build_step_3])),
            )
    }
}

struct BuildDependencies;
impl Documentation for BuildDependencies {
    type Docs = Details;
    fn docs() -> Self::Docs {
        details()
            .summary(paragraph().html_strong("Dependencies"))
            .child(
                list().items([
                    ListItem::from(paragraph().code("rust").text("-").link(
                        "How to install rust",
                        "https://www.rust-lang.org/tools/install",
                    )),
                    ListItem::from(paragraph().code("git").text("-").link(
                        "How to install git",
                        "https://github.com/git-guides/install-git",
                    )),
                    ListItem::from(paragraph().code("gtk-4-layer-shell").text("-").link(
                        "GTK4 Layer Shell",
                        "https://github.com/wmww/gtk4-layer-shell",
                    )),
                    ListItem::from(
                        paragraph()
                            .code("dbus")
                            .text("-")
                            .text("(Used to get currently playing song)"),
                    ),
                ]),
            )
    }
}

struct PostInstallation;
impl Documentation for PostInstallation {
    type Docs = Container;
    fn docs() -> Self::Docs {
        let config_files = list().items([
            ListItem::from(
                paragraph()
                    .link_bold(
                        "config.toml",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/examples/config.toml",
                    )
                    .text(":")
                    .text("This file specifies the behavior and defaults of your launcher.")
                    .link(
                        "Documentation",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/config.md",
                    ),
            ),
            ListItem::from(
                paragraph()
                    .link_bold(
                        "fallback.json",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/examples/fallback.json",
                    )
                    .text(":")
                    .text("This file specifies the features your launcher should have.")
                    .link(
                        "Documentation",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/launchers.md",
                    ),
            ),
            ListItem::from(
                paragraph()
                    .link_bold(
                        "sherlock_alias.json",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/examples/sherlock_alias.json",
                    )
                    .text(":")
                    .text("This file spcifies aliases for applications.")
                    .link(
                        "Documentation",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/aliases.md",
                    ),
            ),
            ListItem::from(
                paragraph()
                    .link_bold(
                        "sherlockignore",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/examples/sherlockignore",
                    )
                    .text(":")
                    .text("This file specifies applications to exclude from your search.")
                    .link(
                        "Documentation",
                        "https://github.com/Skxxtz/sherlock/blob\
                        /main/docs/sherlockignore.md",
                    ),
            ),
        ]);

        md().child(h2().text("Post Installation"))
            .child(h3().text("Config Setup"))
            .child(
                paragraph()
                    .text(
                        "After the installation is completed, \
                        you can set up your configuration files. \
                        Those files live in the",
                    )
                    .code("~/.config/sherlock/")
                    .text(
                        "directory. Depending on your needs, \
                        you should add the following files: ",
                    ),
            )
            .child(config_files)
            .child(
                paragraph()
                    .text("As of")
                    .code("version 0.1.11")
                    .text(", Sherlock comes with the")
                    .code("init")
                    .text("subcommand to automatically create your config.")
                    .text(
                        "This will create versions of the files above, \
                        populated with the default values. Additionally, it will create the",
                    )
                    .code("icons/")
                    .text(",")
                    .code("scripts/")
                    .text(", and")
                    .code("themes/")
                    .text("subdirectories.")
                    .text("All you have to do is run the following command:"),
            )
            .child(codeblock().language("bash").line("sherlock init"))
    }
}
