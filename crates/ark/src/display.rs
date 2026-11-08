//
// display.rs
//
// Replica's patch: sending a MIME bundle from R as a Jupyter `display_data`
// message, for `.ark.display_data()` in `modules/positron/display.R`.
//

use amalthea::socket::iopub::IOPubMessage;
use amalthea::wire::display_data::DisplayData;
use anyhow::anyhow;
use base64::prelude::*;
use harp::object::RObject;
use libr::R_NilValue;
use libr::Rf_xlength;
use libr::RAW;
use libr::RAWSXP;
use libr::SEXP;
use libr::STRSXP;
use serde_json::Map;
use serde_json::Value;

use crate::console::Console;

/// Send `data`, a named list of MIME types and their data, with `metadata`
/// (a list, or `NULL` for none), as a `display_data` message.
#[harp::register]
pub unsafe extern "C-unwind" fn ps_display_data(data: SEXP, metadata: SEXP) -> anyhow::Result<SEXP> {
    let data = mime_bundle(RObject::view(data))?;
    let metadata = match Value::try_from(RObject::view(metadata))? {
        Value::Null => Value::Object(Map::new()),
        metadata => metadata,
    };

    let message = IOPubMessage::DisplayData(DisplayData {
        data,
        metadata,
        transient: Value::Null,
    });
    Console::get().iopub_tx().send(message)?;

    Ok(R_NilValue)
}

/// The JSON object for a MIME bundle: each element's data under its name.
fn mime_bundle(bundle: RObject) -> anyhow::Result<Value> {
    let Some(names) = bundle.names() else {
        return Err(anyhow!("`data` must be a named list of MIME types and their data"));
    };

    let mut map = Map::new();
    for (index, name) in names.into_iter().enumerate() {
        let Some(name) = name else {
            return Err(anyhow!("Every element of `data` must be named by its MIME type"));
        };
        let value = bundle.vector_elt(index as isize)?;
        map.insert(name, mime_data(value)?);
    }
    Ok(Value::Object(map))
}

/// The JSON for one MIME type's data: raw bytes (an image, say) as base64,
/// strings as lines of one, and anything else (`application/json`) as it is.
fn mime_data(value: RObject) -> anyhow::Result<Value> {
    match value.kind() {
        RAWSXP => {
            let bytes = unsafe {
                let len = Rf_xlength(value.sexp) as usize;
                if len == 0 {
                    &[]
                } else {
                    std::slice::from_raw_parts(RAW(value.sexp), len)
                }
            };
            Ok(Value::String(BASE64_STANDARD.encode(bytes)))
        },
        STRSXP => {
            let lines: Vec<String> = value.try_into()?;
            Ok(Value::String(lines.join("\n")))
        },
        _ => Ok(Value::try_from(value)?),
    }
}
