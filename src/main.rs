// src/main.rs

use dioxus::prelude::*;
mod certificate;
mod signing;

use certificate::{CertificateInfo, find_certificates};
use signing::sign_file_with_certificate;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[cfg(feature = "desktop")]
fn main() {
    use dioxus::desktop::Config;

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::default().with_menu(None))
        .launch(App);
}

#[cfg(not(feature = "desktop"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let certificates = use_resource(|| async move {
        find_certificates()
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gray-900 text-white p-4",
            h1 { class: "text-2xl font-bold mb-6 text-center",
                "Электронные подписи в системе"
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

    // ✅ .cloned().take(6).collect() → сами объекты, не ссылки
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
                // ✅ cert — уже владеющий объект
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
                        div { class: "absolute inset-x-0 top-0 h-1 bg-gradient-to-r from-blue-500 via-cyan-400 to-blue-500" }
                        h4 { class: "text-lg font-semibold text-white mb-3", "{cert.issuer_name}" }
                        div { class: "space-y-1 text-xs text-gray-300",
                            p { class: "font-medium",
                                "{cert.subject_name.split(',').next().unwrap_or(&cert.subject_name)}"
                            }
                            p { class: "text-gray-400",
                                "Срок: {cert.valid_from} – {cert.valid_to}"
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
