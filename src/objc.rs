use serde::Deserialize;
use zed::LanguageServerId;
use zed_extension_api::{
    self as zed, Command, DebugAdapterBinary, DebugRequest, DebugScenario, DebugTaskDefinition,
    Result, StartDebuggingRequestArgumentsRequest, TaskTemplate, Worktree, serde_json::Value,
};

#[derive(Debug, Deserialize)]
struct ObjcLspExtension {
    cached_binary_path: Option<String>,
}

impl ObjcLspExtension {
    fn get_path_to_language_server_executable(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            return Ok(path.clone());
        }

        let binary_name = "clangd";

        if let Some(path) = worktree.which(&binary_name) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path.clone());
        }

        Err(
            "clangd binary wasn't found in $PATH. Make sure cqlls is installed on your system.\nCheck the install instructions on Github for further information: https://github.com/Akzestia/cqlls"
                .to_owned(),
        )
    }
}

impl zed::Extension for ObjcLspExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Command> {
        Ok(zed::Command {
            command: self.get_path_to_language_server_executable(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        _worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        todo!()
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        todo!()
    }

    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<DebugScenario> {
        todo!()
    }

    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
    ) -> Result<DebugRequest, String> {
        todo!()
    }
}

zed::register_extension!(ObjcLspExtension);
