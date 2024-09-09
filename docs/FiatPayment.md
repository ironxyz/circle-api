# FiatPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**r#type** | **String** | Type of the payment object. | 
**merchant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the merchant. | 
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]
**amount** | [**models::FiatMoneyUsd**](FiatMoneyUsd.md) |  | 
**from_amount** | Option<[**models::FiatMoney**](FiatMoney.md)> |  | [optional]
**source** | [**models::SourceResponse**](SourceResponse.md) |  | 
**description** | Option<**String**> | Enumerated description of the payment. | [optional]
**status** | [**models::PaymentStatus**](PaymentStatus.md) |  | 
**captured** | Option<**bool**> | Determines if a payment has successfully been captured. This property is only present for payments that did not use auto capture. | [optional]
**capture_amount** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**capture_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**required_action** | Option<[**models::RequiredAction**](RequiredAction.md)> |  | [optional]
**cancel** | Option<[**models::PaymentInfoCancel**](PaymentInfoCancel.md)> |  | [optional]
**refunds** | Option<[**Vec<models::PaymentInfoPaymentAndRefund>**](PaymentInfoPaymentAndRefund.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**channel** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The channel identifier that can be set for the payment. When not provided, the default channel is used. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


