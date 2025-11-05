use crate::{Element, element::ElementAttributor, tag::*};

// ============================================================================
// Global Attributes (can be used on ANY HTML element)
// ============================================================================

pub trait GlobalAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accesskey(self, value: &str) -> Self {
        self.attr("accesskey", value)
    }

    fn autocapitalize(self, value: &str) -> Self {
        self.attr("autocapitalize", value)
    }

    fn autofocus(self, value: &str) -> Self {
        self.attr("autofocus", value)
    }

    fn contenteditable(self, value: &str) -> Self {
        self.attr("contenteditable", value)
    }

    fn dir(self, value: &str) -> Self {
        self.attr("dir", value)
    }

    fn draggable(self, value: &str) -> Self {
        self.attr("draggable", value)
    }

    fn enterkeyhint(self, value: &str) -> Self {
        self.attr("enterkeyhint", value)
    }

    fn exportparts(self, value: &str) -> Self {
        self.attr("exportparts", value)
    }

    fn hidden(self, value: &str) -> Self {
        self.attr("hidden", value)
    }

    fn id(self, value: &str) -> Self {
        self.attr("id", value)
    }

    fn inert(self, value: &str) -> Self {
        self.attr("inert", value)
    }

    fn inputmode(self, value: &str) -> Self {
        self.attr("inputmode", value)
    }

    fn is(self, value: &str) -> Self {
        self.attr("is", value)
    }

    // Microdata attributes
    fn itemid(self, value: &str) -> Self {
        self.attr("itemid", value)
    }

    fn itemprop(self, value: &str) -> Self {
        self.attr("itemprop", value)
    }

    fn itemref(self, value: &str) -> Self {
        self.attr("itemref", value)
    }

    fn itemscope(self, value: &str) -> Self {
        self.attr("itemscope", value)
    }

    fn itemtype(self, value: &str) -> Self {
        self.attr("itemtype", value)
    }

    fn lang(self, value: &str) -> Self {
        self.attr("lang", value)
    }

    fn nonce(self, value: &str) -> Self {
        self.attr("nonce", value)
    }

    fn part(self, value: &str) -> Self {
        self.attr("part", value)
    }

    fn popover(self, value: &str) -> Self {
        self.attr("popover", value)
    }

    fn role(self, value: &str) -> Self {
        self.attr("role", value)
    }

    fn slot(self, value: &str) -> Self {
        self.attr("slot", value)
    }

    fn spellcheck(self, value: &str) -> Self {
        self.attr("spellcheck", value)
    }

    fn style(self, value: &str) -> Self {
        self.attr("style", value)
    }

    fn tabindex(self, value: &str) -> Self {
        self.attr("tabindex", value)
    }

    fn title(self, value: &str) -> Self {
        self.attr("title", value)
    }

    fn translate(self, value: &str) -> Self {
        self.attr("translate", value)
    }

    fn virtualkeyboardpolicy(self, value: &str) -> Self {
        self.attr("virtualkeyboardpolicy", value)
    }

    fn writingsuggestions(self, value: &str) -> Self {
        self.attr("writingsuggestions", value)
    }
}

impl<Tag> GlobalAttributes for Element<'_, Tag> {}

// ============================================================================
// Form Element Attributes
// ============================================================================

pub trait FormElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accept_charset(self, value: &str) -> Self {
        self.attr("accept-charset", value)
    }

    fn action(self, value: &str) -> Self {
        self.attr("action", value)
    }

    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn enctype(self, value: &str) -> Self {
        self.attr("enctype", value)
    }

    fn method(self, value: &str) -> Self {
        self.attr("method", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn novalidate(self, value: &str) -> Self {
        self.attr("novalidate", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl FormElementAttributes for Element<'_, HtmlFormTag> {}

// ============================================================================
// Input Element Attributes
// ============================================================================

pub trait InputElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn accept(self, value: &str) -> Self {
        self.attr("accept", value)
    }

    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn checked(self, value: &str) -> Self {
        self.attr("checked", value)
    }

    fn dirname(self, value: &str) -> Self {
        self.attr("dirname", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn formaction(self, value: &str) -> Self {
        self.attr("formaction", value)
    }

    fn formenctype(self, value: &str) -> Self {
        self.attr("formenctype", value)
    }

    fn formmethod(self, value: &str) -> Self {
        self.attr("formmethod", value)
    }

    fn formnovalidate(self, value: &str) -> Self {
        self.attr("formnovalidate", value)
    }

    fn formtarget(self, value: &str) -> Self {
        self.attr("formtarget", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn list(self, value: &str) -> Self {
        self.attr("list", value)
    }

    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn maxlength(self, value: &str) -> Self {
        self.attr("maxlength", value)
    }

    fn min(self, value: &str) -> Self {
        self.attr("min", value)
    }

    fn minlength(self, value: &str) -> Self {
        self.attr("minlength", value)
    }

    fn multiple(self, value: &str) -> Self {
        self.attr("multiple", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn pattern(self, value: &str) -> Self {
        self.attr("pattern", value)
    }

    fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }

    fn readonly(self, value: &str) -> Self {
        self.attr("readonly", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn size(self, value: &str) -> Self {
        self.attr("size", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn step(self, value: &str) -> Self {
        self.attr("step", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl InputElementAttributes for Element<'_, HtmlInputTag> {}

// ============================================================================
// Button Element Attributes
// ============================================================================

pub trait ButtonElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn command(self, value: &str) -> Self {
        self.attr("command", value)
    }

    fn commandfor(self, value: &str) -> Self {
        self.attr("commandfor", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn formaction(self, value: &str) -> Self {
        self.attr("formaction", value)
    }

    fn formenctype(self, value: &str) -> Self {
        self.attr("formenctype", value)
    }

    fn formmethod(self, value: &str) -> Self {
        self.attr("formmethod", value)
    }

    fn formnovalidate(self, value: &str) -> Self {
        self.attr("formnovalidate", value)
    }

    fn formtarget(self, value: &str) -> Self {
        self.attr("formtarget", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl ButtonElementAttributes for Element<'_, HtmlButtonTag> {}

// ============================================================================
// Select Element Attributes
// ============================================================================

pub trait SelectElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn multiple(self, value: &str) -> Self {
        self.attr("multiple", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn size(self, value: &str) -> Self {
        self.attr("size", value)
    }
}

impl SelectElementAttributes for Element<'_, HtmlSelectTag> {}

// ============================================================================
// Textarea Element Attributes
// ============================================================================

pub trait TextareaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autocomplete(self, value: &str) -> Self {
        self.attr("autocomplete", value)
    }

    fn cols(self, value: &str) -> Self {
        self.attr("cols", value)
    }

    fn dirname(self, value: &str) -> Self {
        self.attr("dirname", value)
    }

    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn maxlength(self, value: &str) -> Self {
        self.attr("maxlength", value)
    }

    fn minlength(self, value: &str) -> Self {
        self.attr("minlength", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn placeholder(self, value: &str) -> Self {
        self.attr("placeholder", value)
    }

    fn readonly(self, value: &str) -> Self {
        self.attr("readonly", value)
    }

    fn required(self, value: &str) -> Self {
        self.attr("required", value)
    }

    fn rows(self, value: &str) -> Self {
        self.attr("rows", value)
    }

    fn wrap(self, value: &str) -> Self {
        self.attr("wrap", value)
    }
}

impl TextareaElementAttributes for Element<'_, HtmlTextareaTag> {}

// ============================================================================
// Label Element Attributes
// ============================================================================

pub trait LabelElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn for_attr(self, value: &str) -> Self {
        self.attr("for", value)
    }
}

impl LabelElementAttributes for Element<'_, HtmlLabelTag> {}

// ============================================================================
// Option Element Attributes
// ============================================================================

pub trait OptionElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }

    fn selected(self, value: &str) -> Self {
        self.attr("selected", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl OptionElementAttributes for Element<'_, HtmlOptionTag> {}

// ============================================================================
// Optgroup Element Attributes
// ============================================================================

pub trait OptgroupElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }
}

impl OptgroupElementAttributes for Element<'_, HtmlOptgroupTag> {}

// ============================================================================
// Fieldset Element Attributes
// ============================================================================

pub trait FieldsetElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn disabled(self, value: &str) -> Self {
        self.attr("disabled", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl FieldsetElementAttributes for Element<'_, HtmlFieldsetTag> {}

// ============================================================================
// Output Element Attributes
// ============================================================================

pub trait OutputElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn for_attr(self, value: &str) -> Self {
        self.attr("for", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl OutputElementAttributes for Element<'_, HtmlOutputTag> {}

// ============================================================================
// Media Element Attributes (audio, video)
// ============================================================================

pub trait MediaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn autoplay(self, value: &str) -> Self {
        self.attr("autoplay", value)
    }

    fn controls(self, value: &str) -> Self {
        self.attr("controls", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn loop_attr(self, value: &str) -> Self {
        self.attr("loop", value)
    }

    fn muted(self, value: &str) -> Self {
        self.attr("muted", value)
    }

    fn preload(self, value: &str) -> Self {
        self.attr("preload", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }
}

impl MediaElementAttributes for Element<'_, HtmlAudioTag> {}
impl MediaElementAttributes for Element<'_, HtmlVideoTag> {}

// ============================================================================
// Video Element Attributes (video-specific, in addition to media)
// ============================================================================

pub trait VideoElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn playsinline(self, value: &str) -> Self {
        self.attr("playsinline", value)
    }

    fn poster(self, value: &str) -> Self {
        self.attr("poster", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl VideoElementAttributes for Element<'_, HtmlVideoTag> {}

// ============================================================================
// Image Element Attributes
// ============================================================================

pub trait ImageElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn decoding(self, value: &str) -> Self {
        self.attr("decoding", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn ismap(self, value: &str) -> Self {
        self.attr("ismap", value)
    }

    fn loading(self, value: &str) -> Self {
        self.attr("loading", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn sizes(self, value: &str) -> Self {
        self.attr("sizes", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcset(self, value: &str) -> Self {
        self.attr("srcset", value)
    }

    fn usemap(self, value: &str) -> Self {
        self.attr("usemap", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl ImageElementAttributes for Element<'_, HtmlImgTag> {}

// ============================================================================
// Anchor/Link Element Attributes
// ============================================================================

pub trait AnchorElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn download(self, value: &str) -> Self {
        self.attr("download", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn hreflang(self, value: &str) -> Self {
        self.attr("hreflang", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl AnchorElementAttributes for Element<'_, HtmlATag> {}

// ============================================================================
// Link Element Attributes (for <link> tag)
// ============================================================================

pub trait LinkElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn as_attr(self, value: &str) -> Self {
        self.attr("as", value)
    }

    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn hreflang(self, value: &str) -> Self {
        self.attr("hreflang", value)
    }

    fn imagesizes(self, value: &str) -> Self {
        self.attr("imagesizes", value)
    }

    fn imagesrcset(self, value: &str) -> Self {
        self.attr("imagesrcset", value)
    }

    fn integrity(self, value: &str) -> Self {
        self.attr("integrity", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl LinkElementAttributes for Element<'_, HtmlLinkTag> {}

// ============================================================================
// Area Element Attributes
// ============================================================================

pub trait AreaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn alt(self, value: &str) -> Self {
        self.attr("alt", value)
    }

    fn coords(self, value: &str) -> Self {
        self.attr("coords", value)
    }

    fn download(self, value: &str) -> Self {
        self.attr("download", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn rel(self, value: &str) -> Self {
        self.attr("rel", value)
    }

    fn shape(self, value: &str) -> Self {
        self.attr("shape", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl AreaElementAttributes for Element<'_, HtmlAreaTag> {}

// ============================================================================
// Iframe Element Attributes
// ============================================================================

pub trait IframeElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn allow(self, value: &str) -> Self {
        self.attr("allow", value)
    }

    fn allowfullscreen(self, value: &str) -> Self {
        self.attr("allowfullscreen", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn loading(self, value: &str) -> Self {
        self.attr("loading", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn sandbox(self, value: &str) -> Self {
        self.attr("sandbox", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcdoc(self, value: &str) -> Self {
        self.attr("srcdoc", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl IframeElementAttributes for Element<'_, HtmlIframeTag> {}

// ============================================================================
// Embed Element Attributes
// ============================================================================

pub trait EmbedElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl EmbedElementAttributes for Element<'_, HtmlEmbedTag> {}

// ============================================================================
// Object Element Attributes
// ============================================================================

pub trait ObjectElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn data(self, value: &str) -> Self {
        self.attr("data", value)
    }

    fn form(self, value: &str) -> Self {
        self.attr("form", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn usemap(self, value: &str) -> Self {
        self.attr("usemap", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl ObjectElementAttributes for Element<'_, HtmlObjectTag> {}

// ============================================================================
// Source Element Attributes
// ============================================================================

pub trait SourceElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }

    fn sizes(self, value: &str) -> Self {
        self.attr("sizes", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srcset(self, value: &str) -> Self {
        self.attr("srcset", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl SourceElementAttributes for Element<'_, HtmlSourceTag> {}

// ============================================================================
// Track Element Attributes
// ============================================================================

pub trait TrackElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn default(self, value: &str) -> Self {
        self.attr("default", value)
    }

    fn kind(self, value: &str) -> Self {
        self.attr("kind", value)
    }

    fn label(self, value: &str) -> Self {
        self.attr("label", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn srclang(self, value: &str) -> Self {
        self.attr("srclang", value)
    }
}

impl TrackElementAttributes for Element<'_, HtmlTrackTag> {}

// ============================================================================
// Map Element Attributes
// ============================================================================

pub trait MapElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }
}

impl MapElementAttributes for Element<'_, HtmlMapTag> {}

// ============================================================================
// Table Cell Attributes (td, th)
// ============================================================================

pub trait TableCellAttributes
where
    Self: ElementAttributor + Sized,
{
    fn colspan(self, value: &str) -> Self {
        self.attr("colspan", value)
    }

    fn headers(self, value: &str) -> Self {
        self.attr("headers", value)
    }

    fn rowspan(self, value: &str) -> Self {
        self.attr("rowspan", value)
    }
}

impl TableCellAttributes for Element<'_, HtmlTdTag> {}
impl TableCellAttributes for Element<'_, HtmlThTag> {}

// ============================================================================
// Th Element Attributes (in addition to TableCellAttributes)
// ============================================================================

pub trait ThElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn abbr(self, value: &str) -> Self {
        self.attr("abbr", value)
    }

    fn scope(self, value: &str) -> Self {
        self.attr("scope", value)
    }
}

impl ThElementAttributes for Element<'_, HtmlThTag> {}

// ============================================================================
// Col/Colgroup Element Attributes
// ============================================================================

pub trait ColElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn span(self, value: &str) -> Self {
        self.attr("span", value)
    }
}

impl ColElementAttributes for Element<'_, HtmlColTag> {}
impl ColElementAttributes for Element<'_, HtmlColgroupTag> {}

// ============================================================================
// Script Element Attributes
// ============================================================================

pub trait ScriptElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn async_attr(self, value: &str) -> Self {
        self.attr("async", value)
    }

    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn defer(self, value: &str) -> Self {
        self.attr("defer", value)
    }

    fn fetchpriority(self, value: &str) -> Self {
        self.attr("fetchpriority", value)
    }

    fn integrity(self, value: &str) -> Self {
        self.attr("integrity", value)
    }

    fn nomodule(self, value: &str) -> Self {
        self.attr("nomodule", value)
    }

    fn referrerpolicy(self, value: &str) -> Self {
        self.attr("referrerpolicy", value)
    }

    fn src(self, value: &str) -> Self {
        self.attr("src", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl ScriptElementAttributes for Element<'_, HtmlScriptTag> {}

// ============================================================================
// Style Element Attributes
// ============================================================================

pub trait StyleElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn blocking(self, value: &str) -> Self {
        self.attr("blocking", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }
}

impl StyleElementAttributes for Element<'_, HtmlStyleTag> {}

// ============================================================================
// Meta Element Attributes
// ============================================================================

pub trait MetaElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn charset(self, value: &str) -> Self {
        self.attr("charset", value)
    }

    fn content(self, value: &str) -> Self {
        self.attr("content", value)
    }

    fn http_equiv(self, value: &str) -> Self {
        self.attr("http-equiv", value)
    }

    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn media(self, value: &str) -> Self {
        self.attr("media", value)
    }
}

impl MetaElementAttributes for Element<'_, HtmlMetaTag> {}

// ============================================================================
// Base Element Attributes
// ============================================================================

pub trait BaseElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn target(self, value: &str) -> Self {
        self.attr("target", value)
    }
}

impl BaseElementAttributes for Element<'_, HtmlBaseTag> {}

// ============================================================================
// Blockquote/Q Element Attributes
// ============================================================================

pub trait QuoteElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cite(self, value: &str) -> Self {
        self.attr("cite", value)
    }
}

impl QuoteElementAttributes for Element<'_, HtmlBlockquoteTag> {}
impl QuoteElementAttributes for Element<'_, HtmlQTag> {}

// ============================================================================
// Del/Ins Element Attributes
// ============================================================================

pub trait ModElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cite(self, value: &str) -> Self {
        self.attr("cite", value)
    }

    fn datetime(self, value: &str) -> Self {
        self.attr("datetime", value)
    }
}

impl ModElementAttributes for Element<'_, HtmlDelTag> {}
impl ModElementAttributes for Element<'_, HtmlInsTag> {}

// ============================================================================
// Time Element Attributes
// ============================================================================

pub trait TimeElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn datetime(self, value: &str) -> Self {
        self.attr("datetime", value)
    }
}

impl TimeElementAttributes for Element<'_, HtmlTimeTag> {}

// ============================================================================
// Data Element Attributes
// ============================================================================

pub trait DataElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl DataElementAttributes for Element<'_, HtmlDataTag> {}

// ============================================================================
// Meter Element Attributes
// ============================================================================

pub trait MeterElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn high(self, value: &str) -> Self {
        self.attr("high", value)
    }

    fn low(self, value: &str) -> Self {
        self.attr("low", value)
    }

    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn min(self, value: &str) -> Self {
        self.attr("min", value)
    }

    fn optimum(self, value: &str) -> Self {
        self.attr("optimum", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl MeterElementAttributes for Element<'_, HtmlMeterTag> {}

// ============================================================================
// Progress Element Attributes
// ============================================================================

pub trait ProgressElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn max(self, value: &str) -> Self {
        self.attr("max", value)
    }

    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl ProgressElementAttributes for Element<'_, HtmlProgressTag> {}

// ============================================================================
// Details Element Attributes
// ============================================================================

pub trait DetailsElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn name(self, value: &str) -> Self {
        self.attr("name", value)
    }

    fn open(self, value: &str) -> Self {
        self.attr("open", value)
    }
}

impl DetailsElementAttributes for Element<'_, HtmlDetailsTag> {}

// ============================================================================
// Dialog Element Attributes
// ============================================================================

pub trait DialogElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn open(self, value: &str) -> Self {
        self.attr("open", value)
    }
}

impl DialogElementAttributes for Element<'_, HtmlDialogTag> {}

// ============================================================================
// Template Element Attributes
// ============================================================================

pub trait TemplateElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn shadowrootclonable(self, value: &str) -> Self {
        self.attr("shadowrootclonable", value)
    }

    fn shadowrootcustomelementregistry(self, value: &str) -> Self {
        self.attr("shadowrootcustomelementregistry", value)
    }

    fn shadowrootdelegatesfocus(self, value: &str) -> Self {
        self.attr("shadowrootdelegatesfocus", value)
    }

    fn shadowrootmode(self, value: &str) -> Self {
        self.attr("shadowrootmode", value)
    }

    fn shadowrootserializable(self, value: &str) -> Self {
        self.attr("shadowrootserializable", value)
    }
}

impl TemplateElementAttributes for Element<'_, HtmlTemplateTag> {}

// ============================================================================
// Ol Element Attributes
// ============================================================================

pub trait OlElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn reversed(self, value: &str) -> Self {
        self.attr("reversed", value)
    }

    fn start(self, value: &str) -> Self {
        self.attr("start", value)
    }

    fn type_attr(self, value: &str) -> Self {
        self.attr("type", value)
    }
}

impl OlElementAttributes for Element<'_, HtmlOlTag> {}

// ============================================================================
// Li Element Attributes (when used in <ol>)
// ============================================================================

pub trait LiElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn value(self, value: &str) -> Self {
        self.attr("value", value)
    }
}

impl LiElementAttributes for Element<'_, HtmlLiTag> {}

// ============================================================================
// Canvas Element Attributes
// ============================================================================

pub trait CanvasElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }
}

impl CanvasElementAttributes for Element<'_, HtmlCanvasTag> {}

// ============================================================================
// SVG Element Attributes (for <svg> root element)
// ============================================================================

pub trait SvgRootElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn xmlns(self, value: &str) -> Self {
        self.attr("xmlns", value)
    }

    fn viewbox(self, value: &str) -> Self {
        self.attr("viewBox", value)
    }

    fn preserve_aspect_ratio(self, value: &str) -> Self {
        self.attr("preserveAspectRatio", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }
}

impl SvgRootElementAttributes for Element<'_, HtmlSvgTag> {}

// ============================================================================
// SVG Presentation Attributes (common to most SVG elements)
// ============================================================================

pub trait SvgPresentationAttributes
where
    Self: ElementAttributor + Sized,
{
    fn fill(self, value: &str) -> Self {
        self.attr("fill", value)
    }

    fn fill_opacity(self, value: &str) -> Self {
        self.attr("fill-opacity", value)
    }

    fn fill_rule(self, value: &str) -> Self {
        self.attr("fill-rule", value)
    }

    fn stroke(self, value: &str) -> Self {
        self.attr("stroke", value)
    }

    fn stroke_width(self, value: &str) -> Self {
        self.attr("stroke-width", value)
    }

    fn stroke_opacity(self, value: &str) -> Self {
        self.attr("stroke-opacity", value)
    }

    fn stroke_linecap(self, value: &str) -> Self {
        self.attr("stroke-linecap", value)
    }

    fn stroke_linejoin(self, value: &str) -> Self {
        self.attr("stroke-linejoin", value)
    }

    fn stroke_dasharray(self, value: &str) -> Self {
        self.attr("stroke-dasharray", value)
    }

    fn stroke_dashoffset(self, value: &str) -> Self {
        self.attr("stroke-dashoffset", value)
    }

    fn stroke_miterlimit(self, value: &str) -> Self {
        self.attr("stroke-miterlimit", value)
    }

    fn opacity(self, value: &str) -> Self {
        self.attr("opacity", value)
    }

    fn transform(self, value: &str) -> Self {
        self.attr("transform", value)
    }

    fn clip_path(self, value: &str) -> Self {
        self.attr("clip-path", value)
    }

    fn clip_rule(self, value: &str) -> Self {
        self.attr("clip-rule", value)
    }

    fn mask(self, value: &str) -> Self {
        self.attr("mask", value)
    }

    fn filter(self, value: &str) -> Self {
        self.attr("filter", value)
    }

    fn display(self, value: &str) -> Self {
        self.attr("display", value)
    }

    fn visibility(self, value: &str) -> Self {
        self.attr("visibility", value)
    }

    fn color(self, value: &str) -> Self {
        self.attr("color", value)
    }

    fn paint_order(self, value: &str) -> Self {
        self.attr("paint-order", value)
    }

    fn vector_effect(self, value: &str) -> Self {
        self.attr("vector-effect", value)
    }
}

// Apply presentation attributes to common SVG elements
impl SvgPresentationAttributes for Element<'_, HtmlSvgTag> {}
impl SvgPresentationAttributes for Element<'_, SvgCircleTag> {}
impl SvgPresentationAttributes for Element<'_, SvgEllipseTag> {}
impl SvgPresentationAttributes for Element<'_, SvgLineTag> {}
impl SvgPresentationAttributes for Element<'_, SvgPathTag> {}
impl SvgPresentationAttributes for Element<'_, SvgPolygonTag> {}
impl SvgPresentationAttributes for Element<'_, SvgPolylineTag> {}
impl SvgPresentationAttributes for Element<'_, SvgRectTag> {}
impl SvgPresentationAttributes for Element<'_, SvgGTag> {}
impl SvgPresentationAttributes for Element<'_, SvgTextTag> {}
impl SvgPresentationAttributes for Element<'_, SvgTspanTag> {}
impl SvgPresentationAttributes for Element<'_, SvgTextPathTag> {}
impl SvgPresentationAttributes for Element<'_, SvgImageTag> {}
impl SvgPresentationAttributes for Element<'_, SvgUseTag> {}

// ============================================================================
// SVG Circle Element Attributes
// ============================================================================

pub trait SvgCircleElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cx(self, value: &str) -> Self {
        self.attr("cx", value)
    }

    fn cy(self, value: &str) -> Self {
        self.attr("cy", value)
    }

    fn r(self, value: &str) -> Self {
        self.attr("r", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgCircleElementAttributes for Element<'_, SvgCircleTag> {}

// ============================================================================
// SVG Ellipse Element Attributes
// ============================================================================

pub trait SvgEllipseElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cx(self, value: &str) -> Self {
        self.attr("cx", value)
    }

    fn cy(self, value: &str) -> Self {
        self.attr("cy", value)
    }

    fn rx(self, value: &str) -> Self {
        self.attr("rx", value)
    }

    fn ry(self, value: &str) -> Self {
        self.attr("ry", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgEllipseElementAttributes for Element<'_, SvgEllipseTag> {}

// ============================================================================
// SVG Line Element Attributes
// ============================================================================

pub trait SvgLineElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x1(self, value: &str) -> Self {
        self.attr("x1", value)
    }

    fn y1(self, value: &str) -> Self {
        self.attr("y1", value)
    }

    fn x2(self, value: &str) -> Self {
        self.attr("x2", value)
    }

    fn y2(self, value: &str) -> Self {
        self.attr("y2", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgLineElementAttributes for Element<'_, SvgLineTag> {}

// ============================================================================
// SVG Path Element Attributes
// ============================================================================

pub trait SvgPathElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn d(self, value: &str) -> Self {
        self.attr("d", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgPathElementAttributes for Element<'_, SvgPathTag> {}

// ============================================================================
// SVG Polygon Element Attributes
// ============================================================================

pub trait SvgPolygonElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn points(self, value: &str) -> Self {
        self.attr("points", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgPolygonElementAttributes for Element<'_, SvgPolygonTag> {}

// ============================================================================
// SVG Polyline Element Attributes
// ============================================================================

pub trait SvgPolylineElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn points(self, value: &str) -> Self {
        self.attr("points", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgPolylineElementAttributes for Element<'_, SvgPolylineTag> {}

// ============================================================================
// SVG Rect Element Attributes
// ============================================================================

pub trait SvgRectElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn rx(self, value: &str) -> Self {
        self.attr("rx", value)
    }

    fn ry(self, value: &str) -> Self {
        self.attr("ry", value)
    }

    fn path_length(self, value: &str) -> Self {
        self.attr("pathLength", value)
    }
}

impl SvgRectElementAttributes for Element<'_, SvgRectTag> {}

// ============================================================================
// SVG Text Element Attributes
// ============================================================================

pub trait SvgTextElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn dx(self, value: &str) -> Self {
        self.attr("dx", value)
    }

    fn dy(self, value: &str) -> Self {
        self.attr("dy", value)
    }

    fn rotate(self, value: &str) -> Self {
        self.attr("rotate", value)
    }

    fn text_length(self, value: &str) -> Self {
        self.attr("textLength", value)
    }

    fn length_adjust(self, value: &str) -> Self {
        self.attr("lengthAdjust", value)
    }

    fn text_anchor(self, value: &str) -> Self {
        self.attr("text-anchor", value)
    }

    fn dominant_baseline(self, value: &str) -> Self {
        self.attr("dominant-baseline", value)
    }

    fn font_family(self, value: &str) -> Self {
        self.attr("font-family", value)
    }

    fn font_size(self, value: &str) -> Self {
        self.attr("font-size", value)
    }

    fn font_weight(self, value: &str) -> Self {
        self.attr("font-weight", value)
    }

    fn font_style(self, value: &str) -> Self {
        self.attr("font-style", value)
    }
}

impl SvgTextElementAttributes for Element<'_, SvgTextTag> {}
impl SvgTextElementAttributes for Element<'_, SvgTspanTag> {}

// ============================================================================
// SVG Use Element Attributes
// ============================================================================

pub trait SvgUseElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }
}

impl SvgUseElementAttributes for Element<'_, SvgUseTag> {}

// ============================================================================
// SVG Image Element Attributes
// ============================================================================

pub trait SvgImageElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }

    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn preserve_aspect_ratio(self, value: &str) -> Self {
        self.attr("preserveAspectRatio", value)
    }

    fn crossorigin(self, value: &str) -> Self {
        self.attr("crossorigin", value)
    }

    fn decoding(self, value: &str) -> Self {
        self.attr("decoding", value)
    }
}

impl SvgImageElementAttributes for Element<'_, SvgImageTag> {}

// ============================================================================
// SVG Gradient Stop Element Attributes
// ============================================================================

pub trait SvgStopElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn offset(self, value: &str) -> Self {
        self.attr("offset", value)
    }

    fn stop_color(self, value: &str) -> Self {
        self.attr("stop-color", value)
    }

    fn stop_opacity(self, value: &str) -> Self {
        self.attr("stop-opacity", value)
    }
}

impl SvgStopElementAttributes for Element<'_, SvgStopTag> {}

// ============================================================================
// SVG Linear Gradient Element Attributes
// ============================================================================

pub trait SvgLinearGradientElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x1(self, value: &str) -> Self {
        self.attr("x1", value)
    }

    fn y1(self, value: &str) -> Self {
        self.attr("y1", value)
    }

    fn x2(self, value: &str) -> Self {
        self.attr("x2", value)
    }

    fn y2(self, value: &str) -> Self {
        self.attr("y2", value)
    }

    fn gradient_units(self, value: &str) -> Self {
        self.attr("gradientUnits", value)
    }

    fn gradient_transform(self, value: &str) -> Self {
        self.attr("gradientTransform", value)
    }

    fn spread_method(self, value: &str) -> Self {
        self.attr("spreadMethod", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }
}

impl SvgLinearGradientElementAttributes for Element<'_, SvgLinearGradientTag> {}

// ============================================================================
// SVG Radial Gradient Element Attributes
// ============================================================================

pub trait SvgRadialGradientElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn cx(self, value: &str) -> Self {
        self.attr("cx", value)
    }

    fn cy(self, value: &str) -> Self {
        self.attr("cy", value)
    }

    fn r(self, value: &str) -> Self {
        self.attr("r", value)
    }

    fn fx(self, value: &str) -> Self {
        self.attr("fx", value)
    }

    fn fy(self, value: &str) -> Self {
        self.attr("fy", value)
    }

    fn fr(self, value: &str) -> Self {
        self.attr("fr", value)
    }

    fn gradient_units(self, value: &str) -> Self {
        self.attr("gradientUnits", value)
    }

    fn gradient_transform(self, value: &str) -> Self {
        self.attr("gradientTransform", value)
    }

    fn spread_method(self, value: &str) -> Self {
        self.attr("spreadMethod", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }
}

impl SvgRadialGradientElementAttributes for Element<'_, SvgRadialGradientTag> {}

// ============================================================================
// SVG Pattern Element Attributes
// ============================================================================

pub trait SvgPatternElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn viewbox(self, value: &str) -> Self {
        self.attr("viewBox", value)
    }

    fn pattern_units(self, value: &str) -> Self {
        self.attr("patternUnits", value)
    }

    fn pattern_content_units(self, value: &str) -> Self {
        self.attr("patternContentUnits", value)
    }

    fn pattern_transform(self, value: &str) -> Self {
        self.attr("patternTransform", value)
    }

    fn preserve_aspect_ratio(self, value: &str) -> Self {
        self.attr("preserveAspectRatio", value)
    }

    fn href(self, value: &str) -> Self {
        self.attr("href", value)
    }
}

impl SvgPatternElementAttributes for Element<'_, SvgPatternTag> {}

// ============================================================================
// SVG Marker Element Attributes
// ============================================================================

pub trait SvgMarkerElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn marker_width(self, value: &str) -> Self {
        self.attr("markerWidth", value)
    }

    fn marker_height(self, value: &str) -> Self {
        self.attr("markerHeight", value)
    }

    fn ref_x(self, value: &str) -> Self {
        self.attr("refX", value)
    }

    fn ref_y(self, value: &str) -> Self {
        self.attr("refY", value)
    }

    fn viewbox(self, value: &str) -> Self {
        self.attr("viewBox", value)
    }

    fn orient(self, value: &str) -> Self {
        self.attr("orient", value)
    }

    fn marker_units(self, value: &str) -> Self {
        self.attr("markerUnits", value)
    }

    fn preserve_aspect_ratio(self, value: &str) -> Self {
        self.attr("preserveAspectRatio", value)
    }
}

impl SvgMarkerElementAttributes for Element<'_, SvgMarkerTag> {}

// ============================================================================
// SVG Filter Element Attributes
// ============================================================================

pub trait SvgFilterElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn x(self, value: &str) -> Self {
        self.attr("x", value)
    }

    fn y(self, value: &str) -> Self {
        self.attr("y", value)
    }

    fn width(self, value: &str) -> Self {
        self.attr("width", value)
    }

    fn height(self, value: &str) -> Self {
        self.attr("height", value)
    }

    fn filter_units(self, value: &str) -> Self {
        self.attr("filterUnits", value)
    }

    fn primitive_units(self, value: &str) -> Self {
        self.attr("primitiveUnits", value)
    }
}

impl SvgFilterElementAttributes for Element<'_, SvgFilterTag> {}

// ============================================================================
// SVG Animation Element Attributes
// ============================================================================

pub trait SvgAnimationElementAttributes
where
    Self: ElementAttributor + Sized,
{
    fn attribute_name(self, value: &str) -> Self {
        self.attr("attributeName", value)
    }

    fn from(self, value: &str) -> Self {
        self.attr("from", value)
    }

    fn to(self, value: &str) -> Self {
        self.attr("to", value)
    }

    fn dur(self, value: &str) -> Self {
        self.attr("dur", value)
    }

    fn begin(self, value: &str) -> Self {
        self.attr("begin", value)
    }

    fn end(self, value: &str) -> Self {
        self.attr("end", value)
    }

    fn repeat_count(self, value: &str) -> Self {
        self.attr("repeatCount", value)
    }

    fn repeat_dur(self, value: &str) -> Self {
        self.attr("repeatDur", value)
    }

    fn fill_anim(self, value: &str) -> Self {
        self.attr("fill", value)
    }

    fn values(self, value: &str) -> Self {
        self.attr("values", value)
    }

    fn key_times(self, value: &str) -> Self {
        self.attr("keyTimes", value)
    }

    fn calc_mode(self, value: &str) -> Self {
        self.attr("calcMode", value)
    }
}

impl SvgAnimationElementAttributes for Element<'_, SvgAnimateTag> {}
impl SvgAnimationElementAttributes for Element<'_, SvgAnimateTransformTag> {}
impl SvgAnimationElementAttributes for Element<'_, SvgSetTag> {}
