# AddressBookRecipientRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**chain** | [**models::Chain**](Chain.md) |  | 
**address** | **String** | An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address. | 
**address_tag** | Option<**String**> | The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format. | [optional]
**metadata** | [**models::AddressBookRecipientMetadata**](AddressBookRecipientMetadata.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


