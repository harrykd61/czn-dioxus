// src/main.rs

use dioxus::prelude::*;
mod certificate;
mod signing;
mod dispenser;
mod storage; // ← добавлено

use certificate::{CertificateInfo, find_certificates};
use signing::{sign_file_with_certificate, extract_attr};
use dispenser::{TaskStatusForUI};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "desktop")]
fn main() {
    // 🔽 Гарантируем создание .czn перед запуском UI
    if let Err(e) = crate::storage::ensure_czn_dir() {
        eprintln!("🚨 Не удалось инициализировать директорию приложения: {}", e);
        return;
    }

    dioxus::LaunchBuilder::desktop()
        .with_cfg(dioxus::desktop::Config::default().with_menu(None))
        .launch(App);
}

#[cfg(not(feature = "desktop"))]
fn main() {
    // Для web — storage не используется (пока)
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let certificates = use_resource(|| async move {
        find_certificates()
    });

    let mut tasks = use_signal(|| Vec::<TaskStatusForUI>::new());
    let mut loading_status = use_signal(|| false);

    use_future(move || async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        loop {
            loading_status.set(true);
            let statuses = dispenser::check_all_tasks().await;
            tasks.set(statuses);
            loading_status.set(false);

            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "min-h-screen bg-gray-900 text-white p-4",
            h1 { class: "text-2xl font-bold mb-6 text-center",
                "Электронные подписи в системе"
            }

            if tasks().len() > 0 {
                div { class: "mb-6 p-4 bg-blue-900/30 border border-blue-700 rounded-xl",
                    h2 { class: "text-lg font-semibold mb-3 flex items-center gap-2",
                        svg {
                            class: "w-5 h-5",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            d: "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                        }
                        "Статус выгрузок"
                    }
                    ul { class: "space-y-2 text-sm",
                        for task in tasks().iter() {
                            li { class: "flex items-center gap-3",
                                if task.is_completed {
                                    span { class: "text-green-400", "✅" }
                                    span { class: "font-medium text-green-100",
                                        "Готово: {task.display_name()}"
                                    }
                                } else if task.error.is_some() {
                                    span { class: "text-red-400", "❌" }
                                    {
                                        let error_msg = task.error.as_deref().unwrap_or("-");
                                        rsx! {
                                            span { class: "text-red-100", "Ошибка {task.display_name()}: {error_msg}" }
                                        }
                                    }
                                } else {
                                    span { class: "text-yellow-400", "⏳" }
                                    span { class: "text-yellow-100",
                                        "В обработке: {task.display_name()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if loading_status() && tasks().is_empty() {
                div { class: "mb-6 p-4 bg-gray-800 border border-gray-600 rounded-xl text-center",
                    "Проверка статуса выгрузок..."
                }
            }

            match certificates() {
                Some(certs) => rsx! {
                    CertificateSection { certificates: certs.clone() }
                },
                None => rsx! {
                    div { class: "text-center py-8", "Загрузка сертификатов..." }
                },
            }
        }
    }
}

#[component]
fn CertificateSection(certificates: Vec<CertificateInfo>) -> Element {
    let mut search_query = use_signal(|| String::new());
    let mut selected_cert = use_signal(|| Option::<CertificateInfo>::None);
    let mut sign_status = use_signal(|| Option::<String>::None);
    let mut loading = use_signal(|| false);

    let filtered_certs = use_memo(move || {
        if search_query().is_empty() {
            certificates.clone()
        } else {
            certificates
                .iter()
                .filter(|cert| cert.subject_name.to_lowercase().contains(&search_query().to_lowercase()))
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    let certs = filtered_certs().into_iter().take(6).collect::<Vec<_>>();

    rsx! {
        div { class: "space-y-6",
            div { class: "mb-6",
                input {
                    class: "w-full p-3 rounded bg-gray-800 text-white border border-gray-700 focus:outline-none focus:border-blue-500",
                    placeholder: "Поиск по сертификатам...",
                    value: search_query(),
                    oninput: move |e| search_query.set(e.value()),
                }
                p { class: "text-sm text-gray-400 mt-2",
                    "Найдено: {filtered_certs().len()} сертификатов"
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-3 lg:grid-cols-3 gap-6",
                for cert in certs {
                    div {
                        class: "relative overflow-hidden rounded-2xl border border-gray-700 bg-gradient-to-br from-gray-800/90 via-gray-800 to-gray-900 p-5 shadow-xl transition-transform duration-200 hover:-translate-y-1 hover:border-blue-500/70 hover:shadow-blue-900/30 whitespace-normal break-words cursor-pointer",
                        onclick: move |_| {
                            if loading() {
                                return;
                            }
                            selected_cert.set(Some(cert.clone()));
                            sign_status.set(None);
                            loading.set(true);
                            let cert_clone = cert.clone();
                            spawn(async move {
                                match sign_file_with_certificate(&cert_clone).await {
                                    Ok(message) => {
                                        sign_status.set(Some(message));
                                    }
                                    Err(error) => {
                                        sign_status.set(Some(format!("Ошибка: {}", error)));
                                    }
                                }
                                loading.set(false);
                            });
                        },
                        div { class: "space-y-1",
                            {
                                let cn_node = extract_attr(&cert.subject_name, "CN=")
                                    .map(|cn| {
                                        rsx! {
                                            p { class: "text-white font-semibold text-base", "{cn}" }
                                        }
                                    });
                                let sn_node = extract_attr(&cert.subject_name, "SN=")
                                    .map(|sn| {
                                        rsx! {
                                            p { class: "text-white text-base", "{sn}" }
                                        }
                                    });
                                let g_node = extract_attr(&cert.subject_name, "G=")
                                    .map(|g| {
                                        rsx! {
                                            p { class: "text-white text-base", "{g}" }
                                        }
                                    });
                                let fallback_node = (!cn_node.is_some() && !sn_node.is_some()
                                    && !g_node.is_some())
                                    .then(|| {
                                        let fallback = cert
                                            .subject_name
                                            .split(',')
                                            .next()
                                            .unwrap_or(&cert.subject_name);
                                        rsx! {
                                            p { class: "text-white font-semibold text-base", "{fallback}" }
                                        }
                                    });
                                rsx! {
                                    {cn_node}
                                    {sn_node}
                                    {g_node}
                                    {fallback_node}
                                }
                            }
                            {
                                if let Some(inn) = extract_attr(&cert.subject_name, "INN=") {
                                    let is_company = extract_attr(&cert.subject_name, "O=").is_some()
                                        || extract_attr(&cert.subject_name, "OU=").is_some();
                                    if is_company { Some(rsx! {
                                        p { class: "text-blue-300 text-sm", "ИНН: {inn}" }
                                    }) } else { None }
                                } else {
                                    None
                                }
                            }
                        }
                    }
                }
            }

            if let Some(msg) = sign_status() {
                div { class: "rounded-xl border border-blue-700/50 bg-blue-900/20 text-blue-100 px-4 py-3 text-sm shadow-inner",
                    "{msg}"
                }
            }

            if loading() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-gray-800 rounded-lg p-6 flex flex-col items-center space-y-4",
                        svg {
                            class: "animate-spin h-10 w-10 text-blue-500",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            circle {
                                class: "opacity-25",
                                stroke: "currentColor",
                                stroke_width: "4",
                                r: "10",
                                cx: "12",
                                cy: "12",
                                stroke_linecap: "round",
                            }
                            path {
                                class: "opacity-75",
                                fill: "currentColor",
                                d: "M4 12a8 8 0 018-8V4a10 10 0 00-10 10h2z",
                            }
                        }
                        p { class: "text-white text-lg font-medium",
                            "Подготовка и подпись..."
                        }
                    }
                }
            }
        }
    }
}
