# CryptoPayout

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**source_wallet_id** | Option<**String**> | The identifier of the source wallet used to fund a payout. | [optional]
**destination** | Option<[**models::CryptoPayoutDestination**](CryptoPayoutDestination.md)> |  | [optional]
**amount** | Option<[**models::PayoutMoney**](PayoutMoney.md)> |  | [optional]
**to_amount** | Option<[**models::PayoutMoney**](PayoutMoney.md)> |  | [optional]
**fees** | Option<[**models::PayoutMoney**](PayoutMoney.md)> |  | [optional]
**network_fees** | Option<[**models::PayoutMoney**](PayoutMoney.md)> |  | [optional]
**status** | Option<[**models::PayoutStatus**](PayoutStatus.md)> |  | [optional]
**error_code** | Option<[**models::PayoutErrorCode**](PayoutErrorCode.md)> |  | [optional]
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


