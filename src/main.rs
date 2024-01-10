use std::collections::BTreeMap;

use clap::{App, Arg};
#[allow(warnings)]
fn main() {

    match matches.subcommand_name() {
        Some("delete") => {
            if let Some(sub_m) = matches.subcommand_matches("delete") {
                if let Some(pages) = sub_m.get_one::<String>("pages").map(|s| s.as_str()) {
                    let pages_vec = pages
                        .replace("[", "")
                        .replace("]", "")
                        .split(",")
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    let file_path = matches
                        .get_one::<String>("file")
                        .expect("`file`is required");
                    // println!("{:?}", pages_vec);
                    // println!("{}", file_path);
                    delete_pages(file_path, pages_vec);
                }
            }
        }
        Some("split") => {
            if let Some(sub_m) = matches.subcommand_matches("split") {
                if let Some(num) = sub_m.get_one::<String>("num").map(|s| s.as_str()) {
                    let start_num = num.parse::<u32>().unwrap();
                    let file_path = matches
                        .get_one::<String>("file")
                        .expect("`file`is required");
                    split_pages(file_path, start_num);
                }
            }
        }
        Some("merge") => {
            if let Some(sub_m) = matches.subcommand_matches("merge") {
                if let Some(path) = sub_m.get_one::<String>("path").map(|s| s.as_str()) {
                    let file_path_0 = matches
                        .get_one::<String>("file")
                        .expect("`file`is required");
                    let file_path_1 = path.to_string();
                    merge_pdf(file_path_0, &file_path_1);
                }
            }
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
