# CheckoutSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique system generated identifier for the entity. | 
**r#type** | **String** | The type of this response | 
**success_url** | Option<**String**> | The URL returned to you through client-side callback when the payment is completed. | [optional]
**client_token** | **String** | The access token for the SDK to access checkout session resources | 
**status** | [**models::CheckoutSessionStatus**](CheckoutSessionStatus.md) |  | 
**expires_on** | **String** | ISO-8601 UTC date/time format. | 
**create_date** | **String** | ISO-8601 UTC date/time format. | 
**update_date** | **String** | ISO-8601 UTC date/time format. | 
**amount** | [**models::CheckoutSessionMoney**](CheckoutSessionMoney.md) |  | 
**amount_paid** | [**models::CheckoutSessionMoney**](CheckoutSessionMoney.md) |  | 
**payment_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | IDs of all the associated payments. | [readonly]
**payment_intent_ids** | [**Vec<uuid::Uuid>**](uuid::Uuid.md) | IDs of all the associated payment intents. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


