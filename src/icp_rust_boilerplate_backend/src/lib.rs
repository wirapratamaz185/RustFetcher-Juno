use std::task::Context;

use candid::Principal;
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
use ic_cdk::api::call::call;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[on_set_doc(collections = ["Receipts_json"])]
async fn on_set_doc(Context: OnSetDocContext<Receipt>) -> Result<(), String> {
    // decode the new data saved in the datastore
    let mut receipt: Receipt = decode_doc_data(&context.data.data.after.data);

    // printout for receipt 
    print(format!("[on_set_doc] Caller: {}", (context.caller.to_text())));

    // printout for collection
    print(format!("[on_set_doc] Collection : {?}", context.data.collection));

    //printout for principal  
    print(format!("[on_set_doc] Principal : {} {}", data.principal.value, data.value.value));

    // encode data back to blob
    let encode_data = encode_doc_data(&data)?;

    // construct parameters to call the function that save the data in the datasotre
    let doc: SetDoc = SetDoc {
        data: encode_data,
        description: context.data.data.after.description,
        updated_at: Some(context.data.data.after.updated_at),
    };

    // call the function that save the data in the datasotre
    let result = set_doc_store(
        context.data.collection,
        context.data.key,
        doc,
        context.data.data.after.updated_at,
    ).await;

    // return the result
    Ok(result)
}

// #[update]
// async fn fetch_receipt(document_id: String) -> Result<Receipt, String> {
//     // Define the canister ID of the Satellite Datastore
//     let satellite_canister_id: Principal = "n5isd-iaaaa-aaaal-adtra-cai".parse().unwrap();

//     // Define the method name to call on satellite canister
//     let method_name = "get_doc";

//     // define the arguments to pass to the method
//     let args = (document_id,);  // Tuple of arguments

//     // Call the method on the Satellite Datastore
//     let result = call(satellite_canister_id, method_name, args).await;

//     match result {
//         Ok(receipt) => {
//             let receipt: Receipt = decode_doc_data(receipt);
//             Ok(receipt)
//         }
//         Err(err) => {
//             Err(err.to_string())
//         }
//     }
// }   