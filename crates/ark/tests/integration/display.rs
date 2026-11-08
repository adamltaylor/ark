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

#[test]
fn test_send_plot_sends_a_plot_before_what_follows_and_updates_it_after() {
    let frontend = DummyArkFrontend::lock();

    // The plot goes out when `.ark.send_plot()` is called, before the output
    // after it, and what is drawn on it afterwards comes as an update to it.
    let code = r#"plot(1:3); .ark.send_plot(); .ark.display_data(list("text/plain" = "after")); lines(1:3)"#;
    frontend.send_execute_request(code, ExecuteRequestOptions::default());
    frontend.recv_iopub_busy();
    frontend.recv_iopub_execute_input();

    let plot = frontend.recv_iopub_display_data_content();
    assert!(plot.data["image/png"].is_string());
    let after = frontend.recv_iopub_display_data_content();
    assert_eq!(after.data, json!({"text/plain": "after"}));
    frontend.recv_iopub_update_display_data();

    frontend.recv_iopub_idle();
    frontend.recv_shell_execute_reply();
}

#[test]
fn test_send_plot_with_nothing_pending_sends_nothing() {
    let frontend = DummyArkFrontend::lock();

    frontend.send_execute_request(
        r#".ark.send_plot(); .ark.display_data(list("text/plain" = "only this"))"#,
        ExecuteRequestOptions::default(),
    );
    frontend.recv_iopub_busy();
    frontend.recv_iopub_execute_input();

    let display = frontend.recv_iopub_display_data_content();
    assert_eq!(display.data, json!({"text/plain": "only this"}));

    frontend.recv_iopub_idle();
    frontend.recv_shell_execute_reply();
}
