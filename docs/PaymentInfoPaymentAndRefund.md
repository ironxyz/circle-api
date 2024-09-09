# PaymentInfoPaymentAndRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**r#type** | Option<**String**> | Type of the payment object. | [optional]
**amount** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**description** | Option<**String**> | Enumerated description of the payment item. | [optional]
**status** | Option<[**models::PaymentStatus**](PaymentStatus.md)> |  | [optional]
**required_action** | Option<[**models::RequiredAction**](RequiredAction.md)> |  | [optional]
**fees** | Option<[**models::FiatMoneyUsd**](FiatMoneyUsd.md)> |  | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


