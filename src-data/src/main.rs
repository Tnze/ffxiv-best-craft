use std::path::Path;

use dialoguer::{Confirm, Input};
use ironworks::{
    Error, Ironworks,
    excel::{Excel, Language},
    sqpack::{Install, SqPack},
};

mod metadata;

fn main() -> Result<(), Error> {
    println!("hello, world");
    let mut install = Install::search();
    let mut found_path = false;
    if let Some(install) = &install {
        println!("Install found: {:#?}", install);
        found_path = Confirm::new()
            .with_prompt("Do you want to continue?")
            .interact()
            .unwrap();
    }
    if !found_path {
        println!("Install not found");
        let path = Input::<String>::new()
            .with_prompt("FFXIV path?")
            .default(r"C:\Program Files (x86)\上海数龙科技有限公司\最终幻想XIV".to_string())
            .interact()
            .unwrap();
        install = Some(Install::at(Path::new(&path)))
    }
    let sqpack = SqPack::new(install.unwrap());
    let ironworks = Ironworks::new().with_resource(sqpack);

    // Read fields out of excel.
    let excel = Excel::new(ironworks).with_default_language(Language::ChineseSimplified);

    let sheet = excel.sheet(metadata::Item)?;
    println!("{:#?}", sheet.row(1));
    Ok(())
}
