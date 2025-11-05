use crate::{Element, InternalBuffer, tag::*};

pub struct Template<'a> {
    buffer: &'a mut InternalBuffer,
}

/// Macro to generate template methods for HTML tags
macro_rules! template_method {
    ($fn_name:ident, $tag_type:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $fn_name(&mut self) -> Element<'_, $tag_type> {
            self.el()
        }
    };
}

pub struct CustomTag;

impl<'a> Template<'a> {
    /// Create a new Template from an Html buffer
    pub fn new(html: &'a mut InternalBuffer) -> Self {
        Self { buffer: html }
    }

    fn el<Tag>(&mut self) -> Element<'_, Tag>
    where
        Tag: HtmlTag,
    {
        self.buffer.open(Tag::name(), Tag::is_void());
        Element::new(self.buffer)
    }

    pub fn custom_element(&mut self, name: &str) -> Element<'_, CustomTag> {
        self.buffer.open(name, false);
        Element::new(self.buffer)
    }

    // Document metadata
    template_method!(html, HtmlHtmlTag, "Create an html element");
    template_method!(head, HtmlHeadTag, "Create a head element");
    template_method!(title, HtmlTitleTag, "Create a title element");
    template_method!(base, HtmlBaseTag, "Create a base element");
    template_method!(link, HtmlLinkTag, "Create a link element");
    template_method!(meta, HtmlMetaTag, "Create a meta element");
    template_method!(style, HtmlStyleTag, "Create a style element");

    // Sections
    template_method!(body, HtmlBodyTag, "Create a body element");
    template_method!(article, HtmlArticleTag, "Create an article element");
    template_method!(section, HtmlSectionTag, "Create a section element");
    template_method!(nav, HtmlNavTag, "Create a nav element");
    template_method!(aside, HtmlAsideTag, "Create an aside element");
    template_method!(h1, HtmlH1Tag, "Create an h1 element");
    template_method!(h2, HtmlH2Tag, "Create an h2 element");
    template_method!(h3, HtmlH3Tag, "Create an h3 element");
    template_method!(h4, HtmlH4Tag, "Create an h4 element");
    template_method!(h5, HtmlH5Tag, "Create an h5 element");
    template_method!(h6, HtmlH6Tag, "Create an h6 element");
    template_method!(hgroup, HtmlHgroupTag, "Create an hgroup element");
    template_method!(header, HtmlHeaderTag, "Create a header element");
    template_method!(footer, HtmlFooterTag, "Create a footer element");
    template_method!(address, HtmlAddressTag, "Create an address element");

    // Grouping content
    template_method!(p, HtmlPTag, "Create a p element");
    template_method!(hr, HtmlHrTag, "Create an hr element");
    template_method!(pre, HtmlPreTag, "Create a pre element");
    template_method!(blockquote, HtmlBlockquoteTag, "Create a blockquote element");
    template_method!(ol, HtmlOlTag, "Create an ol element");
    template_method!(ul, HtmlUlTag, "Create an ul element");
    template_method!(li, HtmlLiTag, "Create an li element");
    template_method!(dl, HtmlDlTag, "Create a dl element");
    template_method!(dt, HtmlDtTag, "Create a dt element");
    template_method!(dd, HtmlDdTag, "Create a dd element");
    template_method!(figure, HtmlFigureTag, "Create a figure element");
    template_method!(figcaption, HtmlFigcaptionTag, "Create a figcaption element");
    template_method!(main, HtmlMainTag, "Create a main element");
    template_method!(div, HtmlDivTag, "Create a div element");
    template_method!(search, HtmlSearchTag, "Create a search element");

    // Text-level semantics
    template_method!(a, HtmlATag, "Create an a element");
    template_method!(em, HtmlEmTag, "Create an em element");
    template_method!(strong, HtmlStrongTag, "Create a strong element");
    template_method!(small, HtmlSmallTag, "Create a small element");
    template_method!(s, HtmlSTag, "Create an s element");
    template_method!(cite, HtmlCiteTag, "Create a cite element");
    template_method!(q, HtmlQTag, "Create a q element");
    template_method!(dfn, HtmlDfnTag, "Create a dfn element");
    template_method!(abbr, HtmlAbbrTag, "Create an abbr element");
    template_method!(ruby, HtmlRubyTag, "Create a ruby element");
    template_method!(rt, HtmlRtTag, "Create an rt element");
    template_method!(rp, HtmlRpTag, "Create an rp element");
    template_method!(data, HtmlDataTag, "Create a data element");
    template_method!(time, HtmlTimeTag, "Create a time element");
    template_method!(code, HtmlCodeTag, "Create a code element");
    template_method!(var, HtmlVarTag, "Create a var element");
    template_method!(samp, HtmlSampTag, "Create a samp element");
    template_method!(kbd, HtmlKbdTag, "Create a kbd element");
    template_method!(sub, HtmlSubTag, "Create a sub element");
    template_method!(sup, HtmlSupTag, "Create a sup element");
    template_method!(i, HtmlITag, "Create an i element");
    template_method!(b, HtmlBTag, "Create a b element");
    template_method!(u, HtmlUTag, "Create a u element");
    template_method!(mark, HtmlMarkTag, "Create a mark element");
    template_method!(bdi, HtmlBdiTag, "Create a bdi element");
    template_method!(bdo, HtmlBdoTag, "Create a bdo element");
    template_method!(span, HtmlSpanTag, "Create a span element");
    template_method!(br, HtmlBrTag, "Create a br element");
    template_method!(wbr, HtmlWbrTag, "Create a wbr element");

    // Edits
    template_method!(ins, HtmlInsTag, "Create an ins element");
    template_method!(del, HtmlDelTag, "Create a del element");

    // Embedded content
    template_method!(picture, HtmlPictureTag, "Create a picture element");
    template_method!(source, HtmlSourceTag, "Create a source element");
    template_method!(img, HtmlImgTag, "Create an img element");
    template_method!(iframe, HtmlIframeTag, "Create an iframe element");
    template_method!(embed, HtmlEmbedTag, "Create an embed element");
    template_method!(object, HtmlObjectTag, "Create an object element");
    template_method!(video, HtmlVideoTag, "Create a video element");
    template_method!(audio, HtmlAudioTag, "Create an audio element");
    template_method!(track, HtmlTrackTag, "Create a track element");
    template_method!(map, HtmlMapTag, "Create a map element");
    template_method!(area, HtmlAreaTag, "Create an area element");
    template_method!(math, HtmlMathTag, "Create a math element");

    // SVG elements
    template_method!(svg, HtmlSvgTag, "Create an svg element");
    template_method!(circle, SvgCircleTag, "Create an SVG circle element");
    template_method!(ellipse, SvgEllipseTag, "Create an SVG ellipse element");
    template_method!(line, SvgLineTag, "Create an SVG line element");
    template_method!(path, SvgPathTag, "Create an SVG path element");
    template_method!(polygon, SvgPolygonTag, "Create an SVG polygon element");
    template_method!(polyline, SvgPolylineTag, "Create an SVG polyline element");
    template_method!(rect, SvgRectTag, "Create an SVG rect element");
    template_method!(g, SvgGTag, "Create an SVG g (group) element");
    template_method!(defs, SvgDefsTag, "Create an SVG defs element");
    template_method!(symbol, SvgSymbolTag, "Create an SVG symbol element");
    template_method!(use_attr, SvgUseTag, "Create an SVG use element");
    template_method!(text, SvgTextTag, "Create an SVG text element");
    template_method!(tspan, SvgTspanTag, "Create an SVG tspan element");
    template_method!(text_path, SvgTextPathTag, "Create an SVG textPath element");
    template_method!(marker, SvgMarkerTag, "Create an SVG marker element");
    template_method!(clip_path, SvgClipPathTag, "Create an SVG clipPath element");
    template_method!(mask, SvgMaskTag, "Create an SVG mask element");
    template_method!(pattern, SvgPatternTag, "Create an SVG pattern element");
    template_method!(
        linear_gradient,
        SvgLinearGradientTag,
        "Create an SVG linearGradient element"
    );
    template_method!(
        radial_gradient,
        SvgRadialGradientTag,
        "Create an SVG radialGradient element"
    );
    template_method!(stop, SvgStopTag, "Create an SVG stop element");
    template_method!(image, SvgImageTag, "Create an SVG image element");
    template_method!(
        foreign_object,
        SvgForeignObjectTag,
        "Create an SVG foreignObject element"
    );
    template_method!(animate, SvgAnimateTag, "Create an SVG animate element");
    template_method!(
        animate_transform,
        SvgAnimateTransformTag,
        "Create an SVG animateTransform element"
    );
    template_method!(set, SvgSetTag, "Create an SVG set element");
    template_method!(filter, SvgFilterTag, "Create an SVG filter element");
    template_method!(fe_blend, SvgFeBlendTag, "Create an SVG feBlend element");
    template_method!(
        fe_color_matrix,
        SvgFeColorMatrixTag,
        "Create an SVG feColorMatrix element"
    );
    template_method!(
        fe_component_transfer,
        SvgFeComponentTransferTag,
        "Create an SVG feComponentTransfer element"
    );
    template_method!(
        fe_composite,
        SvgFeCompositeTag,
        "Create an SVG feComposite element"
    );
    template_method!(
        fe_convolve_matrix,
        SvgFeConvolveMatrixTag,
        "Create an SVG feConvolveMatrix element"
    );
    template_method!(
        fe_diffuse_lighting,
        SvgFeDiffuseLightingTag,
        "Create an SVG feDiffuseLighting element"
    );
    template_method!(
        fe_displacement_map,
        SvgFeDisplacementMapTag,
        "Create an SVG feDisplacementMap element"
    );
    template_method!(
        fe_distant_light,
        SvgFeDistantLightTag,
        "Create an SVG feDistantLight element"
    );
    template_method!(
        fe_drop_shadow,
        SvgFeDropShadowTag,
        "Create an SVG feDropShadow element"
    );
    template_method!(fe_flood, SvgFeFloodTag, "Create an SVG feFlood element");
    template_method!(fe_func_a, SvgFeFuncATag, "Create an SVG feFuncA element");
    template_method!(fe_func_b, SvgFeFuncBTag, "Create an SVG feFuncB element");
    template_method!(fe_func_g, SvgFeFuncGTag, "Create an SVG feFuncG element");
    template_method!(fe_func_r, SvgFeFuncRTag, "Create an SVG feFuncR element");
    template_method!(
        fe_gaussian_blur,
        SvgFeGaussianBlurTag,
        "Create an SVG feGaussianBlur element"
    );
    template_method!(fe_image, SvgFeImageTag, "Create an SVG feImage element");
    template_method!(fe_merge, SvgFeMergeTag, "Create an SVG feMerge element");
    template_method!(
        fe_merge_node,
        SvgFeMergeNodeTag,
        "Create an SVG feMergeNode element"
    );
    template_method!(
        fe_morphology,
        SvgFeMorphologyTag,
        "Create an SVG feMorphology element"
    );
    template_method!(fe_offset, SvgFeOffsetTag, "Create an SVG feOffset element");
    template_method!(
        fe_point_light,
        SvgFePointLightTag,
        "Create an SVG fePointLight element"
    );
    template_method!(
        fe_specular_lighting,
        SvgFeSpecularLightingTag,
        "Create an SVG feSpecularLighting element"
    );
    template_method!(
        fe_spot_light,
        SvgFeSpotLightTag,
        "Create an SVG feSpotLight element"
    );
    template_method!(fe_tile, SvgFeTileTag, "Create an SVG feTile element");
    template_method!(
        fe_turbulence,
        SvgFeTurbulenceTag,
        "Create an SVG feTurbulence element"
    );
    template_method!(svg_a, SvgATag, "Create an SVG a (anchor) element");
    template_method!(svg_script, SvgScriptTag, "Create an SVG script element");
    template_method!(svg_style, SvgStyleTag, "Create an SVG style element");
    template_method!(svg_title, SvgTitleTag, "Create an SVG title element");
    template_method!(desc, SvgDescTag, "Create an SVG desc element");
    template_method!(metadata, SvgMetadataTag, "Create an SVG metadata element");

    // Tabular data
    template_method!(table, HtmlTableTag, "Create a table element");
    template_method!(caption, HtmlCaptionTag, "Create a caption element");
    template_method!(colgroup, HtmlColgroupTag, "Create a colgroup element");
    template_method!(col, HtmlColTag, "Create a col element");
    template_method!(tbody, HtmlTbodyTag, "Create a tbody element");
    template_method!(thead, HtmlTheadTag, "Create a thead element");
    template_method!(tfoot, HtmlTfootTag, "Create a tfoot element");
    template_method!(tr, HtmlTrTag, "Create a tr element");
    template_method!(td, HtmlTdTag, "Create a td element");
    template_method!(th, HtmlThTag, "Create a th element");

    // Forms
    template_method!(form, HtmlFormTag, "Create a form element");
    template_method!(label, HtmlLabelTag, "Create a label element");
    template_method!(input, HtmlInputTag, "Create an input element");
    template_method!(button, HtmlButtonTag, "Create a button element");
    template_method!(select, HtmlSelectTag, "Create a select element");
    template_method!(datalist, HtmlDatalistTag, "Create a datalist element");
    template_method!(optgroup, HtmlOptgroupTag, "Create an optgroup element");
    template_method!(option, HtmlOptionTag, "Create an option element");
    template_method!(textarea, HtmlTextareaTag, "Create a textarea element");
    template_method!(output, HtmlOutputTag, "Create an output element");
    template_method!(progress, HtmlProgressTag, "Create a progress element");
    template_method!(meter, HtmlMeterTag, "Create a meter element");
    template_method!(fieldset, HtmlFieldsetTag, "Create a fieldset element");
    template_method!(legend, HtmlLegendTag, "Create a legend element");

    // Interactive elements
    template_method!(details, HtmlDetailsTag, "Create a details element");
    template_method!(summary, HtmlSummaryTag, "Create a summary element");
    template_method!(dialog, HtmlDialogTag, "Create a dialog element");

    // Scripting
    template_method!(script, HtmlScriptTag, "Create a script element");
    template_method!(noscript, HtmlNoscriptTag, "Create a noscript element");
    template_method!(template, HtmlTemplateTag, "Create a template element");
    template_method!(slot, HtmlSlotTag, "Create a slot element");
    template_method!(canvas, HtmlCanvasTag, "Create a canvas element");

    // Other
    template_method!(
        selectedcontent,
        HtmlSelectedcontentTag,
        "Create a selectedcontent element"
    );
}
