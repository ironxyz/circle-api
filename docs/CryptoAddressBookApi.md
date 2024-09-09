# \CryptoAddressBookApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_address_book_recipient**](CryptoAddressBookApi.md#create_address_book_recipient) | **POST** /v1/addressBook/recipients | Create a recipient
[**delete_address_book_recipient**](CryptoAddressBookApi.md#delete_address_book_recipient) | **DELETE** /v1/addressBook/recipients/{id} | Delete a recipient
[**get_address_book_recipient**](CryptoAddressBookApi.md#get_address_book_recipient) | **GET** /v1/addressBook/recipients/{id} | Get a recipient
[**list_address_book_recipients**](CryptoAddressBookApi.md#list_address_book_recipients) | **GET** /v1/addressBook/recipients | List all recipients
[**modify_address_book_recipient**](CryptoAddressBookApi.md#modify_address_book_recipient) | **PATCH** /v1/addressBook/recipients/{id} | Modify a recipient



## create_address_book_recipient

> models::CreateAddressBookRecipientResponse create_address_book_recipient(address_book_recipient_request)
Create a recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_book_recipient_request** | Option<[**AddressBookRecipientRequest**](AddressBookRecipientRequest.md)> |  |  |

### Return type

[**models::CreateAddressBookRecipientResponse**](CreateAddressBookRecipientResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_address_book_recipient

> delete_address_book_recipient(id)
Delete a recipient

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


## get_address_book_recipient

> models::GetAddressBookRecipientResponse get_address_book_recipient(id)
Get a recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetAddressBookRecipientResponse**](GetAddressBookRecipientResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_address_book_recipients

> models::ListAddressBookRecipientsResponse list_address_book_recipients(address, chain, email, status, from, to, page_before, page_after, page_size)
List all recipients

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | Option<**String**> | Filters results to fetch only address book recipients with the provided address. |  |
**chain** | Option<**String**> | Filters results to fetch only address book recipients with the provided chain. |  |
**email** | Option<**String**> | Filters results to fetch only address book recipients that have the provided email in their metadata. |  |
**status** | Option<**String**> | Filters results to fetch only address book recipients that have the provided status. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListAddressBookRecipientsResponse**](ListAddressBookRecipientsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_address_book_recipient

> models::ModifyAddressBookRecipientResponse modify_address_book_recipient(id, address_book_recipient_modify_request)
Modify a recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**address_book_recipient_modify_request** | Option<[**AddressBookRecipientModifyRequest**](AddressBookRecipientModifyRequest.md)> |  |  |

### Return type

[**models::ModifyAddressBookRecipientResponse**](ModifyAddressBookRecipientResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

