# \CardsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_card**](CardsApi.md#create_card) | **POST** /v1/cards | Create a card
[**get_card**](CardsApi.md#get_card) | **GET** /v1/cards/{id} | Get a card
[**list_cards**](CardsApi.md#list_cards) | **GET** /v1/cards | List all cards
[**update_card**](CardsApi.md#update_card) | **PUT** /v1/cards/{id} | Update a card



## create_card

> models::CreateCardResponse create_card(card_creation_request)
Create a card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_creation_request** | Option<[**CardCreationRequest**](CardCreationRequest.md)> |  |  |

### Return type

[**models::CreateCardResponse**](CreateCardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_card

> models::GetCardResponse get_card(id)
Get a card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetCardResponse**](GetCardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cards

> models::ListCardsResponse list_cards(page_before, page_after, page_size)
List all cards

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListCardsResponse**](ListCardsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_card

> models::UpdateCardResponse update_card(id, card_update)
Update a card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**card_update** | Option<[**CardUpdate**](CardUpdate.md)> |  |  |

### Return type

[**models::UpdateCardResponse**](UpdateCardResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

