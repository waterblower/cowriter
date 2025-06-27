use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "Cowriter".into(),
            items: vec![
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏本窗口", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏其他窗口", super::HideOthers),
                #[cfg(target_os = "macos")]
                // MenuItem::action("Show All", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("退出程序", Quit),
            ],
        },
        Menu {
            name: "文件".into(),
            items: vec![
                MenuItem::action("新文件", workspace::NewFile),
                MenuItem::action("新窗口", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("打开文件...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "打开文件夹..."
                    } else {
                        "打开…"
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    "打开近期...",
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::separator(),
                MenuItem::action("保存", workspace::Save { save_intent: None }),
                MenuItem::action("保存为...", workspace::SaveAs),
                MenuItem::action("保存所有", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "关闭编辑器",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("关闭窗口", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "视图".into(),
            items: vec![
                MenuItem::action(
                    "放大",
                    zed_actions::IncreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "縮小",
                    zed_actions::DecreaseBufferFontSize { persist: true },
                ),
                MenuItem::action("重置", zed_actions::ResetBufferFontSize { persist: true }),
                MenuItem::separator(),
                MenuItem::action("左窗口", workspace::ToggleLeftDock),
                MenuItem::action("右窗口", workspace::ToggleRightDock),
                MenuItem::submenu(Menu {
                    name: "Editor Layout".into(),
                    items: vec![
                        MenuItem::action("左分屏", workspace::SplitLeft),
                        MenuItem::action("右分屏", workspace::SplitRight),
                    ],
                }),
            ],
        },
    ]
}
