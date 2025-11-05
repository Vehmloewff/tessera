pub trait HtmlTag {
    fn name() -> &'static str;
    fn is_void() -> bool;
}

/// Macro to simplify creation of HTML tag structs and trait implementations
macro_rules! html_tag {
    ($struct_name:ident, $tag_name:expr, void) => {
        pub struct $struct_name;
        impl HtmlTag for $struct_name {
            fn name() -> &'static str {
                $tag_name
            }
            fn is_void() -> bool {
                true
            }
        }
    };
    ($struct_name:ident, $tag_name:expr) => {
        pub struct $struct_name;
        impl HtmlTag for $struct_name {
            fn name() -> &'static str {
                $tag_name
            }
            fn is_void() -> bool {
                false
            }
        }
    };
}

// Document metadata
html_tag!(HtmlHtmlTag, "html");
html_tag!(HtmlHeadTag, "head");
html_tag!(HtmlTitleTag, "title");
html_tag!(HtmlBaseTag, "base", void);
html_tag!(HtmlLinkTag, "link", void);
html_tag!(HtmlMetaTag, "meta", void);
html_tag!(HtmlStyleTag, "style");

// Sections
html_tag!(HtmlBodyTag, "body");
html_tag!(HtmlArticleTag, "article");
html_tag!(HtmlSectionTag, "section");
html_tag!(HtmlNavTag, "nav");
html_tag!(HtmlAsideTag, "aside");
html_tag!(HtmlH1Tag, "h1");
html_tag!(HtmlH2Tag, "h2");
html_tag!(HtmlH3Tag, "h3");
html_tag!(HtmlH4Tag, "h4");
html_tag!(HtmlH5Tag, "h5");
html_tag!(HtmlH6Tag, "h6");
html_tag!(HtmlHgroupTag, "hgroup");
html_tag!(HtmlHeaderTag, "header");
html_tag!(HtmlFooterTag, "footer");
html_tag!(HtmlAddressTag, "address");

// Grouping content
html_tag!(HtmlPTag, "p");
html_tag!(HtmlHrTag, "hr", void);
html_tag!(HtmlPreTag, "pre");
html_tag!(HtmlBlockquoteTag, "blockquote");
html_tag!(HtmlOlTag, "ol");
html_tag!(HtmlUlTag, "ul");
html_tag!(HtmlLiTag, "li");
html_tag!(HtmlDlTag, "dl");
html_tag!(HtmlDtTag, "dt");
html_tag!(HtmlDdTag, "dd");
html_tag!(HtmlFigureTag, "figure");
html_tag!(HtmlFigcaptionTag, "figcaption");
html_tag!(HtmlMainTag, "main");
html_tag!(HtmlDivTag, "div");
html_tag!(HtmlSearchTag, "search");

// Text-level semantics
html_tag!(HtmlATag, "a");
html_tag!(HtmlEmTag, "em");
html_tag!(HtmlStrongTag, "strong");
html_tag!(HtmlSmallTag, "small");
html_tag!(HtmlSTag, "s");
html_tag!(HtmlCiteTag, "cite");
html_tag!(HtmlQTag, "q");
html_tag!(HtmlDfnTag, "dfn");
html_tag!(HtmlAbbrTag, "abbr");
html_tag!(HtmlRubyTag, "ruby");
html_tag!(HtmlRtTag, "rt");
html_tag!(HtmlRpTag, "rp");
html_tag!(HtmlDataTag, "data");
html_tag!(HtmlTimeTag, "time");
html_tag!(HtmlCodeTag, "code");
html_tag!(HtmlVarTag, "var");
html_tag!(HtmlSampTag, "samp");
html_tag!(HtmlKbdTag, "kbd");
html_tag!(HtmlSubTag, "sub");
html_tag!(HtmlSupTag, "sup");
html_tag!(HtmlITag, "i");
html_tag!(HtmlBTag, "b");
html_tag!(HtmlUTag, "u");
html_tag!(HtmlMarkTag, "mark");
html_tag!(HtmlBdiTag, "bdi");
html_tag!(HtmlBdoTag, "bdo");
html_tag!(HtmlSpanTag, "span");
html_tag!(HtmlBrTag, "br", void);
html_tag!(HtmlWbrTag, "wbr", void);

// Edits
html_tag!(HtmlInsTag, "ins");
html_tag!(HtmlDelTag, "del");

// Embedded content
html_tag!(HtmlPictureTag, "picture");
html_tag!(HtmlSourceTag, "source", void);
html_tag!(HtmlImgTag, "img", void);
html_tag!(HtmlIframeTag, "iframe");
html_tag!(HtmlEmbedTag, "embed", void);
html_tag!(HtmlObjectTag, "object");
html_tag!(HtmlVideoTag, "video");
html_tag!(HtmlAudioTag, "audio");
html_tag!(HtmlTrackTag, "track", void);
html_tag!(HtmlMapTag, "map");
html_tag!(HtmlAreaTag, "area", void);
html_tag!(HtmlMathTag, "math");
html_tag!(HtmlSvgTag, "svg");

// SVG elements
html_tag!(SvgCircleTag, "circle");
html_tag!(SvgEllipseTag, "ellipse");
html_tag!(SvgLineTag, "line");
html_tag!(SvgPathTag, "path");
html_tag!(SvgPolygonTag, "polygon");
html_tag!(SvgPolylineTag, "polyline");
html_tag!(SvgRectTag, "rect");
html_tag!(SvgGTag, "g");
html_tag!(SvgDefsTag, "defs");
html_tag!(SvgSymbolTag, "symbol");
html_tag!(SvgUseTag, "use");
html_tag!(SvgTextTag, "text");
html_tag!(SvgTspanTag, "tspan");
html_tag!(SvgTextPathTag, "textPath");
html_tag!(SvgMarkerTag, "marker");
html_tag!(SvgClipPathTag, "clipPath");
html_tag!(SvgMaskTag, "mask");
html_tag!(SvgPatternTag, "pattern");
html_tag!(SvgLinearGradientTag, "linearGradient");
html_tag!(SvgRadialGradientTag, "radialGradient");
html_tag!(SvgStopTag, "stop");
html_tag!(SvgImageTag, "image");
html_tag!(SvgForeignObjectTag, "foreignObject");
html_tag!(SvgAnimateTag, "animate");
html_tag!(SvgAnimateTransformTag, "animateTransform");
html_tag!(SvgSetTag, "set");
html_tag!(SvgFilterTag, "filter");
html_tag!(SvgFeBlendTag, "feBlend");
html_tag!(SvgFeColorMatrixTag, "feColorMatrix");
html_tag!(SvgFeComponentTransferTag, "feComponentTransfer");
html_tag!(SvgFeCompositeTag, "feComposite");
html_tag!(SvgFeConvolveMatrixTag, "feConvolveMatrix");
html_tag!(SvgFeDiffuseLightingTag, "feDiffuseLighting");
html_tag!(SvgFeDisplacementMapTag, "feDisplacementMap");
html_tag!(SvgFeDistantLightTag, "feDistantLight");
html_tag!(SvgFeDropShadowTag, "feDropShadow");
html_tag!(SvgFeFloodTag, "feFlood");
html_tag!(SvgFeFuncATag, "feFuncA");
html_tag!(SvgFeFuncBTag, "feFuncB");
html_tag!(SvgFeFuncGTag, "feFuncG");
html_tag!(SvgFeFuncRTag, "feFuncR");
html_tag!(SvgFeGaussianBlurTag, "feGaussianBlur");
html_tag!(SvgFeImageTag, "feImage");
html_tag!(SvgFeMergeTag, "feMerge");
html_tag!(SvgFeMergeNodeTag, "feMergeNode");
html_tag!(SvgFeMorphologyTag, "feMorphology");
html_tag!(SvgFeOffsetTag, "feOffset");
html_tag!(SvgFePointLightTag, "fePointLight");
html_tag!(SvgFeSpecularLightingTag, "feSpecularLighting");
html_tag!(SvgFeSpotLightTag, "feSpotLight");
html_tag!(SvgFeTileTag, "feTile");
html_tag!(SvgFeTurbulenceTag, "feTurbulence");
html_tag!(SvgATag, "a");
html_tag!(SvgScriptTag, "script");
html_tag!(SvgStyleTag, "style");
html_tag!(SvgTitleTag, "title");
html_tag!(SvgDescTag, "desc");
html_tag!(SvgMetadataTag, "metadata");

// Tabular data
html_tag!(HtmlTableTag, "table");
html_tag!(HtmlCaptionTag, "caption");
html_tag!(HtmlColgroupTag, "colgroup");
html_tag!(HtmlColTag, "col", void);
html_tag!(HtmlTbodyTag, "tbody");
html_tag!(HtmlTheadTag, "thead");
html_tag!(HtmlTfootTag, "tfoot");
html_tag!(HtmlTrTag, "tr");
html_tag!(HtmlTdTag, "td");
html_tag!(HtmlThTag, "th");

// Forms
html_tag!(HtmlFormTag, "form");
html_tag!(HtmlLabelTag, "label");
html_tag!(HtmlInputTag, "input", void);
html_tag!(HtmlButtonTag, "button");
html_tag!(HtmlSelectTag, "select");
html_tag!(HtmlDatalistTag, "datalist");
html_tag!(HtmlOptgroupTag, "optgroup");
html_tag!(HtmlOptionTag, "option");
html_tag!(HtmlTextareaTag, "textarea");
html_tag!(HtmlOutputTag, "output");
html_tag!(HtmlProgressTag, "progress");
html_tag!(HtmlMeterTag, "meter");
html_tag!(HtmlFieldsetTag, "fieldset");
html_tag!(HtmlLegendTag, "legend");

// Interactive elements
html_tag!(HtmlDetailsTag, "details");
html_tag!(HtmlSummaryTag, "summary");
html_tag!(HtmlDialogTag, "dialog");

// Scripting
html_tag!(HtmlScriptTag, "script");
html_tag!(HtmlNoscriptTag, "noscript");
html_tag!(HtmlTemplateTag, "template");
html_tag!(HtmlSlotTag, "slot");
html_tag!(HtmlCanvasTag, "canvas");

// Other
html_tag!(HtmlSelectedcontentTag, "selectedcontent");
