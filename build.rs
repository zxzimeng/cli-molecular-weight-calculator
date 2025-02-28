use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    let target_path = PathBuf::from(&manifest_dir)
        .join("target")
        .join(&profile)  // Added & to borrow instead of move
        .join("cmolec2");
    
    let dest_path = PathBuf::from(&manifest_dir).join("cmolec2");
    
    if profile == "release" {
        println!("cargo:warning=Copying from {:?} to {:?}", target_path, dest_path);
        fs::copy(&target_path, &dest_path).expect("Failed to copy executable");
        
        // Make the file executable (rwxr-xr-x)
        let metadata = fs::metadata(&dest_path).unwrap();
        let mut perms = metadata.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms).unwrap();
    }
}