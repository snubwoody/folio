use crate::{RUNTIME, db_pool};
use folio_lib::service::{Account, EditAccount, fetch_accounts, fetch_categories, Category};
use qmetaobject::{QAbstractListModel, QObject, qt_base_class, qt_method};
use qttypes::{QByteArray, QString, QVariant};
use std::collections::HashMap;

#[derive(QObject, Default)]
pub struct CategoryListModel {
    base: qt_base_class!(trait QAbstractListModel),
    load_data: qt_method!(fn(&mut self)),
    delete_category: qt_method!(fn(&mut self, id: String)),
    edit_category: qt_method!(fn(&mut self, id: String,title: String)),
    add_category: qt_method!(fn(&mut self)),
    categories: Vec<Category>,
}

impl CategoryListModel {
    const TITLE_ROLE: i32 = qmetaobject::USER_ROLE + 1;
    const ID_ROLE: i32 = Self::TITLE_ROLE + 1;

    pub fn new() -> Self {
        let categories = RUNTIME.block_on(async {
            // TODO: log error
            fetch_categories(db_pool()).await.unwrap()
        });
        Self {
            categories,
            ..Default::default()
        }
    }

    pub fn load_data(&mut self) {
        self.begin_reset_model();
        self.categories = RUNTIME.block_on(async { fetch_categories(db_pool()).await.unwrap() });
        self.end_reset_model();
    }

    fn delete_category(&mut self,id: String){
        RUNTIME.block_on(async { Category::delete(&id,db_pool()).await.unwrap() });
        self.load_data();
    }

    fn edit_category(&mut self,id: String,title: String){
        RUNTIME.block_on(async { Category::edit(&id,&title,db_pool()).await.unwrap() });
        self.load_data();
    }

    fn add_category(&mut self){
        RUNTIME.block_on(async { Category::create("New category",db_pool()).await.unwrap() });
        self.load_data();
    }


    // pub fn add_account(&mut self, name: QString, balance: QString) {
    //     let name = name.to_string();
    //     let balance = Money::from_str(balance.to_string().as_str()).unwrap_or_default();
    //     RUNTIME.block_on(async { Account::create(&name, balance, db_pool()).await.unwrap() });
    //     self.load_accounts();
    // }
}

impl QAbstractListModel for CategoryListModel {
    fn row_count(&self) -> i32 {
        self.categories.len() as i32
    }

    fn data(&self, index: qttypes::QModelIndex, role: i32) -> QVariant {
        let index = index.row() as usize;
        if index >= self.categories.len() {
            return QVariant::default();
        }

        let category = &self.categories[index];

        match role {
            Self::ID_ROLE => QString::from(category.id.clone()).into(),
            Self::TITLE_ROLE => QString::from(category.title.clone()).into(),
            _ => QVariant::default(),
        }
    }

    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut roles = HashMap::new();
        roles.insert(Self::ID_ROLE, QByteArray::from("id"));
        roles.insert(Self::TITLE_ROLE, QByteArray::from("title"));
        roles
    }
}
