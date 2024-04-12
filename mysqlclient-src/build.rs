use cmake;

fn main() {
    let openssl_dir = std::env::var("DEP_OPENSSL_ROOT").unwrap();
    let mut dst = cmake::Config::new("source")
        .define("WITHOUT_SERVER", "ON")
        .define("WITH_SSL", openssl_dir)
        .build_target("mysqlclient")
        .build();
    dst.push("build");
    dst.push("archive_output_directory");
    println!("cargo::rustc-link-search=native={}", dst.display());
    for entry in std::fs::read_dir(&dst).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let extension = path.extension().and_then(|e| e.to_str());
        if extension == Some("a") || extension == Some("lib") {
            let lib_name = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_start_matches("lib");
            println!("cargo::rustc-link-lib=static={lib_name}");
        }
    }
}
