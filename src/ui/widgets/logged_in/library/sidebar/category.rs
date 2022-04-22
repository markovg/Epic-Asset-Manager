use gtk4::subclass::prelude::*;
use gtk4::{self, prelude::*};
use gtk4::{glib, CompositeTemplate};

pub(crate) mod imp {
    use super::*;
    use glib::ParamSpec;
    use gtk4::gio;
    use gtk4::glib::ParamSpecString;
    use once_cell::sync::OnceCell;
    use std::cell::RefCell;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/io/github/achetagames/epic_asset_manager/sidebar_category.ui")]
    pub struct EpicSidebarCategory {
        pub sidebar: OnceCell<crate::ui::widgets::logged_in::library::sidebar::EpicSidebar>,
        pub actions: gio::SimpleActionGroup,
        pub title: RefCell<Option<String>>,
        pub icon_name: RefCell<Option<String>>,
        pub filter: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpicSidebarCategory {
        const NAME: &'static str = "EpicSidebarCategory";
        type Type = super::EpicSidebarCategory;
        type ParentType = gtk4::Box;

        fn new() -> Self {
            Self {
                sidebar: OnceCell::new(),
                actions: gio::SimpleActionGroup::new(),
                title: RefCell::new(None),
                icon_name: RefCell::new(None),
                filter: RefCell::new(None),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EpicSidebarCategory {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_actions();
        }
        fn properties() -> &'static [ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        "title",
                        "title",
                        "The category title",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "icon-name",
                        "icon name",
                        "The Icon Name",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "filter",
                        "Filter",
                        "Filter",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "title" => {
                    let title = value.get().unwrap();
                    self.title.replace(title);
                }
                "filter" => {
                    let filter = value.get().unwrap();
                    self.filter.replace(filter);
                }
                "icon-name" => {
                    let icon_name = value.get().unwrap();
                    self.icon_name.replace(icon_name);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "title" => self.title.borrow().to_value(),
                "icon-name" => self.icon_name.borrow().to_value(),
                "filter" => self.filter.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for EpicSidebarCategory {}
    impl BoxImpl for EpicSidebarCategory {}
}

glib::wrapper! {
    pub struct EpicSidebarCategory(ObjectSubclass<imp::EpicSidebarCategory>)
        @extends gtk4::Widget, gtk4::Box;
}

impl Default for EpicSidebarCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl EpicSidebarCategory {
    pub fn new() -> Self {
        let stack: Self = glib::Object::new(&[]).expect("Failed to create EpicSidebarCategory");
        stack
    }

    pub fn set_sidebar(
        &self,
        sidebar: &crate::ui::widgets::logged_in::library::sidebar::EpicSidebar,
    ) {
        let self_ = self.imp();
        // Do not run this twice
        if self_.sidebar.get().is_some() {
            return;
        }

        self_.sidebar.set(sidebar.clone()).unwrap();
    }

    pub fn setup_actions(&self) {
        let self_ = self.imp();
        self.insert_action_group("sidebar_category", Some(&self_.actions));
    }
}
