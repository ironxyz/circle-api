# CbitFiatAccountResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**status** | [**models::ExternalFiatAccountStatus**](ExternalFiatAccountStatus.md) |  | 
**tracking_ref** | **String** | Tracking ref that needs to be set in the public description field when you send the funds to Circle CBIT wallet. | 
**wallet_address** | **String** | Your CBIT wallet address. | 
**transfer_types_info** | [**std::collections::HashMap<String, models::TransferTypeInfo>**](TransferTypeInfo.md) | A <TransferType, TransferTypeInfo> map which shows transfer types supported on this account as well as additional information for each. For CBIT accounts this will always only show information for CBIT transfers. | 
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | **String** | ISO-8601 UTC date/time format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


