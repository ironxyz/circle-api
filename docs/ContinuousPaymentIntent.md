# ContinuousPaymentIntent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**currency** | **String** | Desired currency of the payment. | 
**amount_paid** | Option<[**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md)> |  | [optional]
**amount_refunded** | Option<[**models::CryptoPaymentsMoney**](CryptoPaymentsMoney.md)> |  | [optional]
**settlement_currency** | **String** | Desired currency for the payments to settle in. | 
**payment_methods** | [**Vec<models::PaymentMethodBlockchain>**](PaymentMethodBlockchain.md) |  | 
**fees** | Option<[**Vec<models::PaymentIntentFees>**](PaymentIntentFees.md)> |  | [optional]
**timeline** | Option<[**Vec<models::Timeline>**](Timeline.md)> | State management timeline. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**r#type** | **String** |  | 
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


