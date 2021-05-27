#![feature(non_ascii_idents)]

pub use default::*;

// Code automatically generated using https://github.com/Datavirke/odata-rust-generator

// Any changes made to this file may be overwritten by future code generation runs!

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where T: serde::Deserialize<'de>,
      D: serde::Deserializer<'de>,
{
    let opt: Option<String> = serde::Deserialize::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
    	None | Some("") => Ok(None),
    	Some(s) => T::deserialize(serde::de::IntoDeserializer::into_deserializer(s)).map(Some),
    }
}

pub trait OpenDataModel {
    fn name() -> &'static str;

    fn fields() -> &'static [(&'static str, OpenDataType)];
}

pub enum OpenDataType {
    Binary {
        nullable: bool,
    }
    ,
    Boolean {
        nullable: bool,
    }
    ,
    Byte {
        nullable: bool,
    }
    ,
    DateTime {
        nullable: bool,
    }
    ,
    DateTimeOffset {
        nullable: bool,
    }
    ,
    Decimal {
        nullable: bool,
    }
    ,
    Double {
        nullable: bool,
    }
    ,
    Int16 {
        nullable: bool,
    }
    ,
    Int32 {
        nullable: bool,
    }
    ,
    String {
        nullable: bool,
    }
    ,
}

pub mod ft {
    pub mod domain {
        pub mod models {
            use serde::{Serialize, Deserialize};
            use crate::{OpenDataModel, OpenDataType};

            pub fn entity_types() -> &'static [(&'static str, &'static [(&'static str, crate::OpenDataType)])] {
                &[
                	("Afstemning", &[("id", OpenDataType::Int32 { nullable: false }), ("nummer", OpenDataType::Int32 { nullable: false }), ("konklusion", OpenDataType::String { nullable: true }), ("vedtaget", OpenDataType::Boolean { nullable: false }), ("kommentar", OpenDataType::String { nullable: true }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Afstemningstype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Aktstykke", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("Aktør", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("gruppenavnkort", OpenDataType::String { nullable: true }), ("navn", OpenDataType::String { nullable: true }), ("fornavn", OpenDataType::String { nullable: true }), ("efternavn", OpenDataType::String { nullable: true }), ("biografi", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: true }), ("slutdato", OpenDataType::DateTime { nullable: true })]),
                	("AktørAktør", &[("id", OpenDataType::Int32 { nullable: false }), ("fraaktørid", OpenDataType::Int32 { nullable: false }), ("tilaktørid", OpenDataType::Int32 { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: true }), ("slutdato", OpenDataType::DateTime { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]),
                	("AktørAktørRolle", &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Aktørtype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Almdel", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("Dagsordenspunkt", &[("id", OpenDataType::Int32 { nullable: false }), ("kørebemærkning", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("kommentar", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("forhandlingskode", OpenDataType::String { nullable: true }), ("forhandling", OpenDataType::String { nullable: true }), ("superid", OpenDataType::Int32 { nullable: true }), ("sagstrinid", OpenDataType::Int32 { nullable: true }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]),
                	("DagsordenspunktDokument", &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("dagsordenspunktid", OpenDataType::Int32 { nullable: false }), ("note", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("DagsordenspunktSag", &[("id", OpenDataType::Int32 { nullable: false }), ("dagsordenspunktid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Debat", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("Dokument", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: false }), ("statusid", OpenDataType::Int32 { nullable: false }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: false }), ("modtagelsesdato", OpenDataType::DateTime { nullable: true }), ("frigivelsesdato", OpenDataType::DateTime { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::String { nullable: true }), ("spørgsmålsordlyd", OpenDataType::String { nullable: true }), ("spørgsmålstitel", OpenDataType::String { nullable: true }), ("spørgsmålsid", OpenDataType::Int32 { nullable: true }), ("procedurenummer", OpenDataType::String { nullable: true }), ("grundnotatstatus", OpenDataType::String { nullable: true }), ("dagsordenudgavenummer", OpenDataType::Int16 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("DokumentAktør", &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]),
                	("DokumentAktørRolle", &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Dokumentkategori", &[("id", OpenDataType::Int32 { nullable: false }), ("kategori", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Dokumenttype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Dokumentstatus", &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Emneord", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("emneord", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("EmneordDokument", &[("id", OpenDataType::Int32 { nullable: false }), ("emneordid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("EmneordSag", &[("id", OpenDataType::Int32 { nullable: false }), ("emneordid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Emneordstype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("EUsag", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("Forslag", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("Fil", &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("versionsdato", OpenDataType::DateTime { nullable: false }), ("filurl", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("variantkode", OpenDataType::String { nullable: true }), ("format", OpenDataType::String { nullable: true })]),
                	("KolloneBeskrivelse", &[("id", OpenDataType::Int32 { nullable: false }), ("entitetnavn", OpenDataType::String { nullable: true }), ("kollonenavn", OpenDataType::String { nullable: true }), ("beskrivelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]),
                	("EntitetBeskrivelse", &[("id", OpenDataType::Int32 { nullable: false }), ("entitetnavn", OpenDataType::String { nullable: true }), ("beskrivelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]),
                	("Møde", &[("id", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("lokale", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("dagsordenurl", OpenDataType::String { nullable: true }), ("starttidsbemærkning", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: true }), ("typeid", OpenDataType::Int32 { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("MødeAktør", &[("id", OpenDataType::Int32 { nullable: false }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Mødestatus", &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Mødetype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Omtryk", &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("dato", OpenDataType::DateTime { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Periode", &[("id", OpenDataType::Int32 { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: false }), ("slutdato", OpenDataType::DateTime { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("kode", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sag", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]),
                	("SagAktør", &[("id", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]),
                	("SagAktørRolle", &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("SagDokument", &[("id", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("bilagsnummer", OpenDataType::String { nullable: true }), ("frigivelsesdato", OpenDataType::DateTime { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]),
                	("SagDokumentRolle", &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagskategori", &[("id", OpenDataType::Int32 { nullable: false }), ("kategori", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagsstatus", &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagstrin", &[("id", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: true }), ("sagid", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("folketingstidendeurl", OpenDataType::String { nullable: true }), ("folketingstidende", OpenDataType::String { nullable: true }), ("folketingstidendesidenummer", OpenDataType::String { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("SagstrinAktør", &[("id", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]),
                	("SagstrinAktørRolle", &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sambehandlinger", &[("id", OpenDataType::Int32 { nullable: false }), ("førstesagstrinid", OpenDataType::Int32 { nullable: false }), ("andetsagstrinid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("SagstrinDokument", &[("id", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagstrinsstatus", &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagstrinstype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Sagstype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Stemme", &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: true }), ("afstemningid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                	("Stemmetype", &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]),
                ]
            }

            #[derive(Serialize, Deserialize)]
            pub struct Afstemning {
                pub id: i32,
                pub nummer: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub konklusion: Option<String>,
                pub vedtaget: bool,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kommentar: Option<String>,
                pub mødeid: i32,
                pub typeid: i32,
                pub sagstrinid: Option<i32>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Afstemning {
                fn name() -> &'static str {
                    "Afstemning"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("nummer", OpenDataType::Int32 { nullable: false }), ("konklusion", OpenDataType::String { nullable: true }), ("vedtaget", OpenDataType::Boolean { nullable: false }), ("kommentar", OpenDataType::String { nullable: true }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Afstemningstype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Afstemningstype {
                fn name() -> &'static str {
                    "Afstemningstype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Aktstykke {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for Aktstykke {
                fn name() -> &'static str {
                    "Aktstykke"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Aktør {
                pub id: i32,
                pub typeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub gruppenavnkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub navn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub fornavn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub efternavn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub biografi: Option<String>,
                pub periodeid: Option<i32>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub startdato: Option<chrono::NaiveDateTime>,
                pub slutdato: Option<chrono::NaiveDateTime>,
            }

            impl OpenDataModel for Aktør {
                fn name() -> &'static str {
                    "Aktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("gruppenavnkort", OpenDataType::String { nullable: true }), ("navn", OpenDataType::String { nullable: true }), ("fornavn", OpenDataType::String { nullable: true }), ("efternavn", OpenDataType::String { nullable: true }), ("biografi", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: true }), ("slutdato", OpenDataType::DateTime { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct AktørAktør {
                pub id: i32,
                pub fraaktørid: i32,
                pub tilaktørid: i32,
                pub startdato: Option<chrono::NaiveDateTime>,
                pub slutdato: Option<chrono::NaiveDateTime>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            impl OpenDataModel for AktørAktør {
                fn name() -> &'static str {
                    "AktørAktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("fraaktørid", OpenDataType::Int32 { nullable: false }), ("tilaktørid", OpenDataType::Int32 { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: true }), ("slutdato", OpenDataType::DateTime { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct AktørAktørRolle {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for AktørAktørRolle {
                fn name() -> &'static str {
                    "AktørAktørRolle"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Aktørtype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Aktørtype {
                fn name() -> &'static str {
                    "Aktørtype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Almdel {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for Almdel {
                fn name() -> &'static str {
                    "Almdel"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Dagsordenspunkt {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kørebemærkning: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kommentar: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub forhandlingskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub forhandling: Option<String>,
                pub superid: Option<i32>,
                pub sagstrinid: Option<i32>,
                pub mødeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            impl OpenDataModel for Dagsordenspunkt {
                fn name() -> &'static str {
                    "Dagsordenspunkt"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("kørebemærkning", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("kommentar", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("forhandlingskode", OpenDataType::String { nullable: true }), ("forhandling", OpenDataType::String { nullable: true }), ("superid", OpenDataType::Int32 { nullable: true }), ("sagstrinid", OpenDataType::Int32 { nullable: true }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct DagsordenspunktDokument {
                pub id: i32,
                pub dokumentid: i32,
                pub dagsordenspunktid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub note: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for DagsordenspunktDokument {
                fn name() -> &'static str {
                    "DagsordenspunktDokument"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("dagsordenspunktid", OpenDataType::Int32 { nullable: false }), ("note", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct DagsordenspunktSag {
                pub id: i32,
                pub dagsordenspunktid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for DagsordenspunktSag {
                fn name() -> &'static str {
                    "DagsordenspunktSag"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("dagsordenspunktid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Debat {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for Debat {
                fn name() -> &'static str {
                    "Debat"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Dokument {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: i32,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                pub dato: chrono::NaiveDateTime,
                pub modtagelsesdato: Option<chrono::NaiveDateTime>,
                pub frigivelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragrafnummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub spørgsmålsordlyd: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub spørgsmålstitel: Option<String>,
                pub spørgsmålsid: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub procedurenummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub grundnotatstatus: Option<String>,
                pub dagsordenudgavenummer: Option<i16>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Dokument {
                fn name() -> &'static str {
                    "Dokument"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: false }), ("statusid", OpenDataType::Int32 { nullable: false }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: false }), ("modtagelsesdato", OpenDataType::DateTime { nullable: true }), ("frigivelsesdato", OpenDataType::DateTime { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::String { nullable: true }), ("spørgsmålsordlyd", OpenDataType::String { nullable: true }), ("spørgsmålstitel", OpenDataType::String { nullable: true }), ("spørgsmålsid", OpenDataType::Int32 { nullable: true }), ("procedurenummer", OpenDataType::String { nullable: true }), ("grundnotatstatus", OpenDataType::String { nullable: true }), ("dagsordenudgavenummer", OpenDataType::Int16 { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct DokumentAktør {
                pub id: i32,
                pub dokumentid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            impl OpenDataModel for DokumentAktør {
                fn name() -> &'static str {
                    "DokumentAktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct DokumentAktørRolle {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for DokumentAktørRolle {
                fn name() -> &'static str {
                    "DokumentAktørRolle"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Dokumentkategori {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kategori: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Dokumentkategori {
                fn name() -> &'static str {
                    "Dokumentkategori"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("kategori", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Dokumenttype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Dokumenttype {
                fn name() -> &'static str {
                    "Dokumenttype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Dokumentstatus {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Dokumentstatus {
                fn name() -> &'static str {
                    "Dokumentstatus"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Emneord {
                pub id: i32,
                pub typeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub emneord: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Emneord {
                fn name() -> &'static str {
                    "Emneord"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("emneord", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct EmneordDokument {
                pub id: i32,
                pub emneordid: i32,
                pub dokumentid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for EmneordDokument {
                fn name() -> &'static str {
                    "EmneordDokument"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("emneordid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct EmneordSag {
                pub id: i32,
                pub emneordid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for EmneordSag {
                fn name() -> &'static str {
                    "EmneordSag"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("emneordid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Emneordstype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Emneordstype {
                fn name() -> &'static str {
                    "Emneordstype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct EUsag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for EUsag {
                fn name() -> &'static str {
                    "EUsag"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Forslag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for Forslag {
                fn name() -> &'static str {
                    "Forslag"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Fil {
                pub id: i32,
                pub dokumentid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                pub versionsdato: chrono::NaiveDateTime,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub filurl: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub variantkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub format: Option<String>,
            }

            impl OpenDataModel for Fil {
                fn name() -> &'static str {
                    "Fil"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("versionsdato", OpenDataType::DateTime { nullable: false }), ("filurl", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("variantkode", OpenDataType::String { nullable: true }), ("format", OpenDataType::String { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct KolloneBeskrivelse {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub entitetnavn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kollonenavn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub beskrivelse: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            impl OpenDataModel for KolloneBeskrivelse {
                fn name() -> &'static str {
                    "KolloneBeskrivelse"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("entitetnavn", OpenDataType::String { nullable: true }), ("kollonenavn", OpenDataType::String { nullable: true }), ("beskrivelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct EntitetBeskrivelse {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub entitetnavn: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub beskrivelse: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            impl OpenDataModel for EntitetBeskrivelse {
                fn name() -> &'static str {
                    "EntitetBeskrivelse"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("entitetnavn", OpenDataType::String { nullable: true }), ("beskrivelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Møde {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lokale: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub dagsordenurl: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub starttidsbemærkning: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                pub dato: Option<chrono::NaiveDateTime>,
                pub statusid: Option<i32>,
                pub typeid: Option<i32>,
                pub periodeid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Møde {
                fn name() -> &'static str {
                    "Møde"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("lokale", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("dagsordenurl", OpenDataType::String { nullable: true }), ("starttidsbemærkning", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: true }), ("typeid", OpenDataType::Int32 { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct MødeAktør {
                pub id: i32,
                pub mødeid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for MødeAktør {
                fn name() -> &'static str {
                    "MødeAktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("mødeid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Mødestatus {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Mødestatus {
                fn name() -> &'static str {
                    "Mødestatus"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Mødetype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Mødetype {
                fn name() -> &'static str {
                    "Mødetype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Omtryk {
                pub id: i32,
                pub dokumentid: i32,
                pub dato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Omtryk {
                fn name() -> &'static str {
                    "Omtryk"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("dato", OpenDataType::DateTime { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Periode {
                pub id: i32,
                pub startdato: chrono::NaiveDateTime,
                pub slutdato: chrono::NaiveDateTime,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Periode {
                fn name() -> &'static str {
                    "Periode"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("startdato", OpenDataType::DateTime { nullable: false }), ("slutdato", OpenDataType::DateTime { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("kode", OpenDataType::String { nullable: true }), ("titel", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titelkort: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub offentlighedskode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummer: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerprefix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummernumerisk: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub nummerpostfix: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub resume: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelsesresultatkode: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            impl OpenDataModel for Sag {
                fn name() -> &'static str {
                    "Sag"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("kategoriid", OpenDataType::Int32 { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("titelkort", OpenDataType::String { nullable: true }), ("offentlighedskode", OpenDataType::String { nullable: true }), ("nummer", OpenDataType::String { nullable: true }), ("nummerprefix", OpenDataType::String { nullable: true }), ("nummernumerisk", OpenDataType::String { nullable: true }), ("nummerpostfix", OpenDataType::String { nullable: true }), ("resume", OpenDataType::String { nullable: true }), ("afstemningskonklusion", OpenDataType::String { nullable: true }), ("periodeid", OpenDataType::Int32 { nullable: false }), ("afgørelsesresultatkode", OpenDataType::String { nullable: true }), ("baggrundsmateriale", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("statsbudgetsag", OpenDataType::Boolean { nullable: true }), ("begrundelse", OpenDataType::String { nullable: true }), ("paragrafnummer", OpenDataType::Int32 { nullable: true }), ("paragraf", OpenDataType::String { nullable: true }), ("afgørelsesdato", OpenDataType::DateTime { nullable: true }), ("afgørelse", OpenDataType::String { nullable: true }), ("rådsmødedato", OpenDataType::DateTime { nullable: true }), ("lovnummer", OpenDataType::String { nullable: true }), ("lovnummerdato", OpenDataType::DateTime { nullable: true }), ("retsinformationsurl", OpenDataType::String { nullable: true }), ("fremsatundersagid", OpenDataType::Int32 { nullable: true }), ("deltundersagid", OpenDataType::Int32 { nullable: true })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagAktør {
                pub id: i32,
                pub aktørid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            impl OpenDataModel for SagAktør {
                fn name() -> &'static str {
                    "SagAktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagAktørRolle {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for SagAktørRolle {
                fn name() -> &'static str {
                    "SagAktørRolle"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagDokument {
                pub id: i32,
                pub sagid: i32,
                pub dokumentid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub bilagsnummer: Option<String>,
                pub frigivelsesdato: Option<chrono::NaiveDateTime>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            impl OpenDataModel for SagDokument {
                fn name() -> &'static str {
                    "SagDokument"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("sagid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("bilagsnummer", OpenDataType::String { nullable: true }), ("frigivelsesdato", OpenDataType::DateTime { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagDokumentRolle {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for SagDokumentRolle {
                fn name() -> &'static str {
                    "SagDokumentRolle"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagskategori {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub kategori: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagskategori {
                fn name() -> &'static str {
                    "Sagskategori"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("kategori", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagsstatus {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagsstatus {
                fn name() -> &'static str {
                    "Sagsstatus"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagstrin {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub titel: Option<String>,
                pub dato: Option<chrono::NaiveDateTime>,
                pub sagid: i32,
                pub typeid: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub folketingstidendeurl: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub folketingstidende: Option<String>,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub folketingstidendesidenummer: Option<String>,
                pub statusid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagstrin {
                fn name() -> &'static str {
                    "Sagstrin"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("titel", OpenDataType::String { nullable: true }), ("dato", OpenDataType::DateTime { nullable: true }), ("sagid", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: false }), ("folketingstidendeurl", OpenDataType::String { nullable: true }), ("folketingstidende", OpenDataType::String { nullable: true }), ("folketingstidendesidenummer", OpenDataType::String { nullable: true }), ("statusid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagstrinAktør {
                pub id: i32,
                pub sagstrinid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            impl OpenDataModel for SagstrinAktør {
                fn name() -> &'static str {
                    "SagstrinAktør"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false }), ("rolleid", OpenDataType::Int32 { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagstrinAktørRolle {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for SagstrinAktørRolle {
                fn name() -> &'static str {
                    "SagstrinAktørRolle"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("rolle", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sambehandlinger {
                pub id: i32,
                pub førstesagstrinid: i32,
                pub andetsagstrinid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sambehandlinger {
                fn name() -> &'static str {
                    "Sambehandlinger"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("førstesagstrinid", OpenDataType::Int32 { nullable: false }), ("andetsagstrinid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct SagstrinDokument {
                pub id: i32,
                pub sagstrinid: i32,
                pub dokumentid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for SagstrinDokument {
                fn name() -> &'static str {
                    "SagstrinDokument"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("sagstrinid", OpenDataType::Int32 { nullable: false }), ("dokumentid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagstrinsstatus {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagstrinsstatus {
                fn name() -> &'static str {
                    "Sagstrinsstatus"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("status", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagstrinstype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagstrinstype {
                fn name() -> &'static str {
                    "Sagstrinstype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Sagstype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Sagstype {
                fn name() -> &'static str {
                    "Sagstype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Stemme {
                pub id: i32,
                pub typeid: Option<i32>,
                pub afstemningid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Stemme {
                fn name() -> &'static str {
                    "Stemme"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("typeid", OpenDataType::Int32 { nullable: true }), ("afstemningid", OpenDataType::Int32 { nullable: false }), ("aktørid", OpenDataType::Int32 { nullable: false }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }

            #[derive(Serialize, Deserialize)]
            pub struct Stemmetype {
                pub id: i32,
                #[serde(deserialize_with = "crate::empty_string_as_none")]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            impl OpenDataModel for Stemmetype {
                fn name() -> &'static str {
                    "Stemmetype"
                }

                fn fields() -> &'static [(&'static str, OpenDataType)] {
                    &[("id", OpenDataType::Int32 { nullable: false }), ("type", OpenDataType::String { nullable: true }), ("opdateringsdato", OpenDataType::DateTime { nullable: false })]
                }
            }
        }
    }
}

pub mod default {
    pub use crate::ft::domain::models::{Afstemning, Afstemningstype, Aktstykke, Aktør, AktørAktør, AktørAktørRolle, Aktørtype, Almdel, Dagsordenspunkt, DagsordenspunktDokument, DagsordenspunktSag, Debat, Dokument, DokumentAktør, DokumentAktørRolle, Dokumentkategori, Dokumenttype, Dokumentstatus, Emneord, EmneordDokument, EmneordSag, Emneordstype, EUsag, Forslag, Fil, KolloneBeskrivelse, EntitetBeskrivelse, Møde, MødeAktør, Mødestatus, Mødetype, Omtryk, Periode, Sag, SagAktør, SagAktørRolle, SagDokument, SagDokumentRolle, Sagskategori, Sagsstatus, Sagstrin, SagstrinAktør, SagstrinAktørRolle, Sambehandlinger, SagstrinDokument, Sagstrinsstatus, Sagstrinstype, Sagstype, Stemme, Stemmetype};

}