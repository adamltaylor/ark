#
# display.R
#
# Replica's patches: showing a MIME bundle from R, as a Jupyter
# `display_data` message, and pointing IRdisplay at it; and sending the
# current plot now.
#

#' Send a MIME bundle to the frontend as a `display_data` message
#'
#' @param data A named list: each element is the data for the MIME type it
#'   is named by, e.g. `list("text/markdown" = "# Title")`. Several strings
#'   are lines of one; raw bytes (an image, say) are sent base64-encoded.
#' @param metadata A list of metadata, e.g. sizes for images, or `NULL`.
#' @export
.ark.display_data <- function(data, metadata = NULL) {
    .ps.Call("ps_display_data", data, metadata)
    invisible()
}

#' Send the current plot now
#'
#' Ark sends a plot when the next one starts or the code has run, since
#' `lines()`, `legend()` and the like can still be drawing on it. This sends
#' any change to it now, so that it comes out before the output that follows;
#' what is drawn on it afterwards comes as an update to it.
#' @export
.ark.send_plot <- function() {
    .ps.Call("ps_send_plot")
    invisible()
}

# IRdisplay (and packages built on it) sends its bundles through this option,
# which IRkernel sets, so that `IRdisplay::display_markdown()` and the like
# work in Ark too.
options(jupyter.base_display_func = .ark.display_data)
