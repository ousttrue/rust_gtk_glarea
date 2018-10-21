extern crate epoxy;
extern crate gio;
extern crate gl;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

mod renderers;
use self::renderers::renderer::Renderer;
use self::renderers::basic_renderer::BasicRenderer;
//use self::renderers::empty_renderer::EmptyRenderer;

mod gl_loader;


fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Window");
    window.set_default_size(640, 480);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    let gl = std::rc::Rc::new(BasicRenderer::new());

    let gl_area = gtk::GLArea::new();
    //gl_area.set_vexpand(true);
    //gtk_widget_set_hexpand(gl_area, TRUE);
    let gl_clone = gl.clone();
    gl_area.connect_realize(move |gl_area| {
        gl_area.make_current();
        /*
        if (gtk_gl_area_get_error(area) != NULL) {
            fprintf(stderr, "Unknown error\n");
            return;
        }
        */
        gl_area.set_has_depth_buffer(true);

        gl_loader::load();

        gl_clone.initialize();
    });

    let gl_clone = gl.clone();
    gl_area.connect_render(move |_area, _context| {
        gl_clone.render();

        Inhibit(true)
    });

    window.add(&gl_area);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.basic", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_|{

    });

    application.run(&args().collect::<Vec<_>>());
}
