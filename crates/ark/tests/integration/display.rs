//! `.ark.display_data()`, which sends a MIME bundle from R as a Jupyter
//! `display_data` message, and the `jupyter.base_display_func` option that
//! points IRdisplay at it: Replica's patch.

use amalthea::fixtures::dummy_frontend::ExecuteRequestOptions;
use ark_test::DummyArkFrontend;
use serde_json::json;

#[test]
fn test_display_data_sends_a_mime_bundle() {
    let frontend = DummyArkFrontend::lock();

    // Several strings are lines of one; raw bytes are sent base64-encoded;
    // the metadata goes with them; nothing is printed.
    let code = r##".ark.display_data(
        list("text/markdown" = c("# Title", "", "Words"), "image/png" = as.raw(1:3)),
        metadata = list("image/png" = list(width = 10L))
    )"##;
    frontend.send_execute_request(code, ExecuteRequestOptions::default());
    frontend.recv_iopub_busy();
    frontend.recv_iopub_execute_input();

    let display = frontend.recv_iopub_display_data_content();
    assert_eq!(
        display.data,
        json!({"text/markdown": "# Title\n\nWords", "image/png": "AQID"})
    );
    assert_eq!(display.metadata, json!({"image/png": {"width": 10}}));

    frontend.recv_iopub_idle();
    frontend.recv_shell_execute_reply();
}

#[test]
fn test_display_data_without_metadata_sends_an_empty_object() {
    let frontend = DummyArkFrontend::lock();

    frontend.send_execute_request(
        r#".ark.display_data(list("text/html" = "<b>bold</b>"))"#,
        ExecuteRequestOptions::default(),
    );
    frontend.recv_iopub_busy();
    frontend.recv_iopub_execute_input();

    let display = frontend.recv_iopub_display_data_content();
    assert_eq!(display.data, json!({"text/html": "<b>bold</b>"}));
    assert_eq!(display.metadata, json!({}));

    frontend.recv_iopub_idle();
    frontend.recv_shell_execute_reply();
}

#[test]
fn test_irdisplay_base_display_func_sends_display_data() {
    let frontend = DummyArkFrontend::lock();

    // IRdisplay calls getOption("jupyter.base_display_func")(data, metadata).
    frontend.send_execute_request(
        r#"getOption("jupyter.base_display_func")(list("text/plain" = "hello"), NULL)"#,
        ExecuteRequestOptions::default(),
    );
    frontend.recv_iopub_busy();
    frontend.recv_iopub_execute_input();

    let display = frontend.recv_iopub_display_data_content();
    assert_eq!(display.data, json!({"text/plain": "hello"}));

    frontend.recv_iopub_idle();
    frontend.recv_shell_execute_reply();
}
