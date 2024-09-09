# AddressBookRecipient

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique system generated identifier for the entity. | [optional]
**chain** | [**models::Chain**](Chain.md) |  | 
**address** | **String** | An alphanumeric string representing a blockchain address. Will be in different formats for different chains. It is important to preserve the exact formatting and capitalization of the address. | 
**address_tag** | Option<**String**> | The secondary identifier for a blockchain address. An example of this is the memo field on the Stellar network, which can be text, id, or hash format. | [optional]
**metadata** | [**models::AddressBookRecipientMetadata**](AddressBookRecipientMetadata.md) |  | 
**status** | Option<**String**> | Status of the address book recipient. | [optional]
**create_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]
**update_date** | Option<**String**> | ISO-8601 UTC date/time format. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


