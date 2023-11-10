use gtk::{gio, glib, subclass::prelude::ObjectSubclassIsExt};

mod imp {

    use adw::{prelude::TextViewExt, subclass::prelude::*};
    use gtk::{
        gio,
        glib::{self, object_subclass, subclass::InitializingObject},
        ColumnViewColumn,
    };
    use sourceview5::{prelude::*, Buffer, LanguageManager};

    use crate::other_page::OtherPage;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(file = "resources/window.blp")]
    pub struct Window {
        #[template_child]
        pub main_stack: TemplateChild<adw::ViewStack>,

        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,

        #[template_child]
        pub switch_stack: TemplateChild<gtk::Button>,

        #[template_child]
        pub next_page: TemplateChild<gtk::Button>,

        #[template_child]
        pub switch_to_nav: TemplateChild<gtk::Button>,

        #[template_child]
        pub column_view: TemplateChild<gtk::ColumnView>,

        #[template_child]
        pub other_page: TemplateChild<OtherPage>,

        #[template_child]
        pub source_view: TemplateChild<sourceview5::View>,

        #[template_child]
        pub source_view_buffer: TemplateChild<sourceview5::Buffer>,
    }

    #[object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MyWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);

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

            // ColumnViewColumn must be stored in a Vec<Box<ColumnViewColumn>> to
            // always have the same pointer, making them removeable
            self.column_view
                .append_column(&ColumnViewColumn::builder().title("Column 1").build());
            self.column_view
                .append_column(&ColumnViewColumn::builder().title("Column 2").build());
            self.column_view
                .append_column(&ColumnViewColumn::builder().title("Column 3").build());

            let sql = LanguageManager::default().language("sql").unwrap();
            // self.source_view.buffer().set_language(Some(sql));
            self.source_view_buffer.set_language(Some(&sql));
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
        gio::Initable::builder()
            .property("application", app)
            .build(gio::Cancellable::NONE)
            .unwrap()
    }

    pub fn set_view(&self, view: &str) {
        println!("set_view");
        let imp = self.imp();
        if view == "defualt" {
            imp.main_stack.set_visible_child_name("default");
        } else if view == "other" {
            imp.main_stack.set_visible_child_name("nav");
            imp.navigation_view.push_by_tag("other");
        }
    }
}
