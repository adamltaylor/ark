#
# display.R
#
# Replica's patch: showing a MIME bundle from R, as a Jupyter `display_data`
# message, and pointing IRdisplay at it.
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

# IRdisplay (and packages built on it) sends its bundles through this option,
# which IRkernel sets, so that `IRdisplay::display_markdown()` and the like
# work in Ark too.
options(jupyter.base_display_func = .ark.display_data)
