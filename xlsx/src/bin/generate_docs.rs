#![allow(clippy::panic)]
#![allow(clippy::expect_used)]

//! Produces documentation of all the implemented IronCalc functions
//! and saves the result to functions.md
//!
//! Usage: documentation

use std::fs;

use serde::Serialize;
use std::io::{self, BufRead};

#[derive(Serialize)]
struct FunctionItem {
    text: String,
    link: String,
}

#[derive(Serialize)]
struct CategoryItem {
    text: String,
    collapsed: bool,
    link: String,
    items: Vec<FunctionItem>,
}

#[derive(Serialize)]
struct Item {
    text: String,
    collapsed: bool,
    items: Vec<CategoryItem>,
}

fn main() -> io::Result<()> {
    // Step 1: Create "docs" directory in the working directory
    let docs_dir = "docs";
    fs::create_dir_all(docs_dir)?;

    // Step 2: Read files from the "functions" directory
    let functions_dir = "functions";

    let mut category_items = Vec::new();

    for entry in fs::read_dir(functions_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only process files (skip directories)
        if path.is_file() {
            // Get the file name without extension
            if let Some(category) = path.file_stem().and_then(|s| s.to_str()) {
                // Create a directory in "docs" with the name of the file
                let target_dir = format!("{}/{}", docs_dir, category);
                fs::create_dir_all(&target_dir)?;

                // Open the file and read lines
                let file = fs::File::open(&path)?;
                let reader = io::BufReader::new(file);

                let mut file_items = Vec::new();

                for line in reader.lines() {
                    let line = line?;
                    let function_name = line.trim().to_lowercase();
                    let function_name_upper_case = function_name.to_uppercase();

                    if function_name.is_empty() {
                        continue;
                    }

                    // Create a file with the name from the line, ending with .md
                    let file_name = format!("{}/{}.md", target_dir, function_name);

                    // Write "Hello World" into the file
                    fs::write(
                        &file_name,
                        format!(
                            r#"
---
layout: doc
outline: deep
lang: en-US
---

::: warning
**Note:** This page is under construction 🚧
:::

# {function_name_upper_case}

## Parameters

## Overview

## Examples

[See this example in IronCalc](https://app.ironcalc.com/?filename={function_name})

## Links

                        "#
                        )
                        .trim(),
                    )?;

                    // Add the item to file_items
                    let item = FunctionItem {
                        text: function_name_upper_case,
                        link: format!("/functions/{}/{}", category, function_name),
                    };
                    file_items.push(item);
                }
                category_items.push(CategoryItem {
                    text: category.to_string(),
                    collapsed: true,
                    link: format!("/functions/{}", category),
                    items: file_items,
                });
            }
        }
    }

    let root_item = Item {
        text: "Functions".to_string(),
        collapsed: true,
        items: category_items,
    };

    // Serialize root_item to JSON and write to functions.json
    let json_string = serde_json::to_string_pretty(&root_item)?;
    fs::write("functions.json", json_string)?;
    Ok(())
}