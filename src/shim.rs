fn main() {
    let mut args = vec!["run".to_string()];
    let env_args = std::env::args().collect::<Vec<_>>();

    let shim_path = env_args.get(0).unwrap();
    let shim_path = std::path::Path::new(shim_path);
    // Use the shim's name as alias name.
    let alias_name = shim_path.file_stem().unwrap().to_str().unwrap();

    args.push(alias_name.into());
    args.extend(env_args.into_iter().skip(1));

    std::process::Command::new("as")
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
