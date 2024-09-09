# CardUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_id** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) of the public key used in encryption. NOTE the sandbox environment uses the default value of `key1`. For this reason the example supplied is `key1` rather than a UUID. | 
**encrypted_data** | **String** | PGP encrypted base64 encoded string. Contains CVV. * **CVV (Card Verification Number)**: Three or four digit security code. REQUIRED'  | 
**exp_month** | Option<**i32**> | Two digit number representing the card's expiration month. | [optional]
**exp_year** | Option<**i32**> | Four digit number representing the card's expiration year. | [optional]
**billing_details** | Option<[**models::UpdateBillingDetails**](UpdateBillingDetails.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


