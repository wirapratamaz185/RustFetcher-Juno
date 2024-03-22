use ic_cdk::print;
use junobuild_macros::{
    on_delete_asset, on_delete_doc, on_delete_many_assets, on_delete_many_docs, on_set_doc,
    on_set_many_docs, on_upload_asset,
};
use junobuild_satellite::{
    include_satellite, set_doc_store, OnDeleteAssetContext, OnDeleteDocContext,
    OnDeleteManyAssetsContext, OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext,
    OnUploadAssetContext, SetDoc,
};
use junobuild_utils::{decode_doc_data, encode_doc_data};
use serde::{Deserialize, Serialize};

// The data of the document we are looking to update in the Satellite Datastore
#[derive(Serialize, Deserialize)]
struct Receipt {
    storeName: String,
    date: String,
    address: String,
    totalPrice: u64,
    taxAndServ: f64,
    mainCategory: String,
    lineItems: Vec<LineItem>,
    locale: Locale,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct LineItem {
    productName: String,
    category: String,
    itemPrice: u64,
    quantity: u64,
    totalPrice: u64,
    modifiers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Locale {
    language: String,
    country: String,
    currency: String,
}
