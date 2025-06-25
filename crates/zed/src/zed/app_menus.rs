use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use terminal_view::terminal_panel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    vec![
        Menu {
            name: "写作助手".into(),
            items: vec![
                MenuItem::action("关于本软件", zed_actions::About),
                // MenuItem::action("Check for Updates", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Settings".into(),
                    items: vec![
                        MenuItem::action("Open Settings", super::OpenSettings),
                        MenuItem::action("Open Key Bindings", zed_actions::OpenKeymap),
                        MenuItem::action("Open Default Settings", super::OpenDefaultSettings),
                        MenuItem::action(
                            "Open Default Key Bindings",
                            zed_actions::OpenDefaultKeymap,
                        ),
                        MenuItem::action("Open Project Settings", super::OpenProjectSettings),
                        MenuItem::action(
                            "Select Theme...",
                            zed_actions::theme_selector::Toggle::default(),
                        ),
                    ],
                }),
                // MenuItem::separator(),
                // MenuItem::submenu(Menu {
                //     name: "Services".into(),
                //     items: vec![],
                // }),
                MenuItem::separator(),
                // MenuItem::action("Extensions", zed_actions::Extensions::default()),
                // MenuItem::action("Install CLI", install_cli::Install),
                // MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏本窗口", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("隐藏其他窗口", super::HideOthers),
                #[cfg(target_os = "macos")]
                // MenuItem::action("Show All", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Quit Zed", Quit),
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
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Undo", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("Redo", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("Cut", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("Copy", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("Copy and Trim", editor::actions::CopyAndTrim),
                MenuItem::os_action("Paste", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("Find", search::buffer_search::Deploy::find()),
                MenuItem::action("Find In Project", workspace::DeploySearch::find()),
                MenuItem::separator(),
                MenuItem::action(
                    "Toggle Line Comment",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action(
                    "Zoom In",
                    zed_actions::IncreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Zoom Out",
                    zed_actions::DecreaseBufferFontSize { persist: true },
                ),
                MenuItem::action(
                    "Reset Zoom",
                    zed_actions::ResetBufferFontSize { persist: true },
                ),
                MenuItem::separator(),
                MenuItem::action("Toggle Left Dock", workspace::ToggleLeftDock),
                MenuItem::action("Toggle Right Dock", workspace::ToggleRightDock),
                MenuItem::action("Toggle Bottom Dock", workspace::ToggleBottomDock),
                MenuItem::action("Close All Docks", workspace::CloseAllDocks),
                MenuItem::submenu(Menu {
                    name: "Editor Layout".into(),
                    items: vec![
                        MenuItem::action("Split Up", workspace::SplitUp),
                        MenuItem::action("Split Down", workspace::SplitDown),
                        MenuItem::action("Split Left", workspace::SplitLeft),
                        MenuItem::action("Split Right", workspace::SplitRight),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("Project Panel", project_panel::ToggleFocus),
                MenuItem::action("Outline Panel", outline_panel::ToggleFocus),
                MenuItem::action("Collab Panel", collab_panel::ToggleFocus),
                MenuItem::action("Terminal Panel", terminal_panel::ToggleFocus),
                MenuItem::separator(),
                MenuItem::action("Diagnostics", diagnostics::Deploy),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![
                MenuItem::action("Minimize", super::Minimize),
                MenuItem::action("Zoom", super::Zoom),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action(
                    "View Release Notes",
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action("View Telemetry", zed_actions::OpenTelemetryLog),
                MenuItem::action("View Dependency Licenses", zed_actions::OpenLicenses),
                MenuItem::action("Show Welcome", workspace::Welcome),
                MenuItem::action("Give Feedback...", zed_actions::feedback::GiveFeedback),
                MenuItem::separator(),
                MenuItem::action(
                    "Documentation",
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    "Zed Twitter",
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    "Join the Team",
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ]
}
