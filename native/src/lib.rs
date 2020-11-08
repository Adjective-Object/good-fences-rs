extern crate good_fences_rs_core;
use good_fences_rs_core::evaluate_fences::{ImportRuleViolation, ViolatedFenceClause};
use good_fences_rs_core::good_fences_runner::{GoodFencesRunner, UndefinedTagReference};
use neon::prelude::*;

fn convert_violation_to_message<'js_context, T>(
    cx: &mut T,
    violation: &Result<ImportRuleViolation, String>,
) -> JsResult<'js_context, JsObject>
where
    T: Context<'js_context>,
{
    let o = cx.empty_object();
    match violation {
        Err(err_msg) => {
            let message_js_string = cx.string(format!("error while evaluating fences {}", err_msg));
            o.set(cx, "message", message_js_string)?;
            return Ok(o);
        }
        Ok(violation) => {
            let file_path_js_string = cx.string(violation.violating_file_path);
            o.set(cx, "sourceFile", file_path_js_string)?;
            let violating_import_specifier_js_string =
                cx.string(violation.violating_import_specifier);
            o.set(cx, "rawImport", violating_import_specifier_js_string)?;
            let violating_fence_path_js_string = cx.string(&violation.violating_fence.fence_path);
            o.set(cx, "fencePath", violating_fence_path_js_string)?;
            let msg = cx.string(match violation.violating_fence_clause {
                ViolatedFenceClause::ImportAllowList => "Import not allowed".to_owned(),
                ViolatedFenceClause::DependencyRule(dependency_rule) => match dependency_rule {
                    Some(export_rule) => format!(
                        "Dependency is not allowed. {} is accessible to tags {}",
                        export_rule.dependency,
                        export_rule
                            .accessible_to
                            .iter()
                            .map(|x| format!("'{}'", x))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None => "Dependency is not allowed".to_owned(),
                },
                ViolatedFenceClause::ExportRule(export_rule) => match export_rule {
                    Some(export_rule) => format!(
                        "Module is not exported. {} is accessible to tags {}",
                        export_rule.modules,
                        export_rule
                            .accessible_to
                            .iter()
                            .map(|x| format!("'{}'", x))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None => "Module is not exported".to_owned(),
                },
            });
            o.set(cx, "message", msg)?;

            return Ok(o);
        }
    };
}

fn convert_undefined_tag_to_message<'js_context, T>(
    cx: &mut T,
    undefined_tag_reference: &UndefinedTagReference,
) -> JsResult<'js_context, JsObject>
where
    T: Context<'js_context>,
{
    let o = cx.empty_object();
    let message_js_string = cx.string(&format!(
        "Tag '{}' is referred to but is not defined in any fence.",
        undefined_tag_reference.tag,
    ));
    o.set(cx, "message", message_js_string)?;
    let fence_path_js_string = cx.string(undefined_tag_reference.referencing_fence_path);
    o.set(cx, "fencePath", fence_path_js_string)?;

    Ok(o)
}

/**
 * walks a file directory and returns a series of raw rule violation messages
 *
 * tsconfig_paths_json_path: &str,
 * directory_paths_to_walk: &Vec<&str>,
 */
fn run_good_fences(mut cx: FunctionContext) -> JsResult<JsObject> {
    let js_string_tsconfig_path = (cx.argument::<JsString>(0)?).value();
    let js_array_walk_paths = cx.argument::<JsArray>(1)?;
    let walk_paths_js_strings: Vec<Handle<JsValue>> = js_array_walk_paths.to_vec(&mut cx)?;
    let mut walk_path_strings: Vec<String> = vec![];
    for x in walk_paths_js_strings.iter() {
        let r: JsResult<JsString> = x.downcast_or_throw(&mut cx);
        match r {
            Ok(js_string) => {
                let js_string_val = js_string.value();
                walk_path_strings.push(js_string_val);
            }
            Err(e) => return Err(e),
        }
    }

    let walk_path_strings_refs = walk_path_strings
        .iter()
        .map(|x| -> &str { &x })
        .collect::<Vec<_>>();
    let good_fences_runner =
        GoodFencesRunner::new(&js_string_tsconfig_path, &walk_path_strings_refs);
    let import_violations = good_fences_runner.find_import_violations();
    let undefined_tag_warnings = good_fences_runner.find_undefined_tags();

    let errors_js_arr = cx.empty_array();
    let warnings_js_arr = cx.empty_array();
    for (i, import_violation) in import_violations.iter().enumerate() {
        let error = convert_violation_to_message(&mut cx, import_violation);
        errors_js_arr.set(&mut cx, i as u32, error?)?;
    }

    for (i, undefined_tag_warning) in undefined_tag_warnings.iter().enumerate() {
        let warning = convert_undefined_tag_to_message(&mut cx, undefined_tag_warning);
        warnings_js_arr.set(&mut cx, i as u32, warning?)?;
    }

    let obj_to_return = cx.empty_object();
    obj_to_return.set(&mut cx, "errors", errors_js_arr)?;
    obj_to_return.set(&mut cx, "warnings", warnings_js_arr)?;

    return Ok(obj_to_return);
}

register_module!(mut cx, {
    cx.export_function("run_good_fences", run_good_fences)
});
