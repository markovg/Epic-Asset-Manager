use glib::subclass::prelude::*;
use glib::{subclass, ObjectExt};
use serde::de::DeserializeOwned;

// Implementation sub-module of the GObject
mod imp {
    use super::*;
    use glib::ToValue;
    use std::cell::RefCell;

    // The actual data structure that stores our values. This is not accessible
    // directly from the outside.
    pub struct RowData {
        data: RefCell<Option<String>>,
        thumbnail: RefCell<Option<String>>,
    }

    // Basic declaration of our type for the GObject type system
    impl ObjectSubclass for RowData {
        const NAME: &'static str = "RowData";
        type Type = super::RowData;
        type ParentType = glib::Object;
        type Interfaces = ();
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        // Called once at the very beginning of instantiation of each instance and
        // creates the data structure that contains all our state
        fn new() -> Self {
            Self {
                data: RefCell::new(None),
                thumbnail: RefCell::new(None),
            }
        }
    }

    // The ObjectImpl trait provides the setters/getters for GObject properties.
    // Here we need to provide the values that are internally stored back to the
    // caller, or store whatever new value the caller is providing.
    //
    // This maps between the GObject properties and our internal storage of the
    // corresponding values of the properties.
    impl ObjectImpl for RowData {
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpec::string(
                        "data",
                        "Data",
                        "Data",
                        None, // Default value
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpec::string(
                        "thumbnail",
                        "Thumbnail",
                        "Thumbnail",
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
            pspec: &glib::ParamSpec,
        ) {
            match pspec.get_name() {
                "data" => {
                    let data = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.data.replace(data);
                }
                "thumbnail" => {
                    let thumbnail = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.thumbnail.replace(thumbnail);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            pspec: &glib::ParamSpec,
        ) -> glib::Value {
            match pspec.get_name() {
                "data" => self.data.borrow().to_value(),
                "thumbnail" => self.thumbnail.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

// Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
// binding
glib::wrapper! {
    pub struct RowData(ObjectSubclass<imp::RowData>);
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl RowData {
    pub fn new<O>(object: O, image: Vec<u8>) -> RowData
    where
        O: serde::ser::Serialize,
    {
        glib::Object::new(&[
            ("data", &serde_json::to_string(&object).unwrap()),
            ("thumbnail", &Some(hex::encode(image))),
        ])
        .expect("Failed to create row data")
    }

    pub fn deserialize<O>(&self) -> O
    where
        O: DeserializeOwned,
    {
        let data = self.get_property("data").unwrap().get::<String>().unwrap();
        serde_json::from_str(&data.unwrap()).unwrap()
    }

    pub fn image(&self) -> Vec<u8> {
        match self
            .get_property("thumbnail")
            .unwrap()
            .get::<String>()
            .unwrap()
        {
            None => {
                vec![]
            }
            Some(img) => match hex::decode(img) {
                Ok(v) => v,
                Err(_) => {
                    vec![]
                }
            },
        }
    }
}