# \AddressesApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_business_deposit_address**](AddressesApi.md#create_business_deposit_address) | **POST** /v1/businessAccount/wallets/addresses/deposit | Create a deposit address
[**create_business_recipient_address**](AddressesApi.md#create_business_recipient_address) | **POST** /v1/businessAccount/wallets/addresses/recipient | Create a recipient address
[**delete_business_recipient_address**](AddressesApi.md#delete_business_recipient_address) | **DELETE** /v1/businessAccount/wallets/addresses/recipient/{id} | Delete a recipient address
[**get_business_deposit_address**](AddressesApi.md#get_business_deposit_address) | **GET** /v1/businessAccount/wallets/addresses/deposit | List all deposit addresses
[**list_business_recipient_addresses**](AddressesApi.md#list_business_recipient_addresses) | **GET** /v1/businessAccount/wallets/addresses/recipient | List all recipient addresses



## create_business_deposit_address

> models::CreateBusinessDepositAddressResponse create_business_deposit_address(business_generate_address_request)
Create a deposit address

Generates a new blockchain address for a wallet for a given currency/chain pair. Circle may reuse addresses on blockchains that support reuse. For example, if you're requesting two addresses for depositing USD and ETH, both on Ethereum, you may see the same Ethereum address returned. Depositing cryptocurrency to a generated address will credit the associated wallet with the value of the deposit. Note: Circle Mint Singapore customers must verify all transfer recipients using the UI in the Circle Console, as transfers from unverified addresses will be held in `pending` status.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_generate_address_request** | Option<[**BusinessGenerateAddressRequest**](BusinessGenerateAddressRequest.md)> |  |  |

### Return type

[**models::CreateBusinessDepositAddressResponse**](CreateBusinessDepositAddressResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_business_recipient_address

> models::CreateBusinessRecipientAddressResponse create_business_recipient_address(business_recipient_address_creation_request)
Create a recipient address

Stores an external blockchain address. Once added, the recipient address must be verified to ensure that you know and trust each new address. Note: Circle Mint Singapore customers must verify all transfer recipients using the UI in the Circle Console, as transfers from unverified addresses will be held in `pending` status. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**business_recipient_address_creation_request** | Option<[**BusinessRecipientAddressCreationRequest**](BusinessRecipientAddressCreationRequest.md)> |  |  |

### Return type

[**models::CreateBusinessRecipientAddressResponse**](CreateBusinessRecipientAddressResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_business_recipient_address

> delete_business_recipient_address(id)
Delete a recipient address

Deletes an external blockchain address. The recipient address must be in an 'active' or 'pending' state in order to be deleted successfully. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_business_deposit_address

> models::GetBusinessDepositAddressResponse get_business_deposit_address()
List all deposit addresses

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetBusinessDepositAddressResponse**](GetBusinessDepositAddressResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_business_recipient_addresses

> models::ListBusinessRecipientAddressesResponse list_business_recipient_addresses(from, to, page_before, page_after, page_size)
List all recipient addresses

Returns a list of recipient addresses that have each been verified and are eligible for transfers. Any recipient addresses pending administrator verification are not included in the response. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListBusinessRecipientAddressesResponse**](listBusinessRecipientAddressesResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

