// --- LLM-generated --- //
use rustc_ast::{LitKind, MetaItemLit};
use rustc_attr_ir::NvptxAttr;
use rustc_feature::AttributeStability;

use super::prelude::*;
use crate::diagnostics::{NvptxConflictingKeys, NvptxExpectedThreeDims};

const KEYS: &[Symbol] = &[
    sym::max_ctas_per_cluster,
    sym::min_ctas_per_sm,
    sym::max_registers,
    sym::max_threads_per_cta,
    sym::exact_threads_per_cta,
    sym::exact_cluster_dim,
];

pub(crate) struct NvptxParser;

impl SingleAttributeParser for NvptxParser {
    const PATH: &[Symbol] = &[sym::nvptx];
    const ALLOWED_TARGETS: AllowedTargets<'_> =
        AllowedTargets::AllowList(&[Allow(Target::Fn), Allow(Target::Param)]);
    const TEMPLATE: AttributeTemplate = template!(List: &[
        "max_ctas_per_cluster(n), min_ctas_per_sm(n), max_registers(n), \
         max_threads_per_cta(x, y, z), exact_threads_per_cta(x, y, z), exact_cluster_dim(x, y, z)",
        "grid_constant"
    ]);
    const STABILITY: AttributeStability = unstable!(nvptx_ext);

    fn convert(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
        if cx.target == Target::Param {
            return convert_param(cx, args);
        }

        let list = cx.expect_list(args, cx.attr_span)?;
        if list.is_empty() {
            cx.adcx().expected_at_least_one_argument(list.span);
            return None;
        }

        let mut attr = NvptxAttr::default();
        let mut max_threads_span = None;
        let mut exact_threads_span = None;
        let mut max_ctas_span = None;
        let mut exact_cluster_span = None;

        for item in list.mixed() {
            let Some(meta_item) = item.meta_item() else {
                cx.adcx().expected_not_literal(item.span());
                return None;
            };
            let Some(ident) = meta_item.ident() else {
                cx.adcx().expected_specific_argument(meta_item.path().span(), KEYS);
                return None;
            };
            let args = meta_item.args();
            let span = meta_item.span();

            let duplicate = match ident.name {
                sym::max_ctas_per_cluster => {
                    max_ctas_span = Some(span);
                    attr.max_ctas_per_cluster.replace(parse_scalar(cx, args, span)?).is_some()
                }
                sym::min_ctas_per_sm => {
                    attr.min_ctas_per_sm.replace(parse_scalar(cx, args, span)?).is_some()
                }
                sym::max_registers => {
                    attr.max_registers.replace(parse_scalar(cx, args, span)?).is_some()
                }
                sym::max_threads_per_cta => {
                    max_threads_span = Some(span);
                    attr.max_threads_per_cta.replace(parse_dims(cx, args, span)?).is_some()
                }
                sym::exact_threads_per_cta => {
                    exact_threads_span = Some(span);
                    attr.exact_threads_per_cta.replace(parse_dims(cx, args, span)?).is_some()
                }
                sym::exact_cluster_dim => {
                    exact_cluster_span = Some(span);
                    attr.exact_cluster_dim.replace(parse_dims(cx, args, span)?).is_some()
                }
                _ => {
                    cx.adcx().expected_specific_argument(ident.span, KEYS);
                    return None;
                }
            };
            if duplicate {
                cx.adcx().duplicate_key(ident.span, ident.name);
                return None;
            }
        }

        let mut conflict = false;
        for (first, second, first_span, second_span) in [
            (
                sym::max_threads_per_cta,
                sym::exact_threads_per_cta,
                max_threads_span,
                exact_threads_span,
            ),
            (sym::max_ctas_per_cluster, sym::exact_cluster_dim, max_ctas_span, exact_cluster_span),
        ] {
            if let (Some(first_span), Some(second_span)) = (first_span, second_span) {
                cx.emit_err(NvptxConflictingKeys { first_span, second_span, first, second });
                conflict = true;
            }
        }
        if conflict {
            return None;
        }

        Some(AttributeKind::Nvptx(attr, cx.attr_span))
    }
}

fn convert_param(cx: &mut AcceptContext<'_, '_>, args: &ArgParser) -> Option<AttributeKind> {
    let list = cx.expect_list(args, cx.attr_span)?;
    let item = cx.expect_single(list)?;
    let Some(meta_item) = item.meta_item() else {
        cx.adcx().expected_not_literal(item.span());
        return None;
    };
    if meta_item.ident().map(|ident| ident.name) != Some(sym::grid_constant) {
        cx.adcx().expected_specific_argument(meta_item.path().span(), &[sym::grid_constant]);
        return None;
    }
    cx.expect_no_args(meta_item.args())?;
    Some(AttributeKind::NvptxGridConstant(cx.attr_span))
}

fn parse_scalar(cx: &mut AcceptContext<'_, '_>, args: &ArgParser, span: Span) -> Option<u32> {
    let list = cx.expect_list(args, span)?;
    let item = cx.expect_single(list)?;
    let Some(lit) = item.as_lit() else {
        cx.adcx().expected_integer_literal(item.span());
        return None;
    };
    parse_int(cx, lit)
}

fn parse_dims(
    cx: &mut AcceptContext<'_, '_>,
    args: &ArgParser,
    span: Span,
) -> Option<(u32, u32, u32)> {
    let list = cx.expect_list(args, span)?;
    let mut dims = Vec::with_capacity(3);
    for item in list.mixed() {
        let Some(lit) = item.as_lit() else {
            cx.adcx().expected_integer_literal(item.span());
            return None;
        };
        dims.push(parse_int(cx, lit)?);
    }
    let [x, y, z] = dims[..] else {
        cx.emit_err(NvptxExpectedThreeDims { span: list.span, found: dims.len() });
        return None;
    };
    Some((x, y, z))
}

fn parse_int(cx: &mut AcceptContext<'_, '_>, lit: &MetaItemLit) -> Option<u32> {
    if let LitKind::Int(val, _) = lit.kind
        && let Ok(val) = u32::try_from(val.get())
        && val != 0
    {
        return Some(val);
    }
    cx.adcx().expected_integer_literal_in_range(
        lit.span,
        1,
        isize::try_from(u32::MAX).unwrap_or(isize::MAX),
    );
    None
}
