# FiatPaymentPolymorphic

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
**required_action** | Option<[**models::RequiredAction**](RequiredAction.md)> |  | [optional]
**verification** | Option<[**models::PaymentVerificationResponse**](PaymentVerificationResponse.md)> |  | [optional]
**original_payment** | Option<[**models::FiatPayment**](FiatPayment.md)> |  | [optional]
**cancel** | Option<[**models::FiatCancel**](FiatCancel.md)> |  | [optional]
**refunds** | Option<[**Vec<models::FiatRefund>**](FiatRefund.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**tracking_ref** | Option<**String**> | Payment tracking reference. Will be present once known. | [optional]
**external_ref** | Option<**String**> | External network identifier which will be present once provided from the applicable network.   Examples: * **Input/Output Message Accountability Data (IMAD/OMAD)**: unique number given to each FedWire payment when using the Federal Reserve Bank Service which can be used to investigate and track wire transfers.  | [optional]
**error_code** | Option<[**models::PaymentErrorCode**](PaymentErrorCode.md)> |  | [optional]
**metadata** | Option<[**models::MetadataPhoneEmail**](MetadataPhoneEmail.md)> |  | [optional]
**channel** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The channel identifier that can be set for the payment. When not provided, the default channel is used. | [optional]
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


