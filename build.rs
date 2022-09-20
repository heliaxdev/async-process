fn main() {
    let cfg = match autocfg::AutoCfg::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            println!(
                "cargo:warning=async-process: failed to detect compiler features: {}",
                e
            );
            return;
        }
    };

    let is_nightly = rustversion::cfg!(nightly);
    if !cfg.probe_rustc_version(1, 63) || is_nightly {
        autocfg::emit("async_process_no_io_safety");
    }
}
