use crate::*;
/* automatically generated by rust-bindgen 0.71.1 */

unsafe extern "C" {
    #[doc = "Cairo graphics backend accessor.\n\nPass the returned value to puglSetBackend() to draw to a view with Cairo."]
    pub fn puglCairoBackend() -> *const PuglBackend;
}
