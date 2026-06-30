use dioxus::prelude::*;

use crate::i18n::{tx, use_lang, Tx};

const TITLE: Tx = tx("มาสร้างอะไรที่อยู่ยืนกันเถอะ", "Let's build something that lasts");
const BTN_MAIL: Tx = tx("ส่งอีเมลถึงฉัน", "Email me");
const EMAIL: &str = "enberkaych@gmail.com";
const UPDATED_TH: &str = concat!("อัพเดทล่าสุด ", env!("BUILD_DATE_TH"));
const UPDATED_EN: &str = concat!("Last updated ", env!("BUILD_DATE_EN"));

#[component]
pub fn Contact() -> Element {
    let lang = use_lang();
    let l = lang();

    rsx! {
        section { id: "contact",
            div { class: "contact-box",
                h2 { "{TITLE.t(l)}" }
                div { class: "contact-links",
                    a { class: "btn btn-primary", href: "mailto:{EMAIL}", "{BTN_MAIL.t(l)}" }
                    a {
                        class: "btn btn-ghost",
                        href: "https://github.com/levegaze",
                        target: "_blank",
                        "GitHub ↗"
                    }
                }
                p { class: "build-date",
                    if l == crate::i18n::Lang::Th { "{UPDATED_TH}" } else { "{UPDATED_EN}" }
                }
            }
        }
    }
}
