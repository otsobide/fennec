// `sqlx::migrate!` embeds the migrations at compile time, and Cargo does not
// rebuild the crate when only a migration file changes unless told so.
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
