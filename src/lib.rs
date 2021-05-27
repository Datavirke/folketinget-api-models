#![feature(non_ascii_idents)]

pub use default::*;

// Code automatically generated using https://github.com/Datavirke/odata-rust-generator

// Any changes made to this file may be overwritten by future code generation runs!

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = serde::Deserialize::deserialize(de)?;
    let opt = opt.as_deref();
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
    Binary { nullable: bool, key: bool },
    Boolean { nullable: bool, key: bool },
    Byte { nullable: bool, key: bool },
    DateTime { nullable: bool, key: bool },
    DateTimeOffset { nullable: bool, key: bool },
    Decimal { nullable: bool, key: bool },
    Double { nullable: bool, key: bool },
    Int16 { nullable: bool, key: bool },
    Int32 { nullable: bool, key: bool },
    String { nullable: bool, key: bool },
}

pub mod ft {
    pub mod domain {
        pub mod models {
            use crate::{OpenDataModel, OpenDataType};
            use serde::{Deserialize, Serialize};

            pub fn entity_types(
            ) -> &'static [(&'static str, &'static [(&'static str, crate::OpenDataType)])]
            {
                &[
                    (
                        "Afstemning",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "konklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "vedtaget",
                                OpenDataType::Boolean {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kommentar",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "mødeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagstrinid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Afstemningstype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Aktstykke",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Aktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "gruppenavnkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "navn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fornavn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "efternavn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "biografi",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "startdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "AktørAktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "fraaktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "tilaktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "startdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "AktørAktørRolle",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Aktørtype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Almdel",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Dagsordenspunkt",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kørebemærkning",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kommentar",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "forhandlingskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "forhandling",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "superid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "sagstrinid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "mødeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "DagsordenspunktDokument",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenspunktid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "note",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "DagsordenspunktSag",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dagsordenspunktid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Debat",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Dokument",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "modtagelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "frigivelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålsordlyd",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålstitel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålsid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "procedurenummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "grundnotatstatus",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenudgavenummer",
                                OpenDataType::Int16 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "DokumentAktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "DokumentAktørRolle",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Dokumentkategori",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kategori",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Dokumenttype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Dokumentstatus",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Emneord",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "emneord",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "EmneordDokument",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "emneordid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "EmneordSag",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "emneordid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Emneordstype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "EUsag",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Forslag",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Fil",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "versionsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "filurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "variantkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "format",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "KolloneBeskrivelse",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "entitetnavn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kollonenavn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "beskrivelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "EntitetBeskrivelse",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "entitetnavn",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "beskrivelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Møde",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lokale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "starttidsbemærkning",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "MødeAktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "mødeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Mødestatus",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Mødetype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Omtryk",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Periode",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "startdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sag",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagAktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "aktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagAktørRolle",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagDokument",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "bilagsnummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "frigivelsesdato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagDokumentRolle",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagskategori",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kategori",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagsstatus",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagstrin",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "titel",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidendeurl",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidende",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidendesidenummer",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagstrinAktør",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagstrinid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagstrinAktørRolle",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sambehandlinger",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "førstesagstrinid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "andetsagstrinid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "SagstrinDokument",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagstrinid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagstrinsstatus",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagstrinstype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Sagstype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Stemme",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                    (
                        "Stemmetype",
                        &[
                            (
                                "id",
                                OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "konklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "vedtaget",
                            OpenDataType::Boolean {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kommentar",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "mødeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagstrinid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "gruppenavnkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "navn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fornavn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "efternavn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "biografi",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "startdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "fraaktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "tilaktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "startdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kørebemærkning",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kommentar",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "forhandlingskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "forhandling",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "superid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "sagstrinid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "mødeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenspunktid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "note",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dagsordenspunktid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "modtagelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "frigivelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålsordlyd",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålstitel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålsid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "procedurenummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "grundnotatstatus",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenudgavenummer",
                            OpenDataType::Int16 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kategori",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "emneord",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "emneordid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "emneordid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "versionsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "filurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "variantkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "format",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "entitetnavn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kollonenavn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "beskrivelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "entitetnavn",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "beskrivelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lokale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "starttidsbemærkning",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "mødeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "startdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "aktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "bilagsnummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "frigivelsesdato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kategori",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "titel",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidendeurl",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidende",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidendesidenummer",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagstrinid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "førstesagstrinid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "andetsagstrinid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagstrinid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
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
                    &[
                        (
                            "id",
                            OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }
        }
    }
}

pub mod default {
    pub use crate::ft::domain::models::{
        Afstemning, Afstemningstype, Aktstykke, Aktør, AktørAktør, AktørAktørRolle, Aktørtype,
        Almdel, Dagsordenspunkt, DagsordenspunktDokument, DagsordenspunktSag, Debat, Dokument,
        DokumentAktør, DokumentAktørRolle, Dokumentkategori, Dokumentstatus, Dokumenttype, EUsag,
        Emneord, EmneordDokument, EmneordSag, Emneordstype, EntitetBeskrivelse, Fil, Forslag,
        KolloneBeskrivelse, Møde, MødeAktør, Mødestatus, Mødetype, Omtryk, Periode, Sag, SagAktør,
        SagAktørRolle, SagDokument, SagDokumentRolle, Sagskategori, Sagsstatus, Sagstrin,
        SagstrinAktør, SagstrinAktørRolle, SagstrinDokument, Sagstrinsstatus, Sagstrinstype,
        Sagstype, Sambehandlinger, Stemme, Stemmetype,
    };
}
