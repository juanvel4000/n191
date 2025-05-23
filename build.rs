// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

fn main() {
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        winres::WindowsResource::new()
            .set_manifest_file("src/bin/n191/n191.exe.manifest")
            .set("FileDescription", "n191")
            .set("LegalCopyright", "© Microsoft Corporation. All rights reserved. © juanvel400 for n191")
            .compile()
            .unwrap();
    }
}
