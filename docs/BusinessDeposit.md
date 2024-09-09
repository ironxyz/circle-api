# BusinessDeposit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**source_wallet_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The identifier for the bank account where the funds were deposited from. | [optional]
**destination** | [**models::WalletLocation**](WalletLocation.md) |  | 
**amount** | [**models::FiatMoney**](FiatMoney.md) |  | 
**fee** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**status** | **String** | Status of the deposit. Status `pending` indicates that the deposit is in the process of running; `complete` indicates it finished successfully; `failed` indicates it failed. | 
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


