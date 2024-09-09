# BusinessRecipientAddressCreationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**idempotency_key** | [**uuid::Uuid**](uuid::Uuid.md) | Universally unique identifier (UUID v4) idempotency key. This key is utilized to ensure exactly-once execution of mutating requests. | 
**address** | **String** | An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address. | 
**address_tag** | Option<**String**> | The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format. | [optional]
**chain** | [**models::Chain**](Chain.md) |  | 
**currency** | Option<[**models::Currency**](Currency.md)> |  | [optional]
**description** | **String** | An identifier or sentence that describes the recipient. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


