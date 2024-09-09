# Settlement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]
**wallet_id** | Option<**String**> | If this settlement was used for a marketplace payment, the wallet involved in the settlement. Not included for standard merchant settlements. | [optional]
**total_debits** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**total_credits** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**payment_fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**chargeback_fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


