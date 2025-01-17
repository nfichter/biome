use crate::prelude::*;
use crate::utils::AnyJsAssignmentLike;

use biome_js_syntax::JsPropertyObjectMember;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsPropertyObjectMember;

impl FormatNodeRule<JsPropertyObjectMember> for FormatJsPropertyObjectMember {
    fn fmt_fields(&self, node: &JsPropertyObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [AnyJsAssignmentLike::from(node.clone())]]
    }
}
