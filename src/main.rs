use std::{fs, path::{self, Path}, process::Command};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Scaffoldify {
    #[clap(subcommand)]
    pub commands: Subcommands,
}


#[derive(Debug, Subcommand)]
pub enum Subcommands {
   //  create differnt scaffolds
   #[clap(name = "create", short_flag = 'c')] 
   Create(Scaffolds),
   #[clap(name = "new", short_flag = 'n')] 
   New(Scaffolds),
   #[clap(name = "pattern", short_flag = 'p')] 
   Pattern(PatternScaffolds),
}

#[derive(Debug,  Args)]
pub struct PatternScaffolds {
    pub name: Option<String>,
    #[clap(subcommand)]
    pub available_patterns: AvailablePatterns,
}

#[derive(Debug, Subcommand)]
pub enum AvailablePatterns {
    #[clap(name = "css", short_flag = 'c')] 
    Css(CssOptions),
}

#[derive(Debug,  Args)]
pub struct CssOptions {
    #[clap(subcommand)]
    pub css_variants: CssVariants
}

#[derive(Debug, Subcommand)]
pub enum CssVariants {
    #[clap(name = "root", short_flag = 'r')] 
    Root,
    #[clap(name = "global", short_flag = 'g')] 
    Global,
}

#[derive(Debug, Args)]
pub struct Scaffolds {
    pub name: Option<String>,
    #[clap(subcommand)]
    pub available_scaffolds: AvailableScaffolds,
}

#[derive(Debug, Subcommand)]
pub enum AvailableScaffolds { 
    #[clap(name = "react", short_flag = 'r')] 
    React,
    #[clap(name = "lit", short_flag = 'l')] 
    Lit,
    #[clap(name = "go", short_flag = 'g')] 
    Go,
}

fn to_title_case(file_name: &str) -> String {
    file_name
    .chars()
    .enumerate()
    .map(|(pos, letter)| {
        if pos == 0 {
            return letter.to_ascii_uppercase()
        }

        return letter
    })
    .collect()
}

fn create_go_file(file_name: &str, current_dir: &Path) {
    let contents = format!("package main\n import ( clog \"github.com/charmbracelet/log\" )\n func {}() {{\nclog.Info(\"Hello, {}!\")\n}}", file_name, file_name);
    let test_contents = format!("package main\nimport (\"testing\"\n\"github.com/stretchr/testify/assert\")\nfunc Test{}(t *testing.T) {{ assert.Equal(t, 123, 123, \"\")}}", to_title_case(file_name));

    let files = vec![
        (file_name.to_string() + ".go", contents),
        (file_name.to_string() + "_test.go", test_contents), 
    ];

    for file in files {
        fs::write(current_dir.join(file.0), file.1).unwrap();
    }
    std::process::Command::new("go")
    .arg("fmt")
    .current_dir(current_dir)
    .output()
    .unwrap();
}

fn new_go_project(project_name: String) {
std::fs::create_dir_all(&project_name).unwrap();

let current_dir = format!("{}/{}", std::env::current_dir().unwrap().display(), project_name);

std::process::Command::new("go")
.arg("mod")
.arg("init")
.arg(&project_name)
.current_dir(&current_dir)
.output()
.unwrap();

let dependencies = vec!["github.com/charmbracelet/log", "github.com/stretchr/testify"];

for dependency in dependencies {
    std::process::Command::new("go")
    .arg("get")
    .arg(dependency)
    .current_dir(&current_dir)
    .output()
    .unwrap();
}

create_go_file("main", &Path::new(&current_dir));
}



fn get_name(project_name: Option<String>, default: &str) -> String {
    match project_name {
        Some(p) => p,
        None => default.to_string()
    }
}

fn new_lit_project(project_name: String) {
    std::fs::create_dir_all(&project_name).unwrap();

    let current_dir = format!("{}/{}", std::env::current_dir().unwrap().display(), project_name);
    
    std::process::Command::new("git")
    .arg("clone")
    .arg("https://github.com/serranoio/lit.git")
    .arg(".")
    .current_dir(&current_dir)
    .output()
    .unwrap();
}

fn create_lit_files(component_name: &String) {
    std::fs::create_dir_all(&component_name).unwrap();
    // contents,
    // name
    let files: Vec<(String, String)> = vec![
        (format!("{}-css.ts", component_name), format!("import {{ css }} from \"lit\";

export default css`
{}
`;", global_reset_css())),

(format!("{}-element.ts", component_name), format!("import {{ CSSResultGroup, LitElement, html }} from 'lit'
import {{ customElement, property }} from 'lit/decorators.js'
import {}Css from './{}-css'
        
@customElement('{}-element')
export class {}Element extends LitElement {{
static styles?: CSSResultGroup | undefined = [{}Css]

@property()
count: number;

constructor() {{
super()

this.count = 0;
}}

render() {{
return html`
    <div>
    ${{this.count}}
    Hello, World!
    </div>
`
}}
}}", component_name, component_name, component_name, component_name, component_name))
    ];

    let path = Path::new(component_name);
    for file in files {

        let path = Path::new(path).join(file.0);
        fs::write(path, file.1).unwrap();
    }
}

fn new_react_project(project_name: String) {
    std::fs::create_dir_all(&project_name).unwrap();
    let current_dir = format!("{}/{}", std::env::current_dir().unwrap().display(), project_name);

    println!("Cloning...");
    std::process::Command::new("git")
    .arg("clone")
    .arg("https://github.com/serranoio/react.git")
    .arg(".")
    .current_dir(&current_dir)
    .output()
    .unwrap();

    println!("installing dep...");
    Command::new("npm")
    .arg("i")
    .current_dir(&current_dir)
    .output()
    .unwrap();
}


fn root_css() -> String {

let css = ":root {
--brandColorL50: #f5fbf5;
--brandColorL40: #dff2df;
--brandColorL30: #bee6be;
--brandColorL20: #9ed99e;
--brandColorL10: #7ecd7e;
--brandColor: #5cc05c;
--brandColorD10: #49b849;
--brandColorD20: #40a540;
--brandColorD30: #389038;
--brandColorD40: #307b30;
--brandColorD50: #286728;

--white: #ffffff;
--gray98: #f7f8fa;
--gray96: #f2f4f5;
--gray92: #e1e6eb;
--gray80: #c3cbd4;
--gray60: #818d99;
--gray45: #5c6773;
--gray30: #3c444d;
--gray25: #31373e;
--gray22: #2b3033;
--gray20: #171d21;
--black: #000000;

--accentColorL50: #ecf8ff;
--accentColorL40: #bfe9ff;
--accentColorL30: #7ed2ff;
--accentColorL20: #3ebcff;
--accentColorL10: #00a4fd;
--accentColor: #007abd;
--accentColorD10: #006eaa;
--accentColorD20: #006297;
--accentColorD30: #005684;
--accentColorD40: #004a71;
--accentColorD50: #003d5e;

--errorColorL50: #fcedec;
--errorColorL40: #f8dcd9;
--errorColorL30: #f1b9b3;
--errorColorL20: #ea958d;
--errorColorL10: #e37267;
--errorColor: #dc4e41;
--errorColorD10: #c84535;
--errorColorD20: #b23d30;
--errorColorD30: #9c3529;
--errorColorD40: #852d24;
--errorColorD50: #6f261d;

--alertColorL50: #fef3ec;
--alertColorL40: #fde6d9;
--alertColorL30: #facdb3;
--alertColorL20: #f7b48c;
--alertColorL10: #f49b66;
--alertColor: #f1813f;
--alertColorD10: #da742e;
--alertColorD20: #c2672a;
--alertColorD30: #aa5a25;
--alertColorD40: #914d1f;
--alertColorD50: #79401a;

--warningColorL50: #fff9eb;
--warningColorL40: #fef2d7;
--warningColorL30: #fde5ae;
--warningColorL20: #fbd886;
--warningColorL10: #facb5d;
--warningColor: #f8be34;
--warningColorD10: #e0ac16;
--warningColorD20: #c79915;
--warningColorD30: #ae8613;
--warningColorD40: #957312;
--warningColorD50: #7d600f;

--successColorL50: #eef6ee;
--successColorL40: #ddecdd;
--successColorL30: #bbd9ba;
--successColorL20: #98c697;
--successColorL10: #76b374;
--successColor: #53a051;
--successColorD10: #479144;
--successColorD20: #40813d;
--successColorD30: #387135;
--successColorD40: #2f612e;
--successColorD50: #275126;

--infoColorL50: #e5f0f5;
--infoColorL40: #cce2eb;
--infoColorL30: #99c5d7;
--infoColorL20: #66a7c4;
--infoColorL10: #338ab0;
--infoColor: #006d9c;
--infoColorD10: #00577c;
--infoColorD20: #004c6c;
--infoColorD30: #00415d;
--infoColorD40: #00364d;
--infoColorD50: #002b3e;

--textColor: #3c444d;
--textGray: #6b7785;
--textDisabledColor: #c3cbd4;
--linkColor: #006eaa;
--borderRadius: 0.3rem;
--border: 0.1 solid #c3cbd4;

--sansFontFamily: 'Splunk Platform Sans', 'Proxima Nova', Roboto, Droid, 'Helvetica Neue', Helvetica, Arial, sans-serif;
--serifFontFamily: Georgia, 'Times New Roman', Times, serif;
--monoFontFamily: 'Splunk Platform Mono', Inconsolata, Consolas, 'Droid Sans Mono', Monaco, 'Courier New', Courier, monospace;
--fontFamily: 'Splunk Platform Sans', 'Proxima Nova', Roboto, Droid, 'Helvetica Neue', Helvetica, Arial, sans-serif;

--fontWeightLight: 300;
--fontWeightNormal: 400;
--fontWeightSemiBold: 500;
--fontWeightBold: 700;
--fontWeightHeavy: 800;
--fontWeightExtraBold: 900;

--zindexLayer: 1000;
--zindexFixedNavbar: 1030;
--zindexModalBackdrop: 1040;
--zindexModal: 1050;
--zindexPopover: 1060;
--zindexToastMessages: 2000;

--spacingQuarter: .5rem;
--spacingHalf: 1.0rem;
--spacing: 2.0rem;
--fontSizeSmall: 1.2rem;
--fontSize: 1.4rem;
--fontSizeLarge: 1.6rem;
--fontSizeXLarge: 1.8rem;
--fontSizeXXLarge: 2.4rem;
--lineHeight: 2.0rem;
--inputHeight: 3.2rem;
--borderRadius: .3rem;
}
";

return format!("{} {}", css, global_reset_css()); 
}

fn global_reset_css() -> String {
return "
* {
margin: 0;
padding: 0;
box-sizing: border-box;
} 

html {
    font-size: 62.5%;
}
".to_string()
}

fn apply_global_css() {
    print!("{}", global_reset_css());
}

fn apply_root_css() {
    print!("{}", root_css());
}

fn create_react_files(component_name: String) {
    std::fs::create_dir_all(to_title_case(&component_name)).unwrap();
    let folder = Path::new(&component_name);

    let files = vec![
        (format!("{}.tsx", to_title_case(&component_name)), format!("import './{}.css'
import React from 'react'

function {}() {{
    return (
    <>
    </>
    )
}}

export default {}", to_title_case(&component_name), to_title_case(&component_name), to_title_case(&component_name))),
(format!("{}.css", to_title_case(&component_name)), format!("")),
];
    
    for file in files {
        fs::write(folder.join(file.0), file.1).unwrap();
    }
}

fn main() {
    let cli = Scaffoldify::parse();

    match cli.commands {
        Subcommands::New(scaffold) => {
            match scaffold.available_scaffolds {
                AvailableScaffolds::React => {
                    new_react_project(get_name(scaffold.name, "react"))
                },
                AvailableScaffolds::Lit => {
                    new_lit_project(get_name(scaffold.name, "new"))
                },
                AvailableScaffolds::Go => {
                    new_go_project(get_name(scaffold.name, "new"));
                },
            }
        }, Subcommands::Create(scaffold) => {
            match scaffold.available_scaffolds {
                AvailableScaffolds::React => {
                    create_react_files(get_name(scaffold.name, "Default.tsx"));
                },
                AvailableScaffolds::Lit => {
                    create_lit_files(&get_name(scaffold.name, "default.ts"))
                },
                AvailableScaffolds::Go => {
                    create_go_file(&get_name(scaffold.name, "util"), std::env::current_dir().unwrap().as_path())
                },
            }
        }, Subcommands::Pattern(scaffold) => {
            match scaffold.available_patterns {
                AvailablePatterns::Css(css_options) => {
                    match css_options.css_variants {
                        CssVariants::Global => {
                            apply_global_css();
                        },
                        CssVariants::Root => {
                            apply_root_css();
                        }
                    }
                }
            }
        }
    }

    println!("Done!");
}