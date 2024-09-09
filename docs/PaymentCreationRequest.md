# PaymentCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Universally unique identifier (UUID v4) of the public key used in encryption. NOTE the sandbox environment uses the default value of `key1`. For this reason the example supplied is `key1` rather than a UUID. | [optional]
**metadata** | [**models::MetadataPayment**](MetadataPayment.md) |  | 
**amount** | [**models::FiatMoneyUsd**](FiatMoneyUsd.md) |  | 
**auto_capture** | Option<**bool**> | Triggers the automatic capture of the full payment amount. If set to false the payment will only be authorized but not captured. | [optional][default to true]
**verification** | **String** | Indicates the verification method for this payment. | 
**verification_success_url** | Option<**String**> | The URL to redirect users to after successful 3DS authentication. | [optional]
**verification_failure_url** | Option<**String**> | The URL to redirect users to after failed 3DS authentication. | [optional]
**source** | [**models::Source**](Source.md) |  | 
**description** | Option<**String**> | Description of the payment with length restriction of 240 characters. | [optional]
**encrypted_data** | Option<**String**> | PGP encrypted base64 encoded string. Contains CVV. * **CVV (Card Verification Number)**: Three or four digit security code. Only required if `verification` is `cvv`.  | [optional]
**channel** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The channel identifier that can be set for the payment. When not provided, the default channel is used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


