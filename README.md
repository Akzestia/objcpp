# Objective C++

> [!IMPORTANT]
> Zed's [ToS](https://zed.dev/terms#21-eligibility) starting from March 2, 2026 are complete Cancer, I strongly suggest you to switch back to using NeoVim ^^


> [!IMPORTANT]
> ## Credits
> Most of the queries used were taken from the following repo [zed-objc](https://github.com/blacktop/zed-objc) by [@blacktop](https://github.com/blacktop) </br>
> 
> In particular:
> - brackets.scm
> - highlights.scm
> - imports.scm
> - indents.scm
> - injections.scm
> - runnables.scm
> - textobjects.scm
> -----------------------
> Objective-C grammar [repo](https://github.com/tree-sitter-grammars/tree-sitter-objc) by [tree-sitter](https://github.com/tree-sitter)

Support for Objective-C/C++ in Zed

## Setup

Since Zed has a built-in `clangd` language server, you need to configure your settings to use `objcpp-clangd` for Objective-C/C++ files. Add the following to your Zed `settings.json`:

```json
{
  "languages": {
    "Objective-C": {
      "language_servers": ["objcpp-clangd", "!clangd"]
    },
    "Objective-C++": {
      "language_servers": ["objcpp-clangd", "!clangd"]
    }
  }
}
```

If Zed doesn't recognize `.m`/`.mm` files as Objective-C, you may also need:

```json
{
  "file_types": {
    "Objective-C": ["m", "mm"]
  }
}
```

This enables `objcpp-clangd` (provided by this extension) and disables the built-in `clangd` for Objective-C/C++ files.

<img width="3440" height="1431" alt="image" src="https://github.com/user-attachments/assets/11e12f7d-9785-404a-bf9c-e1c618b8ee19" />

-----------------------

> [!NOTE]
> If you have any issues with the extension please open an issue on gihtub!
