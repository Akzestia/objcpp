use zed::LanguageServerId;
use zed_extension_api::{self as zed, Command, Result};

struct ObjcLspExtension;

impl zed::Extension for ObjcLspExtension {
    fn new() -> Self {
        Self
    }

    // language_server_command LICENSE
    /*
     * MIT License
     *
     * Copyright (c) 2025 blacktop
     *
     * Permission is hereby granted, free of charge, to any person obtaining a copy
     * of this software and associated documentation files (the "Software"), to deal
     * in the Software without restriction, including without limitation the rights
     * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     * copies of the Software, and to permit persons to whom the Software is
     * furnished to do so, subject to the following conditions:
     *
     * The above copyright notice and this permission notice shall be included in all
     * copies or substantial portions of the Software.
     *
     * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
     * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
     * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
     * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
     * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
     * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
     * SOFTWARE.
     */
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Command> {
        let path = worktree
            .which("clangd")
            .ok_or_else(|| "clangd must be installed and available in PATH".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![
                // Use compile_commands.json if available
                "--compile-commands-dir".to_string(),
                worktree.root_path(),
                // Enable background indexing for better performance
                "--background-index".to_string(),
                // Reduce log verbosity
                "--log=error".to_string(),
                // Enable header insertion with include-what-you-use style
                "--header-insertion=iwyu".to_string(),
                // Use #import instead of #include for Objective-C (important for ObjC!)
                "--import-insertions".to_string(),
                // Enable clang-tidy checks
                "--clang-tidy".to_string(),
                // Enable completion edits near cursor (dot-to-arrow conversion)
                "--completion-style=detailed".to_string(),
                // Set number of workers for background index
                "-j=4".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ObjcLspExtension);
