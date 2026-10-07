fn main() {
    // macOS Python extensions resolve Python symbols from the loading interpreter.
    // PyO3 scopes these linker arguments to cdylib, leaving server binaries strict.
    pyo3_build_config::add_extension_module_link_args();
}
