# BusinessPayout

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**source_wallet_id** | Option<**String**> | The identifier of the source wallet used to fund a payout. | [optional]
**destination** | Option<[**models::BankDestination**](BankDestination.md)> |  | [optional]
**amount** | Option<[**models::FiatMoney**](FiatMoney.md)> |  | [optional]
**to_amount** | Option<[**models::FiatPayoutToMoney**](FiatPayoutToMoney.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**status** | Option<[**models::PayoutStatus**](PayoutStatus.md)> |  | [optional]
**tracking_ref** | Option<[**serde_json::Value**](.md)> | A payout tracking reference. Will be present once known. | [optional]
**error_code** | Option<[**models::PayoutErrorCode**](PayoutErrorCode.md)> |  | [optional]
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**adjustments** | Option<[**models::FinalAdjustments**](FinalAdjustments.md)> |  | [optional]
**r#return** | Option<[**models::UnwithdrawalObject**](UnwithdrawalObject.md)> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


