use cstr::cstr;
use qmetaobject::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Catalog {
    FullIconThemes,
    Cursors,
    KDEThemes,
}

impl Catalog {
    pub fn get_id(&self) -> &str {
        match self {
            Catalog::FullIconThemes => "132",
            Catalog::Cursors => "107",
            Catalog::KDEThemes => "104",
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Catalog::FullIconThemes => "Full Icon Themes",
            Catalog::Cursors => "Cursor Themes",
            Catalog::KDEThemes => "KDE Themes",
        }
    }

    pub fn get_all_catalog_types() -> Vec<Catalog> {
        vec![Catalog::FullIconThemes, Catalog::Cursors, Catalog::KDEThemes]
    }
}

fn main() {
    let mut engine = QmlEngine::new();
    engine.load_data(
        r#"
        import QtQuick 2.6
        import QtQuick.Controls 2.0

        ApplicationWindow {
            visible: true
            width: 640
            height: 480
            title: qsTr("Linux Theme Store - Qt")

            Text {
                anchors.centerIn: parent
                text: "Qt port in progress..."
            }
        }
        "#
            .into(),
    );
    engine.exec();
}
