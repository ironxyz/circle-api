# DetailedCancel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**r#type** | **String** | Type of the payment object. | 
**merchant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the merchant. | 
**merchant_wallet_id** | Option<**String**> | Unique system generated identifier for the wallet of the merchant. | [optional]
**amount** | [**models::FiatMoneyUsd**](FiatMoneyUsd.md) |  | 
**source** | [**models::SourceResponse**](SourceResponse.md) |  | 
**description** | Option<**String**> | Enumerated description of the payment. | [optional]
**status** | [**models::CancelRefundReversalStatus**](CancelRefundReversalStatus.md) |  | 
**original_payment** | Option<[**models::FiatPayment**](FiatPayment.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**tracking_ref** | Option<**String**> | Payment tracking reference. Will be present once known. | [optional]
**error_code** | Option<[**models::PaymentErrorCode**](PaymentErrorCode.md)> |  | [optional]
**metadata** | Option<[**models::MetadataPhoneEmail**](MetadataPhoneEmail.md)> |  | [optional]
**risk_evaluation** | Option<[**models::RiskEvaluation**](RiskEvaluation.md)> |  | [optional]
**refund** | Option<**bool**> | If the cancel was made after a cutoff time period, it will be processed as a refund. This flag indicates that the cancel was processed as a refund' | [optional][default to false]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


