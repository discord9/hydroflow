use quote::quote_spanned;

use super::{
    FlowProperties, FlowPropertyVal, OperatorCategory, OperatorConstraints, OperatorInstance,
    OperatorWriteOutput, WriteContextArgs, RANGE_0, RANGE_1,
};

/// > 1 input stream, 1 output stream
///
/// > Arguments: A Rust closure
/// For each item passed in, apply the closure to generate an item to emit.
///
/// If you do not want to modify the item stream and instead only want to view
/// each item use the [`inspect`](#inspect) operator instead.
///
/// > Note: The closure has access to the [`context` object](surface_flows.md#the-context-object).
///
/// ```hydroflow
/// source_iter(vec!["hello", "world"]) -> map(|x| x.to_uppercase())
///     -> assert_eq(["HELLO", "WORLD"]);
/// ```
pub const MAP: OperatorConstraints = OperatorConstraints {
    name: "map",
    categories: &[OperatorCategory::Map],
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    num_args: 1,
    persistence_args: RANGE_0,
    type_args: RANGE_0,
    is_external_input: false,
    ports_inn: None,
    ports_out: None,
    properties: FlowProperties {
        deterministic: FlowPropertyVal::DependsOnArgs,
        monotonic: FlowPropertyVal::DependsOnArgs,
        inconsistency_tainted: false,
    },
    input_delaytype_fn: |_| None,
    flow_prop_fn: Some(|flow_props_in, _op_inst, _star_ord| {
        // Preserve input flow properties.
        vec![flow_props_in[0]]
    }),
    write_fn: |wc @ &WriteContextArgs {
                   root,
                   op_span,
                   ident,
                   inputs,
                   outputs,
                   is_pull,
                   op_inst: OperatorInstance { arguments, .. },
                   ..
               },
               _| {
        let func = wc.wrap_check_func_arg(&arguments[0]);
        let write_iterator = if is_pull {
            let input = &inputs[0];
            quote_spanned! {op_span=>
                let #ident = #input.map(#func);
            }
        } else {
            let output = &outputs[0];
            quote_spanned! {op_span=>
                let #ident = #root::pusherator::map::Map::new(#func, #output);
            }
        };
        Ok(OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        })
    },
};
