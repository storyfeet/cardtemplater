use clap_conf::prelude::*;
use failure_derive::*;
use gtmpl::Template;
use gtmpl_helpers::THelper;

#[derive(Fail, Debug)]
#[fail(display = "{}", s)]
pub struct StrErr {
    s: String,
}

fn main() -> Result<(), failure::Error> {
    let clap = clap_app!(CardTemplates =>
        (about:"A program to build templates into whatever ourput you want")
        (author:"Matthew Stoodley")
        (version:crate_version!())
        (@arg template:-t --template +takes_value "The main template to run this program")
    )
    .get_matches();
    let cfg = with_toml_env(&clap, &["any_conf.toml"]);

    let mut template = Template::default().with_defaults().with_exec();

    let tfname = cfg.grab_local().arg("template").conf("template").req()?;
    let fs = std::fs::read_to_string(tfname)?;

    template.parse(fs).map_err(|e| StrErr { s: e })?;

    let res = template.q_render(4).map_err(|e| StrErr { s: e })?;
    println!("{}", res);

    Ok(())
}
