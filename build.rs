use anyhow::Result;
use vergen_gix::{
    BuildBuilder, Emitter, GixBuilder,
};
// generated by `sqlx migrate build-script`
fn main() -> Result<()> {
    // trigger recompilation when a new migration is added
    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&GixBuilder::all_git()?)?
        .emit()?;
    println!("cargo:rerun-if-changed=migrations");
    Ok(())
}
