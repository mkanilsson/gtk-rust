use gtk::{
    gio,
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};

mod imp {

    use adw::subclass::prelude::*;
    use gtk::glib::{self, object_subclass, subclass::InitializingObject};

    use crate::other_page::OtherPage;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/se/andras/gtkadw/window.ui")]
    pub struct Window {
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,

        #[template_child]
        pub switch_stack: TemplateChild<gtk::Button>,

        #[template_child]
        pub next_page: TemplateChild<gtk::Button>,

        #[template_child]
        pub switch_to_nav: TemplateChild<gtk::Button>,

        #[template_child]
        pub other_page: TemplateChild<OtherPage>,
    }

    #[object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MyWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("win.switch_stack_default", None, move |win, _, _| {
                win.imp().main_stack.set_visible_child_name("default");
            });

            klass.install_action("win.switch_stack_nav", None, move |win, _, _| {
                win.imp().main_stack.set_visible_child_name("nav");
            });

            klass.install_action("win.switch_to_next_page", None, move |win, _, _| {
                win.set_view("other");
            });
            klass.install_action("win.switch_to_first_page", None, move |win, _, _| {
                win.imp().navigation_view.pop();
            });
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl InitableImpl for Window {
        fn init(&self, _cancellable: Option<&gtk::gio::Cancellable>) -> Result<(), glib::Error> {
            let win = self.obj();

            win.set_view("default");
            Ok(())
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, adw::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gio::Initable;
}

#[gtk::template_callbacks]
impl Window {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }

    pub fn set_view(&self, view: &str) {
        let imp = self.imp();
        if view == "defualt" {
            imp.main_stack.set_visible_child_name("default");
        } else if view == "other" {
            imp.main_stack.set_visible_child_name("nav");
            imp.navigation_view.push_by_tag("other");
        }
    }
}
