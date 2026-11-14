//! Windows only: embed the app icon and version info (VERSIONINFO) into `PDFTara Pixel.exe`.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `PDFTara Pixel_REQUIRE_WINRES=1` (set by the
//! release workflow) turns it into an error.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/PDFTara Pixel.ico");
    println!("cargo:rerun-if-env-changed=PDFTara Pixel_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    // Printing to PDF (winprint's `PdfCreateRenderer`) imports Windows.Data.Pdf.dll, which
    // stripped Windows images (NTLite, some N/LTSC builds) lack. A load-time import stops the exe
    // from starting at all ("windows.data.pdf.dll was not found", #5884), so load it on first use;
    // the print command checks for it first and reports an error instead.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bins=/DELAYLOAD:windows.data.pdf.dll");
        println!("cargo:rustc-link-arg-bins=delayimp.lib");
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/PDFTara Pixel.ico")
        .set("ProductName", "PDFTara Pixel")
        .set("FileDescription", "PDFTara Pixel image editor")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) the PDFTara Pixel authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "PDFTara Pixel.exe")
        .set("InternalName", "PDFTara Pixel");
    if let Err(e) = res.compile() {
        if std::env::var_os("PDFTara Pixel_REQUIRE_WINRES").is_some() {
            panic!("embedding Windows resources failed: {e}");
        }
        println!("cargo:warning=PDFTara Pixel.exe built without icon/version resources: {e}");
    }
}
