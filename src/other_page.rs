use gtk::glib::{self, Object};

mod imp {

    use adw::subclass::prelude::*;
    use gtk::{
        glib::{self, object_subclass, subclass::InitializingObject},
        Button,
    };

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/se/andras/gtkadw/other.ui")]
    pub struct OtherPage {
        #[template_child]
        pub btn: TemplateChild<Button>,
    }

    #[object_subclass]
    impl ObjectSubclass for OtherPage {
        const NAME: &'static str = "OtherPage";
        type Type = super::OtherPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for OtherPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for OtherPage {}
    impl NavigationPageImpl for OtherPage {}
}

glib::wrapper! {
    pub struct OtherPage(ObjectSubclass<imp::OtherPage>)
        @extends gtk::Widget, adw::NavigationPage;
}

#[gtk::template_callbacks]
impl OtherPage {
    pub fn new(app: &adw::Application) -> Self {
        Object::builder().property("application", app).build()
    }
}
