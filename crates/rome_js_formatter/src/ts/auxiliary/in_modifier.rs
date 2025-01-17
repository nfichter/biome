use crate::prelude::*;
use biome_js_syntax::{TsInModifier, TsInModifierFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsInModifier;

impl FormatNodeRule<TsInModifier> for FormatTsInModifier {
    fn fmt_fields(&self, node: &TsInModifier, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInModifierFields { modifier_token } = node.as_fields();
        write![f, [modifier_token.format()]]
    }
}
