# PixFiatAccountResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**status** | [**models::ExternalFiatAccountStatus**](ExternalFiatAccountStatus.md) |  | 
**description** | **String** | Bank name plus last four digits of the PIX account number. | 
**tracking_ref** | **String** | Wire tracking ref that needs to be set in the wire reference to beneficiary field. | 
**transfer_types_info** | [**std::collections::HashMap<String, models::TransferTypeInfo>**](TransferTypeInfo.md) | A <TransferType, TransferTypeInfo> map which shows transfer types supported on this account as well as additional information for each. For PIX accounts this will always only show information for PIX transfers. | 
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**fingerprint** | **String** | A UUID that uniquely identifies the account number. If the same account is used more than once, each card object will have a different id, but the fingerprint will stay the same. | 
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | **String** | ISO-8601 UTC date/time format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


