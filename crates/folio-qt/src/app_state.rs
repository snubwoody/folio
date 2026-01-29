use std::collections::HashMap;
use crate::{db_pool, RUNTIME};
use folio_lib::service::{Expense, fetch_expenses};
use qmetaobject::{QAbstractTableModel, QObject, qt_base_class, qt_method};
use qttypes::{QByteArray, QModelIndex, QString, QVariant};

// TODO: create a GlobalQObject
