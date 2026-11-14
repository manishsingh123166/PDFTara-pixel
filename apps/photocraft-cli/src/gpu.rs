//! GPU acceleration policy and options for PDFTara Pixel-cli.

use PDFTara Pixel_automation::Headless;
use PDFTara Pixel_io::BandRenderer;

/// Optional GPU composition for supported flat exports and full-size previews.
#[derive(clap::Args)]
pub(crate) struct GpuArgs {
    /// Prefer GPU rendering, falling back to CPU when unavailable or unsupported
    #[arg(long, conflicts_with = "no_gpu")]
    pub(crate) gpu: bool,
    /// Use CPU rendering, overriding PDFTara Pixel_GPU
    #[arg(long, conflicts_with = "gpu")]
    pub(crate) no_gpu: bool,
}

/// Resolved GPU rendering policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GpuPolicy {
    pub(crate) enabled: bool,
}

impl GpuPolicy {
    /// Resolve policy from arguments and environment.
    ///
    /// Precedence:
    /// - Explicit `--gpu` enables GPU.
    /// - Explicit `--no-gpu` disables GPU.
    /// - Neither flag: `PDFTara Pixel_GPU` env var (1 = enabled, 0 or empty/unset = disabled).
    ///   Any other nonempty value is rejected.
    pub(crate) fn from_args(args: &GpuArgs) -> Result<Self, String> {
        if args.gpu {
            return Ok(Self { enabled: true });
        }
        if args.no_gpu {
            return Ok(Self { enabled: false });
        }
        match std::env::var_os("PDFTara Pixel_GPU") {
            Some(val) => {
                let val = val.to_string_lossy();
                let trimmed = val.trim();
                if trimmed.is_empty() || trimmed == "0" {
                    Ok(Self { enabled: false })
                } else if trimmed == "1" {
                    Ok(Self { enabled: true })
                } else {
                    Err(format!("invalid value '{trimmed}' for 'PDFTara Pixel_GPU': expected '1' or '0'"))
                }
            }
            None => Ok(Self { enabled: false }),
        }
    }

    /// Apply policy to a headless automation session.
    pub(crate) fn apply_to_headless(&self, h: &mut Headless) {
        #[cfg(all(feature = "gpu", not(target_arch = "wasm32")))]
        if self.enabled {
            h.enable_gpu();
        }
        #[cfg(any(not(feature = "gpu"), target_arch = "wasm32"))]
        if self.enabled {
            h.set_renderer(Some(Box::new(DisabledGpuRenderer::new())));
        }
    }

    /// Create an optional boxed renderer under this policy.
    #[cfg(all(feature = "gpu", not(target_arch = "wasm32")))]
    pub(crate) fn create_renderer(&self) -> Option<Box<dyn BandRenderer>> {
        if self.enabled { Some(Box::new(PDFTara Pixel_automation::gpu::GpuRenderer::lazy())) } else { None }
    }

    #[cfg(any(not(feature = "gpu"), target_arch = "wasm32"))]
    pub(crate) fn create_renderer(&self) -> Option<Box<dyn BandRenderer>> {
        if self.enabled { Some(Box::new(DisabledGpuRenderer::new())) } else { None }
    }
}

/// Renderer stub emitted when GPU is requested in builds compiled without `--features gpu`.
#[cfg(any(not(feature = "gpu"), target_arch = "wasm32"))]
struct DisabledGpuRenderer {
    emitted: bool,
}

#[cfg(any(not(feature = "gpu"), target_arch = "wasm32"))]
impl DisabledGpuRenderer {
    fn new() -> Self {
        Self { emitted: false }
    }
}

#[cfg(any(not(feature = "gpu"), target_arch = "wasm32"))]
impl BandRenderer for DisabledGpuRenderer {
    fn render_bands(
        &mut self,
        _doc: &PDFTara Pixel_doc::Document,
        _rect: PDFTara Pixel_doc::Rect,
        _band_rows: i32,
        _ctl: &PDFTara Pixel_raster::Interrupt<'_>,
        _sink: &mut dyn FnMut(PDFTara Pixel_io::RenderBand) -> Result<(), PDFTara Pixel_io::IoError>,
    ) -> Result<(), PDFTara Pixel_io::RenderError> {
        Err(PDFTara Pixel_io::RenderError::Unavailable("GPU support is not compiled in this build (compile with --features gpu)".into()))
    }

    fn drain_diagnostics(&mut self) -> Vec<String> {
        if self.emitted {
            Vec::new()
        } else {
            self.emitted = true;
            vec!["GPU acceleration requested but this build was compiled without GPU support (compile with --features gpu); falling back to CPU".into()]
        }
    }
}
