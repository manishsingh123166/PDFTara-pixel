//! `files::save_bytes` is the capability-scoped encoder: a linked path inside the document must
//! not be read through ambient filesystem authority and embedded in the returned bytes.

use PDFTara pixel_automation::files;
use PDFTara pixel_doc::{Document, Layer, LayerContent, SmartContentsId, SmartObject, SmartSource};

#[test]
fn a_linked_smart_object_is_not_read_into_the_bytes() {
    let mut doc = Document::new("linked", PDFTara pixel_doc::Size::new(8, 6), PDFTara pixel_doc::ColorMode::Rgb, PDFTara pixel_doc::SampleType::U8);
    let linked = std::env::current_exe().unwrap();
    let smart = SmartObject {
        contents_id: SmartContentsId::fresh(),
        source: SmartSource::Linked { path: linked.to_string_lossy().into_owned() },
        transform: PDFTara pixel_geom::Affine::IDENTITY,
        smart_filters: vec![],
        cache: Some(PDFTara pixel_raster::Surface::new(doc.pixel_format())),
        psd_raw: None,
        filters_enabled: true,
        filter_mask: None,
        warp: None,
        stack_mode: None,
        perspective: None,
        vector_preview: None,
    };
    doc.layers.push(Layer::new("Linked", LayerContent::Smart(smart)));
    let (bytes, warnings) = files::save_bytes(&doc, "out.psd", None, &Default::default()).unwrap();
    assert!(warnings.iter().any(|w| w.contains("not read for this export")), "{warnings:?}");
    assert!(bytes.len() < 1 << 20, "the test executable was not embedded");
}
