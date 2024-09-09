# PaymentToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the digital wallet token. | 
**r#type** | **String** | Type of the digital wallet token. | 
**expires_on** | **String** | Datetime when the digital token expires. ISO-8601. | 
**card_details** | [**models::TokenizedCardDetails**](TokenizedCardDetails.md) |  | 
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | **String** | ISO-8601 UTC date/time format. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


