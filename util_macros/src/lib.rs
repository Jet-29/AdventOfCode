use std::fs;

use proc_macro::TokenStream;
use quote::{format_ident, quote};

/// # Panics
///
/// Will panic is the directory is unable to be read.
#[proc_macro]
pub fn get_solutions(_: TokenStream) -> TokenStream {
    let year_dirs = fs::read_dir("src/")
        .expect("Failed to read directory")
        .map(|year| year.unwrap().file_name().into_string().unwrap());
    let years_found = year_dirs.filter(|year| year.replace("year_", "").parse::<u32>().is_ok());

    let mut year_modules = Vec::new();
    let mut year_inserts = Vec::new();

    for year_found in years_found {
        let solution_files = fs::read_dir(format!("src/{year_found}"))
            .unwrap()
            .map(|solution| {
                solution
                    .unwrap()
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
            });
        let module_name = format_ident!("{year_found}");
        let year_number = year_found.replace("year_", "").parse::<u32>().unwrap();
        let mut solution_modules = Vec::new();
        let mut solution_inserts = Vec::new();

        for solution_file in solution_files {
            let import_path = format!("{solution_file}.rs");
            let module_name = format_ident!("{solution_file}");
            let day_idx = solution_file
                .replace("day_", "")
                .parse::<u32>()
                .expect("Failed to parse day file");

            solution_modules.push(quote! {
                #[path=#import_path]
                mod #module_name;
            });
            solution_inserts.push(quote! {
                days.insert(#day_idx, &#module_name::solution as Day);
            });
        }

        year_modules.push(quote! {
            mod #module_name {
                use crate::{Day, Event};
                use std::collections::HashMap;

                #(#solution_modules)*

                pub fn get_event() -> Event {
                    let mut days = HashMap::new();
                    #(#solution_inserts)*
                    Event { days }

                }
            }
        });

        year_inserts.push(quote! {
            temp_solutions.insert(#year_number, #module_name::get_event());
        });
    }

    quote! {
        #(#year_modules)*

        let mut temp_solutions = HashMap::new();
        #(#year_inserts)*
        let solutions: HashMap<u32, Event> = temp_solutions;

    }
    .into()
}
