# CardCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**key_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Universally unique identifier (UUID v4) of the public key used in encryption. NOTE the sandbox environment uses the default value of `key1`. For this reason the example supplied is `key1` rather than a UUID. | [optional]
**encrypted_data** | **String** | PGP encrypted base64 encoded string. Contains Number and CVV. * **Number**: Card number. No spaces or other separators. REQUIRED * **CVV (Card Verification Number)**: Three or four digit security code. REQUIRED'  | 
**billing_details** | [**models::BillingDetails**](BillingDetails.md) |  | 
**exp_month** | **i32** | Two digit number representing the card's expiration month. | 
**exp_year** | **i32** | Four digit number representing the card's expiration year. | 
**metadata** | [**models::MetadataCardAndAch**](MetadataCardAndAch.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


