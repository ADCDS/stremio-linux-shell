use adw::subclass::prelude::*;
use gtk::{
    GestureClick,
    gdk::RGBA,
    glib::{self, clone},
    prelude::*,
};
use tracing::error;
use webkit::{WebProcessTerminationReason, WebView as WebKitWebView, prelude::*};

#[derive(Default)]
pub struct WebView {
    pub webview: WebKitWebView,
}

#[glib::object_subclass]
impl ObjectSubclass for WebView {
    const NAME: &'static str = "WebView";
    type Type = super::WebView;
    type ParentType = gtk::Box;
}

impl ObjectImpl for WebView {
    fn constructed(&self) {
        self.parent_constructed();

        let object = self.obj();

        self.webview.set_vexpand(true);
        self.webview.set_hexpand(true);
        self.webview
            .set_background_color(&RGBA::new(0.0, 0.0, 0.0, 0.0));

        if let Some(settings) = WebViewExt::settings(&self.webview) {
            settings.set_enable_media(false);
            settings.set_enable_media_capabilities(false);
            settings.set_enable_media_stream(false);
            settings.set_enable_webaudio(false);
        }

        let gesture = GestureClick::new();
        gesture.set_button(0);
        gesture.connect_pressed(clone!(
            #[weak(rename_to = webview)]
            self.webview,
            move |gesture, _, _, _| {
                let button = gesture.current_button();
                match button {
                    8 => webview.go_back(),
                    9 => webview.go_forward(),
                    _ => {}
                }
            }
        ));
        self.webview.add_controller(gesture);

        // A crashed WebKit content process otherwise leaves the native shell alive
        // with a permanently frozen surface. Reloading starts a fresh content process
        // while preserving the WebView's user-content manager and IPC preload script.
        self.webview
            .connect_web_process_terminated(|webview, reason| {
                error!("WebKit content process terminated: {reason:?}");
                if matches!(
                    reason,
                    WebProcessTerminationReason::Crashed
                        | WebProcessTerminationReason::ExceededMemoryLimit
                ) {
                    webview.reload();
                }
            });

        object.append(&self.webview);
    }
}

impl WidgetImpl for WebView {}
impl BoxImpl for WebView {}
