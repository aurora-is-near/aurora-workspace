use std::{path::PathBuf, str::FromStr};

pub async fn compile_project(project_path: &str) -> PathBuf {
    let path = std::path::Path::new(project_path).join("Cargo.toml");
    let artifact = cargo_near_build::build(cargo_near_build::BuildOpts {
        manifest_path: Some(
            cargo_near_build::camino::Utf8PathBuf::from_str(path.to_str().unwrap())
                .expect("camino PathBuf from str"),
        ),
        no_abi: true,
        no_locked: true,
        ..Default::default()
    })
    .unwrap();

    artifact.path.into_std_path_buf()
}
