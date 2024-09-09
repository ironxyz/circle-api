# UpdateBillingDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | Option<**String**> | First name of the card or bank account holder. | [optional]
**last_name** | Option<**String**> | Last name of the card or bank account holder. | [optional]
**line1** | Option<**String**> | Line one of the street address. | [optional]
**line2** | Option<**String**> | Line two of the street address. | [optional]
**city** | Option<**String**> | City portion of the address. | [optional]
**district** | Option<**String**> | State / County / Province / Region portion of the address. If the country is US or Canada, then district is required and should use the two-letter code for the subdivision. | [optional]
**postal_code** | Option<**String**> | Postal / ZIP code of the address. | [optional]
**country** | Option<**String**> | Country portion of the address. Formatted as a two-letter country code specified in ISO 3166-1 alpha-2. | [optional]
**phone** | Option<**String**> | Phone number of the user in E.164 format. We recommend using a library such as [libphonenumber](https://github.com/google/libphonenumber) to parse and validate phone numbers. | [optional]
**email** | Option<**String**> | Email of the user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


