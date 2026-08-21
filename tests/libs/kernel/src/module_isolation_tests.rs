//! Guards the isolation between the modules the kernel context hosts.
//!
//! Each module owns one aggregate and must stay extractable into its own crate:
//! it may depend on the shared crates, never on a sibling module. Modules relate
//! by shared identifier (each declares its own value object for a foreign id)
//! and communicate only through the command, query and event buses.

use std::fs;
use std::path::{Path, PathBuf};

const MODULES: [&str; 4] = ["ioc", "sighting", "source", "url_source"];

fn rust_files(directory: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(directory).unwrap_or_else(|e| panic!("{}: {e}", directory.display()));

    for entry in entries {
        let path = entry.expect("readable directory entry").path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }

    files
}

#[test]
fn no_module_reaches_into_a_sibling_module() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut violations = Vec::new();

    for module in MODULES {
        let module_root = source_root.join(module);
        assert!(module_root.is_dir(), "{module} is not a module of the kernel crate");

        for file in rust_files(&module_root) {
            let contents = fs::read_to_string(&file).expect("readable source file");

            for (index, line) in contents.lines().enumerate() {
                for sibling in MODULES.iter().filter(|candidate| **candidate != module) {
                    if line.contains(&format!("crate::{sibling}::")) {
                        violations.push(format!(
                            "{}:{}: {module} reaches into {sibling}",
                            file.display(),
                            index + 1
                        ));
                    }
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "kernel modules must stay independently extractable, so none may import another. \
         Relate them by shared identifier and talk over the buses instead:\n{}",
        violations.join("\n")
    );
}

#[test]
fn every_module_carries_the_three_layers() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for module in MODULES {
        for layer in ["domain", "application", "infrastructure"] {
            assert!(
                source_root.join(module).join(layer).is_dir(),
                "{module} is missing its {layer} layer, so it could not be extracted as it stands"
            );
        }
    }
}
