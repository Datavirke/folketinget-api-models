pub use default::*;

// Code automatically generated using https://github.com/Datavirke/odata-rust-generator

// Any changes made to this file may be overwritten by future code generation runs!

#[cfg(feature = "serde")]
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

#[cfg(feature = "reflection")]
pub trait OpenDataModel {
    fn name() -> &'static str;

    fn fields() -> &'static [(&'static str, OpenDataType)];
}

#[cfg(feature = "reflection")]
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
            #[cfg(feature = "reflection")]
            pub fn entity_types(
            ) -> &'static [(&'static str, &'static [(&'static str, crate::OpenDataType)])]
            {
                &[
                    (
                        "Afstemning",
                        &[
                            (
                                "id",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "konklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "vedtaget",
                                crate::OpenDataType::Boolean {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kommentar",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "mødeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "gruppenavnkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "navn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fornavn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "efternavn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "biografi",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "startdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "fraaktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "tilaktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "startdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kørebemærkning",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kommentar",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "forhandlingskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "forhandling",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "superid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "sagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "mødeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenspunktid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "note",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dagsordenspunktid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "modtagelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "frigivelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålsordlyd",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålstitel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "spørgsmålsid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "procedurenummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "grundnotatstatus",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenudgavenummer",
                                crate::OpenDataType::Int16 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kategori",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "emneord",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "emneordid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "emneordid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "versionsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "filurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "variantkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "format",
                                crate::OpenDataType::String {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "entitetnavn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kollonenavn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "beskrivelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "entitetnavn",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "beskrivelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lokale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dagsordenurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "starttidsbemærkning",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "mødeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "startdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "slutdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "kode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "kategoriid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "titelkort",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "offentlighedskode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerprefix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummernumerisk",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "nummerpostfix",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "resume",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningskonklusion",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "periodeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesresultatkode",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "baggrundsmateriale",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "statsbudgetsag",
                                crate::OpenDataType::Boolean {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "begrundelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragrafnummer",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "paragraf",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afgørelse",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "rådsmødedato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "lovnummerdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "retsinformationsurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "fremsatundersagid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "deltundersagid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "aktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "bilagsnummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "frigivelsesdato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "kategori",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "titel",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "dato",
                                crate::OpenDataType::DateTime {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "sagid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidendeurl",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidende",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "folketingstidendesidenummer",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "statusid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "rolleid",
                                crate::OpenDataType::Int32 {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "rolle",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "førstesagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "andetsagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "sagstrinid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "dokumentid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "status",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "typeid",
                                crate::OpenDataType::Int32 {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "afstemningid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "aktørid",
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
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
                                crate::OpenDataType::Int32 {
                                    nullable: false,
                                    key: true,
                                },
                            ),
                            (
                                "type",
                                crate::OpenDataType::String {
                                    nullable: true,
                                    key: false,
                                },
                            ),
                            (
                                "opdateringsdato",
                                crate::OpenDataType::DateTime {
                                    nullable: false,
                                    key: false,
                                },
                            ),
                        ],
                    ),
                ]
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Afstemning {
                pub id: i32,
                pub nummer: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub konklusion: Option<String>,
                pub vedtaget: bool,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kommentar: Option<String>,
                pub mødeid: i32,
                pub typeid: i32,
                pub sagstrinid: Option<i32>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Afstemning {
                fn name() -> &'static str {
                    "Afstemning"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "konklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "vedtaget",
                            crate::OpenDataType::Boolean {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kommentar",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "mødeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Afstemningstype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Afstemningstype {
                fn name() -> &'static str {
                    "Afstemningstype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Aktstykke {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Aktstykke {
                fn name() -> &'static str {
                    "Aktstykke"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Aktør {
                pub id: i32,
                pub typeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub gruppenavnkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub navn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub fornavn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub efternavn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub biografi: Option<String>,
                pub periodeid: Option<i32>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub startdato: Option<chrono::NaiveDateTime>,
                pub slutdato: Option<chrono::NaiveDateTime>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Aktør {
                fn name() -> &'static str {
                    "Aktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "gruppenavnkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "navn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fornavn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "efternavn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "biografi",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "startdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct AktørAktør {
                pub id: i32,
                pub fraaktørid: i32,
                pub tilaktørid: i32,
                pub startdato: Option<chrono::NaiveDateTime>,
                pub slutdato: Option<chrono::NaiveDateTime>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for AktørAktør {
                fn name() -> &'static str {
                    "AktørAktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "fraaktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "tilaktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "startdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct AktørAktørRolle {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for AktørAktørRolle {
                fn name() -> &'static str {
                    "AktørAktørRolle"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Aktørtype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Aktørtype {
                fn name() -> &'static str {
                    "Aktørtype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Almdel {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Almdel {
                fn name() -> &'static str {
                    "Almdel"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Dagsordenspunkt {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kørebemærkning: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kommentar: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub forhandlingskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub forhandling: Option<String>,
                pub superid: Option<i32>,
                pub sagstrinid: Option<i32>,
                pub mødeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Dagsordenspunkt {
                fn name() -> &'static str {
                    "Dagsordenspunkt"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kørebemærkning",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kommentar",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "forhandlingskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "forhandling",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "superid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "sagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "mødeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct DagsordenspunktDokument {
                pub id: i32,
                pub dokumentid: i32,
                pub dagsordenspunktid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub note: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for DagsordenspunktDokument {
                fn name() -> &'static str {
                    "DagsordenspunktDokument"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenspunktid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "note",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct DagsordenspunktSag {
                pub id: i32,
                pub dagsordenspunktid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for DagsordenspunktSag {
                fn name() -> &'static str {
                    "DagsordenspunktSag"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dagsordenspunktid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Debat {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Debat {
                fn name() -> &'static str {
                    "Debat"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Dokument {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: i32,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                pub dato: chrono::NaiveDateTime,
                pub modtagelsesdato: Option<chrono::NaiveDateTime>,
                pub frigivelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragrafnummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub spørgsmålsordlyd: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub spørgsmålstitel: Option<String>,
                pub spørgsmålsid: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub procedurenummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub grundnotatstatus: Option<String>,
                pub dagsordenudgavenummer: Option<i16>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Dokument {
                fn name() -> &'static str {
                    "Dokument"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "modtagelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "frigivelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålsordlyd",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålstitel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "spørgsmålsid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "procedurenummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "grundnotatstatus",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenudgavenummer",
                            crate::OpenDataType::Int16 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct DokumentAktør {
                pub id: i32,
                pub dokumentid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for DokumentAktør {
                fn name() -> &'static str {
                    "DokumentAktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct DokumentAktørRolle {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for DokumentAktørRolle {
                fn name() -> &'static str {
                    "DokumentAktørRolle"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Dokumentkategori {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kategori: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Dokumentkategori {
                fn name() -> &'static str {
                    "Dokumentkategori"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kategori",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Dokumenttype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Dokumenttype {
                fn name() -> &'static str {
                    "Dokumenttype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Dokumentstatus {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Dokumentstatus {
                fn name() -> &'static str {
                    "Dokumentstatus"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Emneord {
                pub id: i32,
                pub typeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub emneord: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Emneord {
                fn name() -> &'static str {
                    "Emneord"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "emneord",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct EmneordDokument {
                pub id: i32,
                pub emneordid: i32,
                pub dokumentid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for EmneordDokument {
                fn name() -> &'static str {
                    "EmneordDokument"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "emneordid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct EmneordSag {
                pub id: i32,
                pub emneordid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for EmneordSag {
                fn name() -> &'static str {
                    "EmneordSag"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "emneordid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Emneordstype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Emneordstype {
                fn name() -> &'static str {
                    "Emneordstype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct EUsag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for EUsag {
                fn name() -> &'static str {
                    "EUsag"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Forslag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Forslag {
                fn name() -> &'static str {
                    "Forslag"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Fil {
                pub id: i32,
                pub dokumentid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                pub versionsdato: chrono::NaiveDateTime,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub filurl: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub variantkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub format: Option<String>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Fil {
                fn name() -> &'static str {
                    "Fil"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "versionsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "filurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "variantkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "format",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct KolloneBeskrivelse {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub entitetnavn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kollonenavn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub beskrivelse: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for KolloneBeskrivelse {
                fn name() -> &'static str {
                    "KolloneBeskrivelse"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "entitetnavn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kollonenavn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "beskrivelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct EntitetBeskrivelse {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub entitetnavn: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub beskrivelse: Option<String>,
                pub opdateringsdato: Option<chrono::NaiveDateTime>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for EntitetBeskrivelse {
                fn name() -> &'static str {
                    "EntitetBeskrivelse"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "entitetnavn",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "beskrivelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Møde {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lokale: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub dagsordenurl: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub starttidsbemærkning: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                pub dato: Option<chrono::NaiveDateTime>,
                pub statusid: Option<i32>,
                pub typeid: Option<i32>,
                pub periodeid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Møde {
                fn name() -> &'static str {
                    "Møde"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lokale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dagsordenurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "starttidsbemærkning",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct MødeAktør {
                pub id: i32,
                pub mødeid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for MødeAktør {
                fn name() -> &'static str {
                    "MødeAktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "mødeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Mødestatus {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Mødestatus {
                fn name() -> &'static str {
                    "Mødestatus"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Mødetype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Mødetype {
                fn name() -> &'static str {
                    "Mødetype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Omtryk {
                pub id: i32,
                pub dokumentid: i32,
                pub dato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Omtryk {
                fn name() -> &'static str {
                    "Omtryk"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Periode {
                pub id: i32,
                pub startdato: chrono::NaiveDateTime,
                pub slutdato: chrono::NaiveDateTime,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Periode {
                fn name() -> &'static str {
                    "Periode"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "startdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "slutdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "kode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sag {
                pub id: i32,
                pub typeid: i32,
                pub kategoriid: Option<i32>,
                pub statusid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titelkort: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub offentlighedskode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummer: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerprefix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummernumerisk: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub nummerpostfix: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub resume: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afstemningskonklusion: Option<String>,
                pub periodeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelsesresultatkode: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub baggrundsmateriale: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub statsbudgetsag: Option<bool>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub begrundelse: Option<String>,
                pub paragrafnummer: Option<i32>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub paragraf: Option<String>,
                pub afgørelsesdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub afgørelse: Option<String>,
                pub rådsmødedato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub lovnummer: Option<String>,
                pub lovnummerdato: Option<chrono::NaiveDateTime>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub retsinformationsurl: Option<String>,
                pub fremsatundersagid: Option<i32>,
                pub deltundersagid: Option<i32>,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sag {
                fn name() -> &'static str {
                    "Sag"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "kategoriid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "titelkort",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "offentlighedskode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerprefix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummernumerisk",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "nummerpostfix",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "resume",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningskonklusion",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "periodeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesresultatkode",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "baggrundsmateriale",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "statsbudgetsag",
                            crate::OpenDataType::Boolean {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "begrundelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragrafnummer",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "paragraf",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afgørelse",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "rådsmødedato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "lovnummerdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "retsinformationsurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "fremsatundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "deltundersagid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagAktør {
                pub id: i32,
                pub aktørid: i32,
                pub sagid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagAktør {
                fn name() -> &'static str {
                    "SagAktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "aktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagAktørRolle {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagAktørRolle {
                fn name() -> &'static str {
                    "SagAktørRolle"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagDokument {
                pub id: i32,
                pub sagid: i32,
                pub dokumentid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub bilagsnummer: Option<String>,
                pub frigivelsesdato: Option<chrono::NaiveDateTime>,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagDokument {
                fn name() -> &'static str {
                    "SagDokument"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "bilagsnummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "frigivelsesdato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagDokumentRolle {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagDokumentRolle {
                fn name() -> &'static str {
                    "SagDokumentRolle"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagskategori {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub kategori: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagskategori {
                fn name() -> &'static str {
                    "Sagskategori"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "kategori",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagsstatus {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagsstatus {
                fn name() -> &'static str {
                    "Sagsstatus"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagstrin {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub titel: Option<String>,
                pub dato: Option<chrono::NaiveDateTime>,
                pub sagid: i32,
                pub typeid: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub folketingstidendeurl: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub folketingstidende: Option<String>,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub folketingstidendesidenummer: Option<String>,
                pub statusid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagstrin {
                fn name() -> &'static str {
                    "Sagstrin"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "titel",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "dato",
                            crate::OpenDataType::DateTime {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "sagid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidendeurl",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidende",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "folketingstidendesidenummer",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "statusid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagstrinAktør {
                pub id: i32,
                pub sagstrinid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
                pub rolleid: i32,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagstrinAktør {
                fn name() -> &'static str {
                    "SagstrinAktør"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "rolleid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagstrinAktørRolle {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub rolle: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagstrinAktørRolle {
                fn name() -> &'static str {
                    "SagstrinAktørRolle"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "rolle",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sambehandlinger {
                pub id: i32,
                pub førstesagstrinid: i32,
                pub andetsagstrinid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sambehandlinger {
                fn name() -> &'static str {
                    "Sambehandlinger"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "førstesagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "andetsagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct SagstrinDokument {
                pub id: i32,
                pub sagstrinid: i32,
                pub dokumentid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for SagstrinDokument {
                fn name() -> &'static str {
                    "SagstrinDokument"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "sagstrinid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "dokumentid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagstrinsstatus {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub status: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagstrinsstatus {
                fn name() -> &'static str {
                    "Sagstrinsstatus"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "status",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagstrinstype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagstrinstype {
                fn name() -> &'static str {
                    "Sagstrinstype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Sagstype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Sagstype {
                fn name() -> &'static str {
                    "Sagstype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Stemme {
                pub id: i32,
                pub typeid: Option<i32>,
                pub afstemningid: i32,
                pub aktørid: i32,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Stemme {
                fn name() -> &'static str {
                    "Stemme"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "typeid",
                            crate::OpenDataType::Int32 {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "afstemningid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "aktørid",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
                                nullable: false,
                                key: false,
                            },
                        ),
                    ]
                }
            }

            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct Stemmetype {
                pub id: i32,
                #[cfg_attr(
                    feature = "serde",
                    serde(deserialize_with = "crate::empty_string_as_none")
                )]
                pub r#type: Option<String>,
                pub opdateringsdato: chrono::NaiveDateTime,
            }

            #[cfg(feature = "reflection")]
            impl crate::OpenDataModel for Stemmetype {
                fn name() -> &'static str {
                    "Stemmetype"
                }

                fn fields() -> &'static [(&'static str, crate::OpenDataType)] {
                    &[
                        (
                            "id",
                            crate::OpenDataType::Int32 {
                                nullable: false,
                                key: true,
                            },
                        ),
                        (
                            "type",
                            crate::OpenDataType::String {
                                nullable: true,
                                key: false,
                            },
                        ),
                        (
                            "opdateringsdato",
                            crate::OpenDataType::DateTime {
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
