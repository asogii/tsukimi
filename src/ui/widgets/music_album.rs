use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

use crate::{ui::provider::tu_item::TuItem, utils::get_image_with_cache};

mod imp {
    use std::cell::OnceCell;

    use crate::{ui::provider::tu_item::TuItem, utils::spawn_g_timeout};

    use super::*;
    use glib::subclass::InitializingObject;

    #[derive(CompositeTemplate, Default, glib::Properties)]
    #[template(resource = "/moe/tsukimi/album_widget.ui")]
    #[properties(wrapper_type = super::AlbumPage)]
    pub struct AlbumPage {
        #[property(get, set, construct_only)]
        pub item: OnceCell<TuItem>,
        #[template_child]
        pub cover_image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub title_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub artist_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub released_label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AlbumPage {
        const NAME: &'static str = "AlbumPage";
        type Type = super::AlbumPage;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for AlbumPage {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            spawn_g_timeout(glib::clone!(@weak obj => async move {
                obj.set_album().await;
            }));
        }
    }

    impl WidgetImpl for AlbumPage {}
    impl AdwDialogImpl for AlbumPage {}
    impl NavigationPageImpl for AlbumPage {}
}

glib::wrapper! {
    /// Preference Window to display and update room details.
    pub struct AlbumPage(ObjectSubclass<imp::AlbumPage>)
        @extends gtk::Widget, adw::Dialog, adw::NavigationPage, @implements gtk::Accessible;
}

impl AlbumPage {
    pub fn new(item: TuItem) -> Self {
        glib::Object::builder().property("item", item).build()
    }

    pub async fn set_album(&self) {
        let item = self.item();
        let image = get_image_with_cache(&item.id(), "Primary", None)
            .await
            .unwrap_or_default();
        self.imp()
            .cover_image
            .set_file(Some(&gtk::gio::File::for_path(image)));

        self.imp().title_label.set_text(&item.name());

        self.imp()
            .artist_label
            .set_text(&item.album_artist().unwrap_or(String::new()));

        self.imp()
            .released_label
            .set_text(&item.production_year().to_string());
    }
}