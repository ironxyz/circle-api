# GooglePayToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**signature** | **String** | Verifies the message came from Google. The signature is created using ECDSA. | 
**protocol_version** | **String** | Identifies which encryption/signing scheme created this message. In this way, the protocol can evolve over time if needed. If it is not set, assume ECv0. | 
**signed_message** | **String** | A serialized JSON string containing the encryptedMessage, ephemeralPublicKey, and tag. To simplify the signature verification process, this value is serialized. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


