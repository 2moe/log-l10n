// Version: 0.0.0
#![allow(dead_code)]

use super::{lang_id_consts, HashMap, LangID};

pub(crate) type L10nMap = ::phf::Map<&'static str, &'static str>;
pub(crate) type SubLocaleMap = ::phf::Map<&'static str, fn() -> L10nMap>;
pub(crate) type LocaleMap = ::phf::Map<&'static str, fn() -> SubLocaleMap>;
pub(crate) type LocaleHashMap = HashMap<LangID, SubLocaleMap>;

/// Language ID: af;
/// Map name: "log-core";
/// Description: Afrikaans, Latyn, Suid-Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ontfout");
/// ```
pub(super) const fn get_af_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ontfout"##),
        ("warn", r##"Waarsku"##),
        ("info", r##"Inligting"##),
        ("error", r##"Fout"##),
        ("trace", r##"Spoor"##),
        ("init-logger", r##"Inisialisering van logger ..."##),
    ],
}
}

/// af: Afrikaans, Latyn, Suid-Afrika
pub(super) const fn get_af_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_af_map_log_core),
    ],
}
}

/// Language ID: am;
/// Map name: "log-core";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "error");
///
/// assert_eq!(msg, "ስህተት");
/// ```
pub(super) const fn get_am_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 10121458955350035957,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("trace", r##"ዱካ"##),
        ("info", r##"መረጃ"##),
        ("init-logger", r##"የመመዝገቢያ ማስጀመር ..."##),
        ("error", r##"ስህተት"##),
        ("warn", r##"አስጠንቅቀዋል"##),
    ],
}
}

/// am: አማርኛ, ኢትዮፒክ, ኢትዮጵያ
pub(super) const fn get_am_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_am_map_log_core),
    ],
}
}

/// Language ID: ar;
/// Map name: "log-core";
/// Description: العربية, العربية, مصر;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "تصحيح");
/// ```
pub(super) const fn get_ar_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"تصحيح"##),
        ("warn", r##"تحذير"##),
        ("info", r##"المعلومات"##),
        ("error", r##"خطأ"##),
        ("trace", r##"تتبع"##),
        ("init-logger", r##"تهيئة المسجل ..."##),
    ],
}
}

/// ar: العربية, العربية, مصر
pub(super) const fn get_ar_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ar_map_log_core),
    ],
}
}

/// Language ID: az;
/// Map name: "log-core";
/// Description: azərbaycan, latın, Azərbaycan;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_az_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"xəbərdarlıq"##),
        ("info", r##"Məlumat"##),
        ("error", r##"səhv"##),
        ("trace", r##"iz"##),
        ("init-logger", r##"Çıxışçılığı başlatmaq ..."##),
    ],
}
}

/// az: azərbaycan, latın, Azərbaycan
pub(super) const fn get_az_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_az_map_log_core),
    ],
}
}

/// Language ID: be;
/// Map name: "log-core";
/// Description: беларуская, кірыліца, Беларусь;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "адладка");
/// ```
pub(super) const fn get_be_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"адладка"##),
        ("warn", r##"папярэджвайце"##),
        ("info", r##"Інфармацыя"##),
        ("error", r##"Памылка"##),
        ("trace", r##"след"##),
        ("init-logger", r##"Ініцыялізацыя рэгістратара ..."##),
    ],
}
}

/// be: беларуская, кірыліца, Беларусь
pub(super) const fn get_be_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_be_map_log_core),
    ],
}
}

/// Language ID: bg;
/// Map name: "log-core";
/// Description: български, кирилица, България;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "отстраняване на грешки");
/// ```
pub(super) const fn get_bg_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"отстраняване на грешки"##),
        ("warn", r##"Предупреждение"##),
        ("info", r##"информация"##),
        ("error", r##"Грешка"##),
        ("trace", r##"следа"##),
        ("init-logger", r##"Инициализиране на дърводобив ..."##),
    ],
}
}

/// bg: български, кирилица, България
pub(super) const fn get_bg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_bg_map_log_core),
    ],
}
}

/// Language ID: bn;
/// Map name: "log-core";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ডিব\u{9be}গ");
/// ```
pub(super) const fn get_bn_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ডিবাগ"##),
        ("warn", r##"সতর্কতা"##),
        ("info", r##"তথ্য"##),
        ("error", r##"ত্রুটি"##),
        ("trace", r##"ট্রেস"##),
        ("init-logger", r##"লগার আরম্ভ করা হচ্ছে..."##),
    ],
}
}

/// bn: বাংলা, বাংলা, বাংলাদেশ
pub(super) const fn get_bn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_bn_map_log_core),
    ],
}
}

/// Language ID: bs;
/// Map name: "log-core";
/// Description: bosanski, latinica, Bosna i Hercegovina;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Ispravljanje pogrešaka");
/// ```
pub(super) const fn get_bs_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Ispravljanje pogrešaka"##),
        ("warn", r##"upozori"##),
        ("info", r##"Info"##),
        ("error", r##"greška"##),
        ("trace", r##"trag"##),
        ("init-logger", r##"Inicijaliziranje drvosječa ..."##),
    ],
}
}

/// bs: bosanski, latinica, Bosna i Hercegovina
pub(super) const fn get_bs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_bs_map_log_core),
    ],
}
}

/// Language ID: ca;
/// Map name: "log-core";
/// Description: català, llatí, Espanya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "depuració");
/// ```
pub(super) const fn get_ca_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"depuració"##),
        ("warn", r##"adverteix"##),
        ("info", r##"Info"##),
        ("error", r##"Error"##),
        ("trace", r##"TRACE"##),
        ("init-logger", r##"Inicialització del registre ..."##),
    ],
}
}

/// ca: català, llatí, Espanya
pub(super) const fn get_ca_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ca_map_log_core),
    ],
}
}

/// Language ID: ceb;
/// Map name: "log-core";
/// Description: Cebuano, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_ceb_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"Pasidaan"##),
        ("info", r##"impormasyon"##),
        ("error", r##"sayup"##),
        ("trace", r##"pagsubay"##),
        ("init-logger", r##"Pag-una sa logger ..."##),
    ],
}
}

/// ceb: Cebuano, Latin, Pilipinas
pub(super) const fn get_ceb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ceb_map_log_core),
    ],
}
}

/// Language ID: co;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_co_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"avisa"##),
        ("info", r##"info"##),
        ("error", r##"errore"##),
        ("trace", r##"traccia"##),
        ("init-logger", r##"Inizializà Logger ..."##),
    ],
}
}

/// co: co-Latn-FR
pub(super) const fn get_co_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_co_map_log_core),
    ],
}
}

/// Language ID: cs;
/// Map name: "log-core";
/// Description: čeština, latinka, Česko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_cs_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"VAROVÁNÍ"##),
        ("info", r##"INFORMACE"##),
        ("error", r##"CHYBA"##),
        ("trace", r##"STOPA"##),
        ("init-logger", r##"Inicializuji logger..."##),
    ],
}
}

/// cs: čeština, latinka, Česko
pub(super) const fn get_cs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_cs_map_log_core),
    ],
}
}

/// Language ID: cy;
/// Map name: "log-core";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Dadfygio");
/// ```
pub(super) const fn get_cy_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Dadfygio"##),
        ("warn", r##"Rhybudd"##),
        ("info", r##"Gwybodaeth"##),
        ("error", r##"Wallau"##),
        ("trace", r##"olrhain"##),
        ("init-logger", r##"Cychwyn Logger ..."##),
    ],
}
}

/// cy: Cymraeg, Lladin, Y Deyrnas Unedig
pub(super) const fn get_cy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_cy_map_log_core),
    ],
}
}

/// Language ID: da;
/// Map name: "log-core";
/// Description: dansk, latinsk, Danmark;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_da_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"advare"##),
        ("info", r##"info"##),
        ("error", r##"fejl"##),
        ("trace", r##"Spor"##),
        ("init-logger", r##"Initialisering af logger ..."##),
    ],
}
}

/// da: dansk, latinsk, Danmark
pub(super) const fn get_da_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_da_map_log_core),
    ],
}
}

/// Language ID: de;
/// Map name: "log-core";
/// Description: Deutsch, Lateinisch, Deutschland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_de_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"WARNUNG"##),
        ("info", r##"INFO"##),
        ("error", r##"FEHLER"##),
        ("trace", r##"SPUR"##),
        ("init-logger", r##"Logger wird initialisiert ..."##),
    ],
}
}

/// de: Deutsch, Lateinisch, Deutschland
pub(super) const fn get_de_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_de_map_log_core),
    ],
}
}

/// Language ID: el;
/// Map name: "log-core";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "σφαλμάτων");
/// ```
pub(super) const fn get_el_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"σφαλμάτων"##),
        ("warn", r##"προειδοποιήστε"##),
        ("info", r##"Πληροφορίες"##),
        ("error", r##"Σφάλμα"##),
        ("trace", r##"ίχνος"##),
        ("init-logger", r##"Αρχικοποίηση του καταγραφέα ..."##),
    ],
}
}

/// el: Ελληνικά, Ελληνικό, Ελλάδα
pub(super) const fn get_el_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_el_map_log_core),
    ],
}
}

/// Language ID: en;
/// Map name: "log-core";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_en_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"WARN"##),
        ("info", r##"INFO"##),
        ("error", r##"ERROR"##),
        ("trace", r##"TRACE"##),
        ("init-logger", r##"Initializing logger ..."##),
    ],
}
}

/// en: English, Latin, United States
pub(super) const fn get_en_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_en_map_log_core),
    ],
}
}

/// Language ID: en-GB;
/// Map name: "log-core";
/// Description: English, Latin, United Kingdom;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "init-logger");
///
/// assert_eq!(msg, "Initialising logger ...");
/// ```
pub(super) const fn get_en_gb_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("init-logger", r##"Initialising logger ..."##),
    ],
}
}

/// en-GB: English, Latin, United Kingdom
pub(super) const fn get_en_gb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_en_gb_map_log_core),
    ],
}
}

/// Language ID: eo;
/// Map name: "log-core";
/// Description: esperanto, Latn, Mondo;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_eo_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"avertu"##),
        ("info", r##"Info"##),
        ("error", r##"eraro"##),
        ("trace", r##"spuro"##),
        ("init-logger", r##"Inicializanta logger ..."##),
    ],
}
}

/// eo: esperanto, Latn, Mondo
pub(super) const fn get_eo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_eo_map_log_core),
    ],
}
}

/// Language ID: es;
/// Map name: "log-core";
/// Description: español, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEPURACIÓN");
/// ```
pub(super) const fn get_es_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEPURACIÓN"##),
        ("warn", r##"ADVERTENCIA"##),
        ("info", r##"INFORMACIÓN"##),
        ("error", r##"ERROR"##),
        ("trace", r##"TRAZA"##),
        ("init-logger", r##"Inicializando el registro..."##),
    ],
}
}

/// es: español, latino, España
pub(super) const fn get_es_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_es_map_log_core),
    ],
}
}

/// Language ID: et;
/// Map name: "log-core";
/// Description: eesti, ladina, Eesti;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "silumine");
/// ```
pub(super) const fn get_et_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"silumine"##),
        ("warn", r##"hoiatab"##),
        ("info", r##"teave"##),
        ("error", r##"viga"##),
        ("trace", r##"jälg"##),
        ("init-logger", r##"Logija initsialiseerimine ..."##),
    ],
}
}

/// et: eesti, ladina, Eesti
pub(super) const fn get_et_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_et_map_log_core),
    ],
}
}

/// Language ID: eu;
/// Map name: "log-core";
/// Description: euskara, latinoa, Espainia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "arazketa");
/// ```
pub(super) const fn get_eu_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"arazketa"##),
        ("warn", r##"ohartarazi"##),
        ("info", r##"informazioa"##),
        ("error", r##"errorea"##),
        ("trace", r##"arrastoa"##),
        ("init-logger", r##"Logger hasieran ..."##),
    ],
}
}

/// eu: euskara, latinoa, Espainia
pub(super) const fn get_eu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_eu_map_log_core),
    ],
}
}

/// Language ID: fa;
/// Map name: "log-core";
/// Description: فارسی, عربی, ایران;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "اشکال زدایی");
/// ```
pub(super) const fn get_fa_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"اشکال زدایی"##),
        ("warn", r##"هشدار دهید"##),
        ("info", r##"اطلاعات"##),
        ("error", r##"خطا"##),
        ("trace", r##"ردیابی"##),
        ("init-logger", r##"اولیه سازی logger ..."##),
    ],
}
}

/// fa: فارسی, عربی, ایران
pub(super) const fn get_fa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_fa_map_log_core),
    ],
}
}

/// Language ID: fi;
/// Map name: "log-core";
/// Description: suomi, latinalainen, Suomi;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "virheenkorjaus");
/// ```
pub(super) const fn get_fi_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"virheenkorjaus"##),
        ("warn", r##"varoita"##),
        ("info", r##"tiedot"##),
        ("error", r##"virhe"##),
        ("trace", r##"Jälki"##),
        ("init-logger", r##"Loggerin alustaminen ..."##),
    ],
}
}

/// fi: suomi, latinalainen, Suomi
pub(super) const fn get_fi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_fi_map_log_core),
    ],
}
}

/// Language ID: fil;
/// Map name: "log-core";
/// Description: Filipino, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBAG");
/// ```
pub(super) const fn get_fil_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBAG"##),
        ("warn", r##"BABALA"##),
        ("info", r##"IMPORMASYON"##),
        ("error", r##"ERROR"##),
        ("trace", r##"TUMUTULONG"##),
        ("init-logger", r##"Pagsisimula ng logger..."##),
    ],
}
}

/// fil: Filipino, Latin, Pilipinas
pub(super) const fn get_fil_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_fil_map_log_core),
    ],
}
}

/// Language ID: fr;
/// Map name: "log-core";
/// Description: français, latin, France;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DÉBOGAGE");
/// ```
pub(super) const fn get_fr_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DÉBOGAGE"##),
        ("warn", r##"AVERTISSEMENT"##),
        ("info", r##"INFORMATIONS"##),
        ("error", r##"ERREUR"##),
        ("trace", r##"TRACER"##),
        ("init-logger", r##"Initialisation du journal..."##),
    ],
}
}

/// fr: français, latin, France
pub(super) const fn get_fr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_fr_map_log_core),
    ],
}
}

/// Language ID: fy;
/// Map name: "log-core";
/// Description: Frysk, Latyn, Nederlân;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_fy_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"warskôgje"##),
        ("info", r##"ynfo"##),
        ("error", r##"flater"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Inisjalisearjen fan logger ..."##),
    ],
}
}

/// fy: Frysk, Latyn, Nederlân
pub(super) const fn get_fy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_fy_map_log_core),
    ],
}
}

/// Language ID: ga;
/// Map name: "log-core";
/// Description: Gaeilge, Laidineach, Éire;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "dífhabhtaithe");
/// ```
pub(super) const fn get_ga_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"dífhabhtaithe"##),
        ("warn", r##"rabhadh"##),
        ("info", r##"Eolas"##),
        ("error", r##"Earráid"##),
        ("trace", r##"rian"##),
        ("init-logger", r##"Tús a chur le logálaí ..."##),
    ],
}
}

/// ga: Gaeilge, Laidineach, Éire
pub(super) const fn get_ga_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ga_map_log_core),
    ],
}
}

/// Language ID: gd;
/// Map name: "log-core";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "deasbad");
/// ```
pub(super) const fn get_gd_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"deasbad"##),
        ("warn", r##"rabhadh"##),
        ("info", r##"fiosrachadh"##),
        ("error", r##"mearachd"##),
        ("trace", r##"lorg"##),
        ("init-logger", r##"A 'tòiseachadh logger ..."##),
    ],
}
}

/// gd: Gàidhlig, Laideann, An Rìoghachd Aonaichte
pub(super) const fn get_gd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_gd_map_log_core),
    ],
}
}

/// Language ID: gl;
/// Map name: "log-core";
/// Description: galego, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Depuración");
/// ```
pub(super) const fn get_gl_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Depuración"##),
        ("warn", r##"ADVERTIR"##),
        ("info", r##"información"##),
        ("error", r##"Erro"##),
        ("trace", r##"rastrexo"##),
        ("init-logger", r##"Inicializar o rexistro ..."##),
    ],
}
}

/// gl: galego, latino, España
pub(super) const fn get_gl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_gl_map_log_core),
    ],
}
}

/// Language ID: gu;
/// Map name: "log-core";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ડિબગ");
/// ```
pub(super) const fn get_gu_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ડિબગ"##),
        ("warn", r##"ચેતવણી"##),
        ("info", r##"માહિતી"##),
        ("error", r##"ભૂલ"##),
        ("trace", r##"ટ્રેસ"##),
        ("init-logger", r##"પ્રારંભિક લોગર ..."##),
    ],
}
}

/// gu: ગુજરાતી, ગુજરાતી, ભારત
pub(super) const fn get_gu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_gu_map_log_core),
    ],
}
}

/// Language ID: ha;
/// Map name: "log-core";
/// Description: Hausa, Latin, Nijeriya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_ha_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"gargadi"##),
        ("info", r##"Bayani"##),
        ("error", r##"kuskure"##),
        ("trace", r##"gano"##),
        ("init-logger", r##"Fara logger ..."##),
    ],
}
}

/// ha: Hausa, Latin, Nijeriya
pub(super) const fn get_ha_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ha_map_log_core),
    ],
}
}

/// Language ID: haw;
/// Map name: "log-core";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_haw_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"Haeō"##),
        ("info", r##"ʻike"##),
        ("error", r##"Haki"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Ke hoʻomakaʻana me ka logger ..."##),
    ],
}
}

/// haw: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa
pub(super) const fn get_haw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_haw_map_log_core),
    ],
}
}

/// Language ID: he;
/// Map name: "log-core";
/// Description: עברית, עברי, ישראל;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ניפוי באגים");
/// ```
pub(super) const fn get_he_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ניפוי באגים"##),
        ("warn", r##"להזהיר"##),
        ("info", r##"מידע"##),
        ("error", r##"שגיאה"##),
        ("trace", r##"עקבות"##),
        ("init-logger", r##"אתחול לוגר ..."##),
    ],
}
}

/// he: עברית, עברי, ישראל
pub(super) const fn get_he_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_he_map_log_core),
    ],
}
}

/// Language ID: hi;
/// Map name: "log-core";
/// Description: हिन्दी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "डिबग");
/// ```
pub(super) const fn get_hi_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"डिबग"##),
        ("warn", r##"चेतावनी"##),
        ("info", r##"जानकारी"##),
        ("error", r##"त्रुटि"##),
        ("trace", r##"ट्रेस"##),
        ("init-logger", r##"आरंभिक लकड़हारा ..."##),
    ],
}
}

/// hi: हिन्दी, देवनागरी, भारत
pub(super) const fn get_hi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_hi_map_log_core),
    ],
}
}

/// Language ID: hr;
/// Map name: "log-core";
/// Description: hrvatski, latinica, Hrvatska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "uklanjanje pogrešaka");
/// ```
pub(super) const fn get_hr_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"uklanjanje pogrešaka"##),
        ("warn", r##"upozoriti"##),
        ("info", r##"Informacije"##),
        ("error", r##"Pogreška"##),
        ("trace", r##"PRACE"##),
        ("init-logger", r##"Inicijaliziranje zapisnika ..."##),
    ],
}
}

/// hr: hrvatski, latinica, Hrvatska
pub(super) const fn get_hr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_hr_map_log_core),
    ],
}
}

/// Language ID: ht;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_ht_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"avèti"##),
        ("info", r##"Info"##),
        ("error", r##"Erè"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Inisyalize anrejistrè ..."##),
    ],
}
}

/// ht: ht-Latn-HT
pub(super) const fn get_ht_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ht_map_log_core),
    ],
}
}

/// Language ID: hu;
/// Map name: "log-core";
/// Description: magyar, Latin, Magyarország;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "hibakeresési");
/// ```
pub(super) const fn get_hu_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"hibakeresési"##),
        ("warn", r##"Figyelmeztetés"##),
        ("info", r##"Info"##),
        ("error", r##"hiba"##),
        ("trace", r##"nyomkövetés"##),
        ("init-logger", r##"A fakitermelő inicializálása ..."##),
    ],
}
}

/// hu: magyar, Latin, Magyarország
pub(super) const fn get_hu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_hu_map_log_core),
    ],
}
}

/// Language ID: hy;
/// Map name: "log-core";
/// Description: հայերեն, հայկական, Հայաստան;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "կարգաբերում");
/// ```
pub(super) const fn get_hy_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"կարգաբերում"##),
        ("warn", r##"նախազգուշացում"##),
        ("info", r##"Տեղեկություն"##),
        ("error", r##"սխալ"##),
        ("trace", r##"հետք"##),
        ("init-logger", r##"Նախաձեռնող անտառահատ ..."##),
    ],
}
}

/// hy: հայերեն, հայկական, Հայաստան
pub(super) const fn get_hy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_hy_map_log_core),
    ],
}
}

/// Language ID: id;
/// Map name: "log-core";
/// Description: Indonesia, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_id_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"memperingatkan"##),
        ("info", r##"info"##),
        ("error", r##"kesalahan"##),
        ("trace", r##"jejak"##),
        ("init-logger", r##"Menginisialisasi logger ..."##),
    ],
}
}

/// id: Indonesia, Latin, Indonesia
pub(super) const fn get_id_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_id_map_log_core),
    ],
}
}

/// Language ID: ig;
/// Map name: "log-core";
/// Description: Igbo, Latin, Naịjịrịa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_ig_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"dọrọ aka na ntị"##),
        ("info", r##"info"##),
        ("error", r##"njehie"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Mgbakwunye Ndekọ ..."##),
    ],
}
}

/// ig: Igbo, Latin, Naịjịrịa
pub(super) const fn get_ig_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ig_map_log_core),
    ],
}
}

/// Language ID: is;
/// Map name: "log-core";
/// Description: íslenska, latneskt, Ísland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "kembiforrit");
/// ```
pub(super) const fn get_is_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"kembiforrit"##),
        ("warn", r##"vara við"##),
        ("info", r##"upplýsingar"##),
        ("error", r##"villa"##),
        ("trace", r##"rekja"##),
        ("init-logger", r##"Frumstilla skógarhöggsmann ..."##),
    ],
}
}

/// is: íslenska, latneskt, Ísland
pub(super) const fn get_is_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_is_map_log_core),
    ],
}
}

/// Language ID: it;
/// Map name: "log-core";
/// Description: italiano, latino, Italia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_it_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"AVVERTIMENTO"##),
        ("info", r##"INFORMAZIONE"##),
        ("error", r##"ERRORE"##),
        ("trace", r##"TRACCIA"##),
        ("init-logger", r##"Inizializzazione del logger..."##),
    ],
}
}

/// it: italiano, latino, Italia
pub(super) const fn get_it_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_it_map_log_core),
    ],
}
}

/// Language ID: iw;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ניפוי באגים");
/// ```
pub(super) const fn get_iw_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ניפוי באגים"##),
        ("warn", r##"להזהיר"##),
        ("info", r##"מידע"##),
        ("error", r##"שגיאה"##),
        ("trace", r##"עקבות"##),
        ("init-logger", r##"אתחול לוגר ..."##),
    ],
}
}

/// iw: iw-Hebr-IL
pub(super) const fn get_iw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_iw_map_log_core),
    ],
}
}

/// Language ID: ja;
/// Map name: "log-core";
/// Description: 日本語, 日本語の文字, 日本;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "デバッグ");
/// ```
pub(super) const fn get_ja_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"デバッグ"##),
        ("warn", r##"警告"##),
        ("info", r##"情報"##),
        ("error", r##"エラー"##),
        ("trace", r##"トレース"##),
        ("init-logger", r##"ロガーの初期化..."##),
    ],
}
}

/// ja: 日本語, 日本語の文字, 日本
pub(super) const fn get_ja_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ja_map_log_core),
    ],
}
}

/// Language ID: jw;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_jw_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"ngelingake"##),
        ("info", r##"Info"##),
        ("error", r##"Kesalahan"##),
        ("trace", r##"tilak"##),
        ("init-logger", r##"Inisialisasi logger ..."##),
    ],
}
}

/// jw: jw-Latn-ID
pub(super) const fn get_jw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_jw_map_log_core),
    ],
}
}

/// Language ID: ka;
/// Map name: "log-core";
/// Description: ქართული, ქართული, საქართველო;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "გამართვა");
/// ```
pub(super) const fn get_ka_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"გამართვა"##),
        ("warn", r##"აფრთხილებს"##),
        ("info", r##"ინფორმაცია"##),
        ("error", r##"შეცდომა"##),
        ("trace", r##"კვალი"##),
        ("init-logger", r##"ლოგინის ინიციალიზაცია ..."##),
    ],
}
}

/// ka: ქართული, ქართული, საქართველო
pub(super) const fn get_ka_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ka_map_log_core),
    ],
}
}

/// Language ID: kk;
/// Map name: "log-core";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Атқару");
/// ```
pub(super) const fn get_kk_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Атқару"##),
        ("warn", r##"ескертіңіз"##),
        ("info", r##"ақпарат"##),
        ("error", r##"қате"##),
        ("trace", r##"із"##),
        ("init-logger", r##"Бастауды бастау ..."##),
    ],
}
}

/// kk: қазақ тілі, кирилл жазуы, Қазақстан
pub(super) const fn get_kk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_kk_map_log_core),
    ],
}
}

/// Language ID: km;
/// Map name: "log-core";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ប\u{17c6}បាត\u{17cb}ក\u{17c6}ហ\u{17bb}ស");
/// ```
pub(super) const fn get_km_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"បំបាត់កំហុស"##),
        ("warn", r##"ព្រមាន"##),
        ("info", r##"ព័ត៌មាន"##),
        ("error", r##"កំហុស"##),
        ("trace", r##"ដាន"##),
        ("init-logger", r##"ចាប់ផ្តើមអ្នកកាប់ឈើ ..."##),
    ],
}
}

/// km: ខ្មែរ, ខ្មែរ, កម្ពុជា
pub(super) const fn get_km_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_km_map_log_core),
    ],
}
}

/// Language ID: kn;
/// Map name: "log-core";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ಡೀಬಗ\u{ccd}");
/// ```
pub(super) const fn get_kn_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ಡೀಬಗ್"##),
        ("warn", r##"ಎಚ್ಚರಿಸಿ"##),
        ("info", r##"ಮಾಹಿತಿ"##),
        ("error", r##"ದೋಷ"##),
        ("trace", r##"ಜಾಡಿನ"##),
        ("init-logger", r##"ಲಾಗರ್ ಅನ್ನು ಪ್ರಾರಂಭಿಸುವುದು ..."##),
    ],
}
}

/// kn: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ
pub(super) const fn get_kn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_kn_map_log_core),
    ],
}
}

/// Language ID: ko;
/// Map name: "log-core";
/// Description: 한국어, 한국 문자, 대한민국;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "디버그");
/// ```
pub(super) const fn get_ko_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"디버그"##),
        ("warn", r##"경고"##),
        ("info", r##"정보"##),
        ("error", r##"오류"##),
        ("trace", r##"추적"##),
        ("init-logger", r##"로거 초기화 ..."##),
    ],
}
}

/// ko: 한국어, 한국 문자, 대한민국
pub(super) const fn get_ko_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ko_map_log_core),
    ],
}
}

/// Language ID: ku;
/// Map name: "log-core";
/// Description: kurdî, latînî, Tirkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_ku_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"hişyar kirin"##),
        ("info", r##"Agahdariya"##),
        ("error", r##"Erewtî"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Destpêkirina logger ..."##),
    ],
}
}

/// ku: kurdî, latînî, Tirkiye
pub(super) const fn get_ku_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ku_map_log_core),
    ],
}
}

/// Language ID: ky;
/// Map name: "log-core";
/// Description: кыргызча, Кирилл, Кыргызстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_ky_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"Эскертүү"##),
        ("info", r##"Маалымат"##),
        ("error", r##"Error"##),
        ("trace", r##"изин"##),
        ("init-logger", r##"Loggerди ишке киргизүү ..."##),
    ],
}
}

/// ky: кыргызча, Кирилл, Кыргызстан
pub(super) const fn get_ky_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ky_map_log_core),
    ],
}
}

/// Language ID: la;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_la_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"monere"##),
        ("info", r##"Info"##),
        ("error", r##"Error"##),
        ("trace", r##"vestigium"##),
        ("init-logger", r##"Initializing Logger ..."##),
    ],
}
}

/// la: la-Latn-VA
pub(super) const fn get_la_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_la_map_log_core),
    ],
}
}

/// Language ID: lb;
/// Map name: "log-core";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_lb_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"warnen"##),
        ("info", r##"Info"##),
        ("error", r##"Feeler"##),
        ("trace", r##"Spur"##),
        ("init-logger", r##"Initialiséiere Logger ..."##),
    ],
}
}

/// lb: Lëtzebuergesch, Laténgesch, Lëtzebuerg
pub(super) const fn get_lb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_lb_map_log_core),
    ],
}
}

/// Language ID: lo;
/// Map name: "log-core";
/// Description: ລາວ, ລາວ, ລາວ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_lo_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"ເຕືອນ"##),
        ("info", r##"ຂໍ້ມູນ"##),
        ("error", r##"ຜິດພາດ"##),
        ("trace", r##"ຮ່ອງຮອຍ"##),
        ("init-logger", r##"ເລີ່ມຕົ້ນຜູ້ຕັດໄມ້ ..."##),
    ],
}
}

/// lo: ລາວ, ລາວ, ລາວ
pub(super) const fn get_lo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_lo_map_log_core),
    ],
}
}

/// Language ID: lt;
/// Map name: "log-core";
/// Description: lietuvių, lotynų, Lietuva;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Derintis");
/// ```
pub(super) const fn get_lt_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Derintis"##),
        ("warn", r##"perspėti"##),
        ("info", r##"Informacija"##),
        ("error", r##"Klaida"##),
        ("trace", r##"pėdsakas"##),
        ("init-logger", r##"Inicijuoti kaupiklį ..."##),
    ],
}
}

/// lt: lietuvių, lotynų, Lietuva
pub(super) const fn get_lt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_lt_map_log_core),
    ],
}
}

/// Language ID: lv;
/// Map name: "log-core";
/// Description: latviešu, latīņu, Latvija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "atkļūdošana");
/// ```
pub(super) const fn get_lv_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"atkļūdošana"##),
        ("warn", r##"brīdināt"##),
        ("info", r##"Informācija"##),
        ("error", r##"Kļūda"##),
        ("trace", r##"izsekot"##),
        ("init-logger", r##"Inicializējot mežizstrādātāju ..."##),
    ],
}
}

/// lv: latviešu, latīņu, Latvija
pub(super) const fn get_lv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_lv_map_log_core),
    ],
}
}

/// Language ID: mg;
/// Map name: "log-core";
/// Description: Malagasy, Latn, Madagasikara;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_mg_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"mampitandrina"##),
        ("info", r##"Info"##),
        ("error", r##"Hadisoana"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Initialing Logger ..."##),
    ],
}
}

/// mg: Malagasy, Latn, Madagasikara
pub(super) const fn get_mg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mg_map_log_core),
    ],
}
}

/// Language ID: mi;
/// Map name: "log-core";
/// Description: Māori, Rātina, Aotearoa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_mi_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"Warn"##),
        ("info", r##"Korero"##),
        ("error", r##"Hapa"##),
        ("trace", r##"Wāhi hūkahu"##),
        ("init-logger", r##"Te kōkiri i te kaitirotiro ..."##),
    ],
}
}

/// mi: Māori, Rātina, Aotearoa
pub(super) const fn get_mi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mi_map_log_core),
    ],
}
}

/// Language ID: mk;
/// Map name: "log-core";
/// Description: македонски, кирилско писмо, Северна Македонија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Дебагирање");
/// ```
pub(super) const fn get_mk_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Дебагирање"##),
        ("warn", r##"предупредува"##),
        ("info", r##"информации"##),
        ("error", r##"грешка"##),
        ("trace", r##"трага"##),
        ("init-logger", r##"Иницијализирање на дрвосечач ..."##),
    ],
}
}

/// mk: македонски, кирилско писмо, Северна Македонија
pub(super) const fn get_mk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mk_map_log_core),
    ],
}
}

/// Language ID: ml;
/// Map name: "log-core";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ഡീബഗ\u{d4d}");
/// ```
pub(super) const fn get_ml_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ഡീബഗ്"##),
        ("warn", r##"മുന്നറിയിപ്പ്"##),
        ("info", r##"വിവരം"##),
        ("error", r##"പിശക്"##),
        ("trace", r##"ട്രേസ്"##),
        ("init-logger", r##"ലോഗർ സമാരംഭിക്കുന്നു ..."##),
    ],
}
}

/// ml: മലയാളം, മലയാളം, ഇന്ത്യ
pub(super) const fn get_ml_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ml_map_log_core),
    ],
}
}

/// Language ID: mn;
/// Map name: "log-core";
/// Description: монгол, кирилл, Монгол;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "дебаг");
/// ```
pub(super) const fn get_mn_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"дебаг"##),
        ("warn", r##"сэрэмжлүүлэг"##),
        ("info", r##"мэдээлэл"##),
        ("error", r##"алдаа"##),
        ("trace", r##"ул мөр"##),
        ("init-logger", r##"Бүртгэлийг эхлүүлэх ..."##),
    ],
}
}

/// mn: монгол, кирилл, Монгол
pub(super) const fn get_mn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mn_map_log_core),
    ],
}
}

/// Language ID: mr;
/// Map name: "log-core";
/// Description: मराठी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "डीबग");
/// ```
pub(super) const fn get_mr_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"डीबग"##),
        ("warn", r##"चेतावणी द्या"##),
        ("info", r##"माहिती"##),
        ("error", r##"त्रुटी"##),
        ("trace", r##"ट्रेस"##),
        ("init-logger", r##"लॉगर आरंभ करीत आहे ..."##),
    ],
}
}

/// mr: मराठी, देवनागरी, भारत
pub(super) const fn get_mr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mr_map_log_core),
    ],
}
}

/// Language ID: ms;
/// Map name: "log-core";
/// Description: Melayu, Latin, Malaysia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_ms_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"memberi amaran"##),
        ("info", r##"Maklumat"##),
        ("error", r##"ralat"##),
        ("trace", r##"jejak"##),
        ("init-logger", r##"Memulakan logger ..."##),
    ],
}
}

/// ms: Melayu, Latin, Malaysia
pub(super) const fn get_ms_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ms_map_log_core),
    ],
}
}

/// Language ID: mt;
/// Map name: "log-core";
/// Description: Malti, Latin, Malta;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_mt_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"twissi"##),
        ("info", r##"Info"##),
        ("error", r##"Żball"##),
        ("trace", r##"Traċċa"##),
        ("init-logger", r##"Inizjalizza l-logger ..."##),
    ],
}
}

/// mt: Malti, Latin, Malta
pub(super) const fn get_mt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_mt_map_log_core),
    ],
}
}

/// Language ID: my;
/// Map name: "log-core";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_my_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"သတိပေး"##),
        ("info", r##"အချက်အလက်"##),
        ("error", r##"အမှား"##),
        ("trace", r##"သဲလွန်စ"##),
        ("init-logger", r##"logger ကိုစတင်ရန် ..."##),
    ],
}
}

/// my: မြန်မာ, မြန်မာ, မြန်မာ
pub(super) const fn get_my_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_my_map_log_core),
    ],
}
}

/// Language ID: ne;
/// Map name: "log-core";
/// Description: नेपाली, देवानागरी, नेपाल;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "डिबग");
/// ```
pub(super) const fn get_ne_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"डिबग"##),
        ("warn", r##"चेतावनी"##),
        ("info", r##"जानकारी"##),
        ("error", r##"त्रुटि"##),
        ("trace", r##"ट्रेस"##),
        ("init-logger", r##"Logger आरम्भिक"##),
    ],
}
}

/// ne: नेपाली, देवानागरी, नेपाल
pub(super) const fn get_ne_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ne_map_log_core),
    ],
}
}

/// Language ID: nl;
/// Map name: "log-core";
/// Description: Nederlands, Latijns, Nederland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_nl_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"waarschuwen"##),
        ("info", r##"info"##),
        ("error", r##"Fout"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Logger initialiseren ..."##),
    ],
}
}

/// nl: Nederlands, Latijns, Nederland
pub(super) const fn get_nl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_nl_map_log_core),
    ],
}
}

/// Language ID: no;
/// Map name: "log-core";
/// Description: norsk, latinsk, Norge;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUG");
/// ```
pub(super) const fn get_no_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUG"##),
        ("warn", r##"Advarsel"##),
        ("info", r##"Info"##),
        ("error", r##"Feil"##),
        ("trace", r##"Spor"##),
        ("init-logger", r##"Initialisere logger ..."##),
    ],
}
}

/// no: norsk, latinsk, Norge
pub(super) const fn get_no_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_no_map_log_core),
    ],
}
}

/// Language ID: ny;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_ny_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"kuchenjeza"##),
        ("info", r##"Kuchuluka"##),
        ("error", r##"cholakwika"##),
        ("trace", r##"Kufufuza"##),
        ("init-logger", r##"Poyambitsa Logger ..."##),
    ],
}
}

/// ny: ny-Latn-MW
pub(super) const fn get_ny_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ny_map_log_core),
    ],
}
}

/// Language ID: or;
/// Map name: "log-core";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ଡ\u{b3f}ବଗ\u{b4d}");
/// ```
pub(super) const fn get_or_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ଡିବଗ୍"##),
        ("warn", r##"ଚେତାବନୀ"##),
        ("info", r##"ସୂଚନା"##),
        ("error", r##"ତ୍ରୁଟି"##),
        ("trace", r##"ଟ୍ରେସ୍"##),
        ("init-logger", r##"ଲଗର୍ ଆରମ୍ଭ କରିବା ..."##),
    ],
}
}

/// or: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ
pub(super) const fn get_or_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_or_map_log_core),
    ],
}
}

/// Language ID: pa;
/// Map name: "log-core";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ਡੀਬ\u{a71}ਗ");
/// ```
pub(super) const fn get_pa_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ਡੀਬੱਗ"##),
        ("warn", r##"ਚੇਤਾਵਨੀ"##),
        ("info", r##"ਜਾਣਕਾਰੀ"##),
        ("error", r##"ਗਲਤੀ"##),
        ("trace", r##"ਟਰੇਸ"##),
        ("init-logger", r##"ਸ਼ੁਰੂਆਤੀ ਲਾਗਰ ..."##),
    ],
}
}

/// pa: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub(super) const fn get_pa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_pa_map_log_core),
    ],
}
}

/// Language ID: pl;
/// Map name: "log-core";
/// Description: polski, łacińskie, Polska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "DEBUGOWANIE");
/// ```
pub(super) const fn get_pl_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"DEBUGOWANIE"##),
        ("warn", r##"OSTRZEŻENIE"##),
        ("info", r##"INFORMACJA"##),
        ("error", r##"BŁĄD"##),
        ("trace", r##"ŚLEDZENIE"##),
        ("init-logger", r##"Inicjowanie loggera..."##),
    ],
}
}

/// pl: polski, łacińskie, Polska
pub(super) const fn get_pl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_pl_map_log_core),
    ],
}
}

/// Language ID: ps;
/// Map name: "log-core";
/// Description: پښتو, عربي, افغانستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ډیبګ");
/// ```
pub(super) const fn get_ps_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ډیبګ"##),
        ("warn", r##"خبرداری"##),
        ("info", r##"معلومات"##),
        ("error", r##"تېروتنه"##),
        ("trace", r##"ټریس"##),
        ("init-logger", r##"د لاګر پیل پیل کړئ ..."##),
    ],
}
}

/// ps: پښتو, عربي, افغانستان
pub(super) const fn get_ps_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ps_map_log_core),
    ],
}
}

/// Language ID: pt;
/// Map name: "log-core";
/// Description: português, latim, Brasil;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_pt_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"avisar"##),
        ("info", r##"informações"##),
        ("error", r##"Erro"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Inicializando o Logger ..."##),
    ],
}
}

/// pt: português, latim, Brasil
pub(super) const fn get_pt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_pt_map_log_core),
    ],
}
}

/// Language ID: ro;
/// Map name: "log-core";
/// Description: română, latină, România;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "depanare");
/// ```
pub(super) const fn get_ro_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"depanare"##),
        ("warn", r##"avertizează"##),
        ("info", r##"informații"##),
        ("error", r##"eroare"##),
        ("trace", r##"urmă"##),
        ("init-logger", r##"Inițializarea loggerului ..."##),
    ],
}
}

/// ro: română, latină, România
pub(super) const fn get_ro_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ro_map_log_core),
    ],
}
}

/// Language ID: ru;
/// Map name: "log-core";
/// Description: русский, кириллица, Россия;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "отладка");
/// ```
pub(super) const fn get_ru_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"отладка"##),
        ("warn", r##"предупреждать"##),
        ("info", r##"Информация"##),
        ("error", r##"ошибка"##),
        ("trace", r##"трассировка"##),
        ("init-logger", r##"Инициализация регистратора ..."##),
    ],
}
}

/// ru: русский, кириллица, Россия
pub(super) const fn get_ru_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ru_map_log_core),
    ],
}
}

/// Language ID: sd;
/// Map name: "log-core";
/// Description: سنڌي, عربي, پاڪستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ڊيبگ");
/// ```
pub(super) const fn get_sd_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ڊيبگ"##),
        ("warn", r##"ڊي warn اريندڙ"##),
        ("info", r##"info اڻ"##),
        ("error", r##"غلطي"##),
        ("trace", r##"نشان"##),
        ("init-logger", r##"لوگر شروع ڪرڻ ..."##),
    ],
}
}

/// sd: سنڌي, عربي, پاڪستان
pub(super) const fn get_sd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sd_map_log_core),
    ],
}
}

/// Language ID: si;
/// Map name: "log-core";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "න\u{dd2}දොස\u{dca} ක\u{dd2}ර\u{dd3}ම");
/// ```
pub(super) const fn get_si_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"නිදොස් කිරීම"##),
        ("warn", r##"අනතුරු ඇඟවීම"##),
        ("info", r##"තොරතුරු"##),
        ("error", r##"දෝෂයකි"##),
        ("trace", r##"හෝඩුවාව"##),
        ("init-logger", r##"ලොගර් ආරම්භ කිරීම ..."##),
    ],
}
}

/// si: සිංහල, සිංහල, ශ්‍රී ලංකාව
pub(super) const fn get_si_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_si_map_log_core),
    ],
}
}

/// Language ID: sk;
/// Map name: "log-core";
/// Description: slovenčina, latinka, Slovensko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ladenie");
/// ```
pub(super) const fn get_sk_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ladenie"##),
        ("warn", r##"varovať"##),
        ("info", r##"Informácie"##),
        ("error", r##"chyba"##),
        ("trace", r##"stopa"##),
        ("init-logger", r##"Inicializácia Loggeru ..."##),
    ],
}
}

/// sk: slovenčina, latinka, Slovensko
pub(super) const fn get_sk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sk_map_log_core),
    ],
}
}

/// Language ID: sl;
/// Map name: "log-core";
/// Description: slovenščina, latinica, Slovenija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "odpravljanje napak");
/// ```
pub(super) const fn get_sl_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"odpravljanje napak"##),
        ("warn", r##"Opozori"##),
        ("info", r##"informacije"##),
        ("error", r##"Napaka"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Inicializacija zapisovanja ..."##),
    ],
}
}

/// sl: slovenščina, latinica, Slovenija
pub(super) const fn get_sl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sl_map_log_core),
    ],
}
}

/// Language ID: sm;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_sm_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"lapatai"##),
        ("info", r##"faʻamatalaga"##),
        ("error", r##"sese"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Amataga i le Logger ..."##),
    ],
}
}

/// sm: sm-Latn-WS
pub(super) const fn get_sm_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sm_map_log_core),
    ],
}
}

/// Language ID: sn;
/// Map name: "log-core";
/// Description: chiShona, Latn, Zimbabwe;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_sn_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"Inyambira"##),
        ("info", r##"info"##),
        ("error", r##"kukanganisa"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Kutanga logger ..."##),
    ],
}
}

/// sn: chiShona, Latn, Zimbabwe
pub(super) const fn get_sn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sn_map_log_core),
    ],
}
}

/// Language ID: so;
/// Map name: "log-core";
/// Description: Soomaali, Laatiin, Soomaaliya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Bax");
/// ```
pub(super) const fn get_so_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Bax"##),
        ("warn", r##"Dig"##),
        ("info", r##"Macluumaadka"##),
        ("error", r##"qaladka"##),
        ("trace", r##"raad raac"##),
        ("init-logger", r##"Bilowga Logger ..."##),
    ],
}
}

/// so: Soomaali, Laatiin, Soomaaliya
pub(super) const fn get_so_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_so_map_log_core),
    ],
}
}

/// Language ID: sq;
/// Map name: "log-core";
/// Description: shqip, latin, Shqipëri;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debugimi");
/// ```
pub(super) const fn get_sq_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debugimi"##),
        ("warn", r##"Paralajmëroni"##),
        ("info", r##"Informacioni"##),
        ("error", r##"Gabim"##),
        ("trace", r##"Gjurmë"##),
        ("init-logger", r##"Inicializimi i loggerit ..."##),
    ],
}
}

/// sq: shqip, latin, Shqipëri
pub(super) const fn get_sq_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sq_map_log_core),
    ],
}
}

/// Language ID: sr;
/// Map name: "log-core";
/// Description: српски, ћирилица, Србија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "уклањање погрешака");
/// ```
pub(super) const fn get_sr_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"уклањање погрешака"##),
        ("warn", r##"упозорите"##),
        ("info", r##"Информације о"##),
        ("error", r##"Грешка"##),
        ("trace", r##"траг"##),
        ("init-logger", r##"Иницијализација логгер ..."##),
    ],
}
}

/// sr: српски, ћирилица, Србија
pub(super) const fn get_sr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sr_map_log_core),
    ],
}
}

/// Language ID: st;
/// Map name: "log-core";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_st_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"warn"##),
        ("info", r##"Info"##),
        ("error", r##"phoso"##),
        ("trace", r##"trace"##),
        ("init-logger", r##"Ho qala ka mokopu ..."##),
    ],
}
}

/// st: st-Latn-ZA
pub(super) const fn get_st_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_st_map_log_core),
    ],
}
}

/// Language ID: su;
/// Map name: "log-core";
/// Description: Basa Sunda, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "debug");
/// ```
pub(super) const fn get_su_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"debug"##),
        ("warn", r##"ngingetkeun"##),
        ("info", r##"info"##),
        ("error", r##"Kasalahan"##),
        ("trace", r##"Renal"##),
        ("init-logger", r##"NAMPANGKEUN"##),
    ],
}
}

/// su: Basa Sunda, Latin, Indonesia
pub(super) const fn get_su_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_su_map_log_core),
    ],
}
}

/// Language ID: sv;
/// Map name: "log-core";
/// Description: svenska, latinska, Sverige;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "felsökning");
/// ```
pub(super) const fn get_sv_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"felsökning"##),
        ("warn", r##"varna"##),
        ("info", r##"info"##),
        ("error", r##"fel"##),
        ("trace", r##"spår"##),
        ("init-logger", r##"Initialisering av logger ..."##),
    ],
}
}

/// sv: svenska, latinska, Sverige
pub(super) const fn get_sv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sv_map_log_core),
    ],
}
}

/// Language ID: sw;
/// Map name: "log-core";
/// Description: Kiswahili, Kilatini, Tanzania;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_sw_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"Onyo"##),
        ("info", r##"Maelezo"##),
        ("error", r##"kosa"##),
        ("trace", r##"kufuatilia"##),
        ("init-logger", r##"Kuanzisha Logger ..."##),
    ],
}
}

/// sw: Kiswahili, Kilatini, Tanzania
pub(super) const fn get_sw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_sw_map_log_core),
    ],
}
}

/// Language ID: ta;
/// Map name: "log-core";
/// Description: தமிழ், தமிழ், இந்தியா;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "பிழைத\u{bcd}திருத\u{bcd}தம\u{bcd}");
/// ```
pub(super) const fn get_ta_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"பிழைத்திருத்தம்"##),
        ("warn", r##"எச்சரிக்கை"##),
        ("info", r##"தகவல்"##),
        ("error", r##"பிழை"##),
        ("trace", r##"சுவடு"##),
        ("init-logger", r##"லாகரைத் தொடங்குதல் ..."##),
    ],
}
}

/// ta: தமிழ், தமிழ், இந்தியா
pub(super) const fn get_ta_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ta_map_log_core),
    ],
}
}

/// Language ID: te;
/// Map name: "log-core";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "డ\u{c40}బగ\u{c4d}");
/// ```
pub(super) const fn get_te_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"డీబగ్"##),
        ("warn", r##"హెచ్చరించండి"##),
        ("info", r##"సమాచారం"##),
        ("error", r##"లోపం"##),
        ("trace", r##"ట్రేస్"##),
        ("init-logger", r##"లాగర్ ప్రారంభించడం ..."##),
    ],
}
}

/// te: తెలుగు, తెలుగు, భారతదేశం
pub(super) const fn get_te_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_te_map_log_core),
    ],
}
}

/// Language ID: tg;
/// Map name: "log-core";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_tg_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"Огоҳӣ"##),
        ("info", r##"Маълумот"##),
        ("error", r##"Хатогӣ"##),
        ("trace", r##"Пай"##),
        ("init-logger", r##"Оғози система ..."##),
    ],
}
}

/// tg: тоҷикӣ, Кириллӣ, Тоҷикистон
pub(super) const fn get_tg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_tg_map_log_core),
    ],
}
}

/// Language ID: th;
/// Map name: "log-core";
/// Description: ไทย, ไทย, ไทย;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "การด\u{e35}บ\u{e31}ก");
/// ```
pub(super) const fn get_th_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"การดีบัก"##),
        ("warn", r##"เตือน"##),
        ("info", r##"ข้อมูล"##),
        ("error", r##"ข้อผิดพลาด"##),
        ("trace", r##"การติดตาม"##),
        ("init-logger", r##"เริ่มต้น Logger ..."##),
    ],
}
}

/// th: ไทย, ไทย, ไทย
pub(super) const fn get_th_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_th_map_log_core),
    ],
}
}

/// Language ID: tr;
/// Map name: "log-core";
/// Description: Türkçe, Latin, Türkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Hata ayıklama");
/// ```
pub(super) const fn get_tr_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Hata ayıklama"##),
        ("warn", r##"Uyar"##),
        ("info", r##"Bilgi"##),
        ("error", r##"Hatası"##),
        ("trace", r##"İzleme"##),
        ("init-logger", r##"Logger'ı başlatma ..."##),
    ],
}
}

/// tr: Türkçe, Latin, Türkiye
pub(super) const fn get_tr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_tr_map_log_core),
    ],
}
}

/// Language ID: ug;
/// Map name: "log-core";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "manug");
/// ```
pub(super) const fn get_ug_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"manug"##),
        ("warn", r##"ئاگاھلاندۇرۇش"##),
        ("info", r##"informo"##),
        ("error", r##"خاتالىقى"##),
        ("trace", r##"ئىز"##),
        ("init-logger", r##"خاتىرىسىنى باشلاش ..."##),
    ],
}
}

/// ug: ئۇيغۇرچە, ئەرەب, جۇڭگو
pub(super) const fn get_ug_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ug_map_log_core),
    ],
}
}

/// Language ID: uk;
/// Map name: "log-core";
/// Description: українська, кирилиця, Україна;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "налагодження");
/// ```
pub(super) const fn get_uk_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"налагодження"##),
        ("warn", r##"попередити"##),
        ("info", r##"Інформація"##),
        ("error", r##"помилка"##),
        ("trace", r##"слід"##),
        ("init-logger", r##"Ініціалізація журналу ..."##),
    ],
}
}

/// uk: українська, кирилиця, Україна
pub(super) const fn get_uk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_uk_map_log_core),
    ],
}
}

/// Language ID: ur;
/// Map name: "log-core";
/// Description: اردو, عربی, پاکستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "ڈیبگ");
/// ```
pub(super) const fn get_ur_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"ڈیبگ"##),
        ("warn", r##"انتباہ کریں"##),
        ("info", r##"معلومات"##),
        ("error", r##"غلطی"##),
        ("trace", r##"ٹریس"##),
        ("init-logger", r##"لاگر کو شروع کرنا ..."##),
    ],
}
}

/// ur: اردو, عربی, پاکستان
pub(super) const fn get_ur_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_ur_map_log_core),
    ],
}
}

/// Language ID: uz;
/// Map name: "log-core";
/// Description: o‘zbek, lotin, Oʻzbekiston;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "disksing");
/// ```
pub(super) const fn get_uz_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"disksing"##),
        ("warn", r##"ogohlantirish"##),
        ("info", r##"INFO"##),
        ("error", r##"xato"##),
        ("trace", r##"iz"##),
        ("init-logger", r##"Loggeratsiyani boshlash ..."##),
    ],
}
}

/// uz: o‘zbek, lotin, Oʻzbekiston
pub(super) const fn get_uz_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_uz_map_log_core),
    ],
}
}

/// Language ID: vi;
/// Map name: "log-core";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "gỡ lỗi");
/// ```
pub(super) const fn get_vi_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"gỡ lỗi"##),
        ("warn", r##"cảnh báo"##),
        ("info", r##"thông tin"##),
        ("error", r##"lỗi"##),
        ("trace", r##"dấu vết"##),
        ("init-logger", r##"Khởi tạo logger ..."##),
    ],
}
}

/// vi: Tiếng Việt, Chữ La tinh, Việt Nam
pub(super) const fn get_vi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_vi_map_log_core),
    ],
}
}

/// Language ID: xh;
/// Map name: "log-core";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "Debug");
/// ```
pub(super) const fn get_xh_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"Debug"##),
        ("warn", r##"Lumkisa"##),
        ("info", r##"Ulwazi"##),
        ("error", r##"Impazamo"##),
        ("trace", r##"Trace"##),
        ("init-logger", r##"Ukuqala ungene ..."##),
    ],
}
}

/// xh: IsiXhosa, IsiLatin, EMzantsi Afrika
pub(super) const fn get_xh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_xh_map_log_core),
    ],
}
}

/// Language ID: yi;
/// Map name: "log-core";
/// Description: ייִדיש, העברעיש, וועלט;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "דעבוג");
/// ```
pub(super) const fn get_yi_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"דעבוג"##),
        ("warn", r##"וואָרענען"##),
        ("info", r##"אינפֿאָרמאַציע"##),
        ("error", r##"טעות"##),
        ("trace", r##"שפּור"##),
        ("init-logger", r##"יניטיאַליזינג לאַגער ..."##),
    ],
}
}

/// yi: ייִדיש, העברעיש, וועלט
pub(super) const fn get_yi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_yi_map_log_core),
    ],
}
}

/// Language ID: yo;
/// Map name: "log-core";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "wadi");
/// ```
pub(super) const fn get_yo_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"wadi"##),
        ("warn", r##"kirn"##),
        ("info", r##"Alaye"##),
        ("error", r##"aṣiṣe"##),
        ("trace", r##"wa kakiri"##),
        ("init-logger", r##"Logoli ti ipilẹṣẹ ..."##),
    ],
}
}

/// yo: Èdè Yorùbá, Èdè Látìn, Nàìjíríà
pub(super) const fn get_yo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_yo_map_log_core),
    ],
}
}

/// Language ID: zh;
/// Map name: "log-core";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "调试");
/// ```
pub(super) const fn get_zh_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"调试"##),
        ("warn", r##"警告"##),
        ("info", r##"信息"##),
        ("error", r##"错误"##),
        ("trace", r##"追踪"##),
        ("init-logger", r##"正在初始化 logger（日志记录器）..."##),
    ],
}
}

/// zh: 简体中文, 中国
pub(super) const fn get_zh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_zh_map_log_core),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "log-core";
/// Description: 正體中文, 中國台灣;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "除錯");
/// ```
pub(super) const fn get_zh_hant_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"除錯"##),
        ("warn", r##"警告"##),
        ("info", r##"資訊"##),
        ("error", r##"錯誤"##),
        ("trace", r##"追蹤"##),
        ("init-logger", r##"正在初始化 logger（日誌記錄器）..."##),
    ],
}
}

/// zh-Hant: 正體中文, 中國台灣
pub(super) const fn get_zh_hant_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_zh_hant_map_log_core),
    ],
}
}

/// Language ID: zh-Hant-HK;
/// Map name: "log-core";
/// Description: 繁體中文, 中國香港;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "調試");
/// ```
pub(super) const fn get_zh_hant_hk_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"調試"##),
        ("warn", r##"警告"##),
        ("info", r##"信息"##),
        ("error", r##"錯誤"##),
        ("trace", r##"追蹤"##),
        ("init-logger", r##"正在初始化 logger（日誌記錄器）..."##),
    ],
}
}

/// zh-Hant-HK: 繁體中文, 中國香港
pub(super) const fn get_zh_hant_hk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_zh_hant_hk_map_log_core),
    ],
}
}

/// Language ID: zu;
/// Map name: "log-core";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("log-core", "debug");
///
/// assert_eq!(msg, "iphutha");
/// ```
pub(super) const fn get_zu_map_log_core() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 5),
        (0, 0),
    ],
    entries: &[
        ("debug", r##"iphutha"##),
        ("warn", r##"xwayisa"##),
        ("info", r##"imininingwane"##),
        ("error", r##"Iphutha"##),
        ("trace", r##"Landelela"##),
        ("init-logger", r##"Ukuqalisa i-logger ..."##),
    ],
}
}

/// zu: isiZulu, isi-Latin, iNingizimu Afrika
pub(super) const fn get_zu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("log-core", get_zu_map_log_core),
    ],
}
}

/// # Example
///
/// ```no_run
/// let map = locale_map();
///
/// for k in map.keys() {
///     println!("{k}")
/// }
///
/// map.get("en").map(|v| dbg!(v()));
/// ```
pub(super) const fn locale_map() -> LocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 5),
        (33, 70),
        (0, 94),
        (0, 0),
        (5, 7),
        (0, 17),
        (1, 19),
        (0, 105),
        (0, 56),
        (0, 37),
        (43, 61),
        (0, 0),
        (3, 14),
        (63, 87),
        (0, 10),
        (7, 7),
        (0, 7),
        (0, 1),
        (10, 18),
        (1, 54),
        (0, 3),
        (16, 54),
    ],
    entries: &[
        ("mk", get_mk_map),
        ("fr", get_fr_map),
        ("si", get_si_map),
        ("ro", get_ro_map),
        ("ko", get_ko_map),
        ("ga", get_ga_map),
        ("fa", get_fa_map),
        ("lv", get_lv_map),
        ("pt", get_pt_map),
        ("tg", get_tg_map),
        ("co", get_co_map),
        ("mr", get_mr_map),
        ("ku", get_ku_map),
        ("nl", get_nl_map),
        ("hu", get_hu_map),
        ("lo", get_lo_map),
        ("yi", get_yi_map),
        ("kk", get_kk_map),
        ("ne", get_ne_map),
        ("sr", get_sr_map),
        ("no", get_no_map),
        ("he", get_he_map),
        ("ka", get_ka_map),
        ("be", get_be_map),
        ("ml", get_ml_map),
        ("sl", get_sl_map),
        ("sv", get_sv_map),
        ("kn", get_kn_map),
        ("af", get_af_map),
        ("vi", get_vi_map),
        ("jw", get_jw_map),
        ("sn", get_sn_map),
        ("fi", get_fi_map),
        ("ky", get_ky_map),
        ("fy", get_fy_map),
        ("uk", get_uk_map),
        ("bg", get_bg_map),
        ("lt", get_lt_map),
        ("am", get_am_map),
        ("cy", get_cy_map),
        ("or", get_or_map),
        ("ht", get_ht_map),
        ("sk", get_sk_map),
        ("sw", get_sw_map),
        ("zh-Hant-HK", get_zh_hant_hk_map),
        ("cs", get_cs_map),
        ("yo", get_yo_map),
        ("el", get_el_map),
        ("mi", get_mi_map),
        ("az", get_az_map),
        ("pl", get_pl_map),
        ("ja", get_ja_map),
        ("mg", get_mg_map),
        ("eo", get_eo_map),
        ("zh", get_zh_map),
        ("ny", get_ny_map),
        ("de", get_de_map),
        ("zu", get_zu_map),
        ("eu", get_eu_map),
        ("zh-Hant", get_zh_hant_map),
        ("ceb", get_ceb_map),
        ("xh", get_xh_map),
        ("ru", get_ru_map),
        ("ha", get_ha_map),
        ("my", get_my_map),
        ("te", get_te_map),
        ("bn", get_bn_map),
        ("hr", get_hr_map),
        ("tr", get_tr_map),
        ("bs", get_bs_map),
        ("ca", get_ca_map),
        ("hi", get_hi_map),
        ("gu", get_gu_map),
        ("sm", get_sm_map),
        ("lb", get_lb_map),
        ("th", get_th_map),
        ("ar", get_ar_map),
        ("mt", get_mt_map),
        ("sq", get_sq_map),
        ("gd", get_gd_map),
        ("so", get_so_map),
        ("is", get_is_map),
        ("km", get_km_map),
        ("ig", get_ig_map),
        ("ms", get_ms_map),
        ("da", get_da_map),
        ("es", get_es_map),
        ("sd", get_sd_map),
        ("ps", get_ps_map),
        ("ta", get_ta_map),
        ("en", get_en_map),
        ("et", get_et_map),
        ("ur", get_ur_map),
        ("uz", get_uz_map),
        ("it", get_it_map),
        ("iw", get_iw_map),
        ("ug", get_ug_map),
        ("en-GB", get_en_gb_map),
        ("id", get_id_map),
        ("mn", get_mn_map),
        ("pa", get_pa_map),
        ("hy", get_hy_map),
        ("gl", get_gl_map),
        ("st", get_st_map),
        ("su", get_su_map),
        ("haw", get_haw_map),
        ("la", get_la_map),
        ("fil", get_fil_map),
    ],
}
}

/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```
pub(super) fn locale_hashmap() -> LocaleHashMap {
    use lang_id_consts::*;

    HashMap::from_iter([
        (unsafe { get_af() }, get_af_map()),
        (unsafe { get_am() }, get_am_map()),
        (unsafe { get_ar() }, get_ar_map()),
        (unsafe { get_az() }, get_az_map()),
        (unsafe { get_be() }, get_be_map()),
        (unsafe { get_bg() }, get_bg_map()),
        (unsafe { get_bn() }, get_bn_map()),
        (unsafe { get_bs() }, get_bs_map()),
        (unsafe { get_ca() }, get_ca_map()),
        (unsafe { get_ceb() }, get_ceb_map()),
        (unsafe { get_co() }, get_co_map()),
        (unsafe { get_cs() }, get_cs_map()),
        (unsafe { get_cy() }, get_cy_map()),
        (unsafe { get_da() }, get_da_map()),
        (unsafe { get_de() }, get_de_map()),
        (unsafe { get_el() }, get_el_map()),
        (unsafe { get_en() }, get_en_map()),
        (unsafe { get_en_gb() }, get_en_gb_map()),
        (unsafe { get_eo() }, get_eo_map()),
        (unsafe { get_es() }, get_es_map()),
        (unsafe { get_et() }, get_et_map()),
        (unsafe { get_eu() }, get_eu_map()),
        (unsafe { get_fa() }, get_fa_map()),
        (unsafe { get_fi() }, get_fi_map()),
        (unsafe { get_fil() }, get_fil_map()),
        (unsafe { get_fr() }, get_fr_map()),
        (unsafe { get_fy() }, get_fy_map()),
        (unsafe { get_ga() }, get_ga_map()),
        (unsafe { get_gd() }, get_gd_map()),
        (unsafe { get_gl() }, get_gl_map()),
        (unsafe { get_gu() }, get_gu_map()),
        (unsafe { get_ha() }, get_ha_map()),
        (unsafe { get_haw() }, get_haw_map()),
        (unsafe { get_he() }, get_he_map()),
        (unsafe { get_hi() }, get_hi_map()),
        (unsafe { get_hr() }, get_hr_map()),
        (unsafe { get_ht() }, get_ht_map()),
        (unsafe { get_hu() }, get_hu_map()),
        (unsafe { get_hy() }, get_hy_map()),
        (unsafe { get_id() }, get_id_map()),
        (unsafe { get_ig() }, get_ig_map()),
        (unsafe { get_is() }, get_is_map()),
        (unsafe { get_it() }, get_it_map()),
        (unsafe { get_iw() }, get_iw_map()),
        (unsafe { get_ja() }, get_ja_map()),
        (unsafe { get_jw() }, get_jw_map()),
        (unsafe { get_ka() }, get_ka_map()),
        (unsafe { get_kk() }, get_kk_map()),
        (unsafe { get_km() }, get_km_map()),
        (unsafe { get_kn() }, get_kn_map()),
        (unsafe { get_ko() }, get_ko_map()),
        (unsafe { get_ku() }, get_ku_map()),
        (unsafe { get_ky() }, get_ky_map()),
        (unsafe { get_la() }, get_la_map()),
        (unsafe { get_lb() }, get_lb_map()),
        (unsafe { get_lo() }, get_lo_map()),
        (unsafe { get_lt() }, get_lt_map()),
        (unsafe { get_lv() }, get_lv_map()),
        (unsafe { get_mg() }, get_mg_map()),
        (unsafe { get_mi() }, get_mi_map()),
        (unsafe { get_mk() }, get_mk_map()),
        (unsafe { get_ml() }, get_ml_map()),
        (unsafe { get_mn() }, get_mn_map()),
        (unsafe { get_mr() }, get_mr_map()),
        (unsafe { get_ms() }, get_ms_map()),
        (unsafe { get_mt() }, get_mt_map()),
        (unsafe { get_my() }, get_my_map()),
        (unsafe { get_ne() }, get_ne_map()),
        (unsafe { get_nl() }, get_nl_map()),
        (unsafe { get_no() }, get_no_map()),
        (unsafe { get_ny() }, get_ny_map()),
        (unsafe { get_or() }, get_or_map()),
        (unsafe { get_pa() }, get_pa_map()),
        (unsafe { get_pl() }, get_pl_map()),
        (unsafe { get_ps() }, get_ps_map()),
        (unsafe { get_pt() }, get_pt_map()),
        (unsafe { get_ro() }, get_ro_map()),
        (unsafe { get_ru() }, get_ru_map()),
        (unsafe { get_sd() }, get_sd_map()),
        (unsafe { get_si() }, get_si_map()),
        (unsafe { get_sk() }, get_sk_map()),
        (unsafe { get_sl() }, get_sl_map()),
        (unsafe { get_sm() }, get_sm_map()),
        (unsafe { get_sn() }, get_sn_map()),
        (unsafe { get_so() }, get_so_map()),
        (unsafe { get_sq() }, get_sq_map()),
        (unsafe { get_sr() }, get_sr_map()),
        (unsafe { get_st() }, get_st_map()),
        (unsafe { get_su() }, get_su_map()),
        (unsafe { get_sv() }, get_sv_map()),
        (unsafe { get_sw() }, get_sw_map()),
        (unsafe { get_ta() }, get_ta_map()),
        (unsafe { get_te() }, get_te_map()),
        (unsafe { get_tg() }, get_tg_map()),
        (unsafe { get_th() }, get_th_map()),
        (unsafe { get_tr() }, get_tr_map()),
        (unsafe { get_ug() }, get_ug_map()),
        (unsafe { get_uk() }, get_uk_map()),
        (unsafe { get_ur() }, get_ur_map()),
        (unsafe { get_uz() }, get_uz_map()),
        (unsafe { get_vi() }, get_vi_map()),
        (unsafe { get_xh() }, get_xh_map()),
        (unsafe { get_yi() }, get_yi_map()),
        (unsafe { get_yo() }, get_yo_map()),
        (unsafe { get_zh() }, get_zh_map()),
        (unsafe { get_zh_hant() }, get_zh_hant_map()),
        (unsafe { get_zh_hant_hk() }, get_zh_hant_hk_map()),
        (unsafe { get_zu() }, get_zu_map()),
    ])
}
