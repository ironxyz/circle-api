# \CryptoPaymentIntentsApi

All URIs are relative to *https://api-sandbox.circle.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_payment_intent**](CryptoPaymentIntentsApi.md#create_payment_intent) | **POST** /v1/paymentIntents | Create a payment intent
[**expire_payment_intent**](CryptoPaymentIntentsApi.md#expire_payment_intent) | **POST** /v1/paymentIntents/{id}/expire | Expire a payment intent
[**get_payment_intent**](CryptoPaymentIntentsApi.md#get_payment_intent) | **GET** /v1/paymentIntents/{id} | Get a payment intent
[**list_payment_intents**](CryptoPaymentIntentsApi.md#list_payment_intents) | **GET** /v1/paymentIntents | List all payment intents
[**refund_payment_intent**](CryptoPaymentIntentsApi.md#refund_payment_intent) | **POST** /v1/paymentIntents/{id}/refund | Refund a payment intent



## create_payment_intent

> models::CreatePaymentIntentResponse create_payment_intent(create_payment_intent_request)
Create a payment intent

Create a transient or continuous payment intent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_payment_intent_request** | Option<[**CreatePaymentIntentRequest**](CreatePaymentIntentRequest.md)> |  |  |

### Return type

[**models::CreatePaymentIntentResponse**](CreatePaymentIntentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expire_payment_intent

> models::ExpirePaymentIntentResponse expire_payment_intent(id, body)
Expire a payment intent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::ExpirePaymentIntentResponse**](ExpirePaymentIntentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_payment_intent

> models::GetPaymentIntentResponse get_payment_intent(id)
Get a payment intent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |

### Return type

[**models::GetPaymentIntentResponse**](GetPaymentIntentResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_payment_intents

> models::ListPaymentIntentsResponse list_payment_intents(status, context, from, to, page_before, page_after, page_size)
List all payment intents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Filters by the most recent `timeline.status` within the payment intent. |  |
**context** | Option<**String**> | Filters by the most recent `timeline.context` within the payment intent. |  |
**from** | Option<**String**> | Queries items created since the specified date-time (inclusive). |  |
**to** | Option<**String**> | Queries items created before the specified date-time (inclusive). |  |
**page_before** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive end of a page. When provided, the collection resource will return the next `n` items before the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageAfter.  |  |
**page_after** | Option<**String**> | A collection ID value used for pagination.  It marks the exclusive begin of a page. When provided, the collection resource will return the next `n` items after the id, with `n` being specified by `pageSize`.  The items will be returned in the natural order of the collection.  The resource will return the first page if neither `pageAfter` nor `pageBefore` are specified.  SHOULD NOT be used in conjuction with pageBefore.  |  |
**page_size** | Option<**i32**> | Limits the number of items to be returned.  Some collections have a strict upper bound that will disregard this value. In case the specified value is higher than the allowed limit, the collection limit will be used.  If avoided, the collection will determine the page size itself.  |  |

### Return type

[**models::ListPaymentIntentsResponse**](ListPaymentIntentsResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refund_payment_intent

> models::CreateCryptoRefundResponse refund_payment_intent(id, crypto_refund_creation_request)
Refund a payment intent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Universally unique identifier (UUID v4) of a resource. | [required] |
**crypto_refund_creation_request** | Option<[**CryptoRefundCreationRequest**](CryptoRefundCreationRequest.md)> |  |  |

### Return type

[**models::CreateCryptoRefundResponse**](CreateCryptoRefundResponse.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

