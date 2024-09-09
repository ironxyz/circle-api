# CryptoPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**r#type** | **String** | Type of the payment object. | 
**merchant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the merchant. | 
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]
**amount** | [**models::CryptoPaymentsOptionalAmountMoney**](CryptoPaymentsOptionalAmountMoney.md) |  | 
**status** | [**models::PaymentStatus**](PaymentStatus.md) |  | 
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**network_fees** | Option<[**models::CryptoPaymentNetworkFee**](CryptoPaymentNetworkFee.md)> |  | [optional]
**payment_intent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**settlement_amount** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**from_addresses** | Option<[**models::CryptoPaymentFromAddresses**](CryptoPayment_fromAddresses.md)> |  | [optional]
**deposit_address** | Option<[**models::CryptoPaymentDepositAddress**](CryptoPayment_depositAddress.md)> |  | [optional]
**transaction_hash** | Option<**String**> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


