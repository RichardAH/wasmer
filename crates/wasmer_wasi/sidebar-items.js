initSidebarItems({"enum":[["WasiVersion","The version of WASI. This is determined by the imports namespace string."]],"fn":[["generate_import_object","Creates a Wasi [`ImportObject`] with [`WasiState`] with the latest snapshot of WASI."],["generate_import_object_for_version","Creates a Wasi [`ImportObject`] with [`WasiState`] for the given [`WasiVersion`]."],["generate_import_object_from_state","Create an [`ImportObject`] with an existing [`WasiState`]. [`WasiState`] can be constructed from a `WasiStateBuilder`."],["generate_import_object_snapshot0","Creates a legacy Wasi [`ImportObject`] with [`WasiState`]."],["generate_import_object_snapshot0_inner","Combines a state generating function with the import list for legacy WASI"],["generate_import_object_snapshot1_inner","Combines a state generating function with the import list for snapshot 1"],["get_wasi_version","Detect the version of WASI being used based on the import namespaces."],["is_wasi_module","Check if a provided module is compiled for some version of WASI. Use [`get_wasi_version`] to find out which version of WASI the module is."]],"mod":[["macros","Macros to simplify some common WASI-specific tasks."],["ptr","This is a wrapper around the `WasmPtr` abstraction that returns __WASI_EFAULT if memory access failed"],["state","WARNING: the API exposed here is unstable and very experimental.  Certain things are not ready yet and may be broken in patch releases.  If you're using this and have any specific needs, please let us know here https://github.com/wasmerio/wasmer/issues/583 or by filing an issue."],["syscalls",""],["types",""],["utils",""]],"struct":[["ExitCode","This is returned in `RuntimeError`. Use `downcast` or `downcast_ref` to retrieve the `ExitCode`."]]});