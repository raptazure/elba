use failure::ResultExt;
use inflector::Inflector;
use package::Name;
use std::{fs, path::PathBuf};
use util::errors::Res;

pub struct NewCtx {
    pub path: PathBuf,
    pub name: Name,
    // Tuple of name and email.
    pub author: Option<(String, String)>,
    pub bin: bool,
}

pub fn new(ctx: NewCtx) -> Res<String> {
    let path = &ctx.path;
    if fs::metadata(path).is_ok() {
        bail!(
            "destination `{}` already exists\n\n\
             create a new `elba.toml` manifest file in the directory instead",
            path.display()
        )
    }

    fs::create_dir_all(path).context(format_err!("could not create dir {}", path.display()))?;

    init(ctx)
}

pub fn init(ctx: NewCtx) -> Res<String> {
    let name = &ctx.name;
    let author = if let Some((author, email)) = ctx.author {
        format!("{} <{}>", author, email)
    } else {
        "".to_string()
    };
    let path = &ctx.path;

    let target = if ctx.bin {
        format!(
            r#"[[targets.bin]]
name = "{}"
main = "src/Main.idr"

"#,
            name.name()
        )
    } else {
        format!(
            r#"[targets.lib]
path = "src/"
mods = [
    "{}.{}"
]

"#,
            name.group().to_pascal_case(),
            name.name().to_pascal_case(),
        )
    };

    fs::write(
        &ctx.path.join("elba.toml"),
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
authors = [{}]

[dependencies]

{}"#,
            name, author, target
        ).as_bytes(),
    )?;

    if !ctx.bin {
        fs::create_dir_all(path.join(format!("src/{}", name.group().to_pascal_case())))
            .context(format_err!("could not create dir {}", path.display()))?;
        fs::write(
            &path.join(format!(
                "src/{}/{}.idr",
                name.group().to_pascal_case(),
                name.name().to_pascal_case()
            )),
            format!(
                r#"module {}.{}

hello : IO ()
hello = do
  print "Hello, world!"
"#,
                name.group().to_pascal_case(),
                name.name().to_pascal_case()
            ).as_bytes(),
        )?;
    } else {
        fs::create_dir_all(path.join("src"))
            .context(format_err!("could not create dir {}", path.display()))?;
        fs::write(
            &path.join("src/Main.idr"),
            r#"module Main

main : IO ()
main = do
  print "Hello, world!"
"#,
        )?;
    }

    Ok(format!(
        "new package with {} target created at {}",
        if ctx.bin { "binary" } else { "library" },
        path.display()
    ))
}
